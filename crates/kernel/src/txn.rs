//! Workspace transactions over the filesystem — the Phase 0 heart of D-005.
//!
//! `begin` snapshots a target directory into a copy-on-write workspace (v0:
//! full copy — personal scale; lazy CoW is an optimization behind this
//! interface). Work happens in the workspace at full speed. `diff` computes
//! the truthful effect list by comparing workspace against the snapshot
//! manifest — hashes, not model claims (D-015). `commit` re-verifies the
//! target hasn't drifted, applies changes under capability checks (R1,
//! exercise-time), and records every effect in the ledger. `abort` deletes
//! the workspace; the target was never touched.
//!
//! v0 notes: native fs ops (WASM adapter isolation arrives with the plugin
//! framework); capability scope is the target root string (per-adapter scope
//! grammar is spec 0.2); ledger events are unsigned pending the daemon
//! identity key.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use substrate::event::{Kind, Origin, Privacy, Trust};
use substrate::Substrate;

use crate::capability::{Capability, CapabilityTable, EffectClass};

pub const TXN_BEGIN: &str = "txn.begin/1";
pub const TXN_COMMIT: &str = "txn.commit/1";
pub const TXN_ABORT: &str = "txn.abort/1";
pub const LEDGER_EFFECT: &str = "ledger.effect/1";

/// Directories never snapshotted: the record itself and VCS internals.
const SKIP_DIRS: [&str; 2] = [".nexus", ".git"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TxnState {
    Open,
    Committed,
    Aborted,
}

#[derive(Serialize, Deserialize)]
pub struct TxnMeta {
    pub id: String,
    pub target: PathBuf,
    pub created: DateTime<Utc>,
    pub state: TxnState,
    pub capability: Capability,
    /// Snapshot manifest: relative path → BLAKE3 hex at begin time.
    pub manifest: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeOp {
    Created,
    Modified,
    Deleted,
}

/// One entry of the truthful effect list.
#[derive(Debug, Serialize)]
pub struct Change {
    pub op: ChangeOp,
    pub path: String,
    pub before: Option<String>,
    pub after: Option<String>,
}

pub struct TxnManager {
    dir: PathBuf,
}

impl TxnManager {
    pub fn new(data_dir: &Path) -> Result<Self> {
        let dir = data_dir.join("txns");
        fs::create_dir_all(&dir)?;
        Ok(Self { dir })
    }

    pub fn workspace(&self, id: &str) -> PathBuf {
        self.dir.join(id).join("ws")
    }

    fn meta_path(&self, id: &str) -> PathBuf {
        self.dir.join(id).join("meta.json")
    }

    fn load(&self, id: &str) -> Result<TxnMeta> {
        let raw = fs::read_to_string(self.meta_path(id))
            .with_context(|| format!("no such transaction: {id}"))?;
        Ok(serde_json::from_str(&raw)?)
    }

    fn save(&self, meta: &TxnMeta) -> Result<()> {
        fs::write(self.meta_path(&meta.id), serde_json::to_string_pretty(meta)?)?;
        Ok(())
    }

    /// Snapshot `target` into a workspace and mint the transaction's R1
    /// capability. Logs `txn.begin/1`.
    pub fn begin(&self, sub: &mut Substrate, target: &Path) -> Result<TxnMeta> {
        if !target.is_dir() {
            bail!("target is not a directory: {}", target.display());
        }
        let target = target.canonicalize()?;
        let id = ulid::Ulid::new().to_string();
        let ws = self.workspace(&id);
        fs::create_dir_all(&ws)?;

        let manifest = hash_tree(&target)?;
        copy_tree(&target, &ws)?;

        let mut table = CapabilityTable::new();
        let cap_id = table.mint(
            &id,
            "fs",
            vec!["create".into(), "write".into(), "delete".into()],
            vec![target.to_string_lossy().into_owned()],
            EffectClass::R1,
            Utc::now() + Duration::hours(24),
        );
        let capability = table.get(&cap_id).unwrap().clone();

        let meta = TxnMeta {
            id: id.clone(),
            target: target.clone(),
            created: Utc::now(),
            state: TxnState::Open,
            capability,
            manifest,
        };
        self.save(&meta)?;

        // The authoritative target and capability live in the signed record,
        // not the plaintext meta (S4): commit re-derives them from here, so a
        // forged meta.json cannot redirect a commit or escalate authority.
        let body = crate::ledger::sign_body(
            sub,
            serde_json::json!({
                "txn": id,
                "target": target.to_string_lossy(),
                "files_snapshotted": meta.manifest.len(),
                "capability": meta.capability.id,
                "capability_full": serde_json::to_value(&meta.capability)?,
            }),
        )?;
        sub.append(
            Kind::Action,
            TXN_BEGIN,
            kernel_origin(),
            Privacy::P1,
            vec![],
            body.to_string().as_bytes(),
        )?;
        Ok(meta)
    }

    /// The effect list (workspace vs. snapshot) and target drift (target vs.
    /// snapshot). Drift non-empty means the world moved under the
    /// transaction; commit refuses until resolved.
    pub fn diff(&self, id: &str) -> Result<(TxnMeta, Vec<Change>, Vec<String>)> {
        let meta = self.load(id)?;
        if meta.state != TxnState::Open {
            bail!("transaction {id} is not open ({:?})", meta.state);
        }
        let ws_state = hash_tree(&self.workspace(id))?;
        let mut changes = Vec::new();
        for (path, after) in &ws_state {
            match meta.manifest.get(path) {
                None => changes.push(Change {
                    op: ChangeOp::Created,
                    path: path.clone(),
                    before: None,
                    after: Some(after.clone()),
                }),
                Some(before) if before != after => changes.push(Change {
                    op: ChangeOp::Modified,
                    path: path.clone(),
                    before: Some(before.clone()),
                    after: Some(after.clone()),
                }),
                _ => {}
            }
        }
        for (path, before) in &meta.manifest {
            if !ws_state.contains_key(path) {
                changes.push(Change {
                    op: ChangeOp::Deleted,
                    path: path.clone(),
                    before: Some(before.clone()),
                    after: None,
                });
            }
        }

        let target_state = hash_tree(&meta.target)?;
        let mut drift = Vec::new();
        for (path, hash) in &target_state {
            match meta.manifest.get(path) {
                None => drift.push(format!("created in target: {path}")),
                Some(h) if h != hash => drift.push(format!("modified in target: {path}")),
                _ => {}
            }
        }
        for path in meta.manifest.keys() {
            if !target_state.contains_key(path) {
                drift.push(format!("deleted in target: {path}"));
            }
        }
        Ok((meta, changes, drift))
    }

    /// Authoritative target + capability for a transaction, read from the
    /// signed `txn.begin/1` event in the record — the tamper-evident source of
    /// truth (S4). The plaintext `meta.json` is only a cache; commit trusts
    /// this, not that.
    fn authoritative(&self, sub: &Substrate, id: &str) -> Result<(PathBuf, Capability)> {
        let (events, _) = sub.events(true)?;
        for e in &events {
            if e.header.schema != TXN_BEGIN {
                continue;
            }
            let substrate::BodyState::Plain(b) = &e.body else { continue };
            let v: serde_json::Value = serde_json::from_slice(b)?;
            if v["txn"].as_str() != Some(id) {
                continue;
            }
            let target = PathBuf::from(
                v["target"].as_str().context("begin event missing target")?,
            );
            let capability: Capability = serde_json::from_value(v["capability_full"].clone())
                .context("begin event missing capability")?;
            return Ok((target, capability));
        }
        bail!("no signed begin event for transaction {id} — refusing to commit")
    }

    /// Apply the effect list to the target: drift check, then per-change
    /// capability check (R1, exercise-time) and a `ledger.effect/1` event per
    /// applied change, then `txn.commit/1`.
    pub fn commit(&self, sub: &mut Substrate, id: &str) -> Result<Vec<Change>> {
        // S4: the meta.json on disk is untrusted. Re-derive target and
        // capability from the signed record and refuse if the cache disagrees.
        let (auth_target, auth_cap) = self.authoritative(sub, id)?;
        let (mut meta, changes, drift) = self.diff(id)?;
        if meta.target != auth_target || meta.capability != auth_cap {
            bail!(
                "transaction metadata does not match the signed record — refusing to commit \
                 (meta.json was tampered)"
            );
        }
        if !drift.is_empty() {
            bail!(
                "target drifted since snapshot — refusing to commit:\n  {}",
                drift.join("\n  ")
            );
        }

        let mut table = CapabilityTable::new();
        table.restore(meta.capability.clone());
        let scope = meta.target.to_string_lossy().into_owned();
        let ws = self.workspace(id);

        for change in &changes {
            let op = match change.op {
                ChangeOp::Created => "create",
                ChangeOp::Modified => "write",
                ChangeOp::Deleted => "delete",
            };
            table
                .check(&meta.capability.id, op, &scope, EffectClass::R1, Trust::Local, Utc::now())
                .with_context(|| format!("capability check failed for {op} {}", change.path))?;

            let target_file = meta.target.join(&change.path);
            match change.op {
                ChangeOp::Created | ChangeOp::Modified => {
                    if let Some(parent) = target_file.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::copy(ws.join(&change.path), &target_file)?;
                }
                ChangeOp::Deleted => {
                    fs::remove_file(&target_file)?;
                }
            }

            let body = crate::ledger::sign_body(
                sub,
                serde_json::json!({
                    "txn": meta.id,
                    "capability": meta.capability.id,
                    "op": op,
                    "path": change.path,
                    "class": "R1",
                    "before": change.before,
                    "after": change.after,
                }),
            )?;
            sub.append(
                Kind::Action,
                LEDGER_EFFECT,
                kernel_origin(),
                Privacy::P1,
                vec![],
                body.to_string().as_bytes(),
            )?;
        }

        let body = crate::ledger::sign_body(
            sub,
            serde_json::json!({ "txn": meta.id, "effects": changes.len() }),
        )?;
        sub.append(
            Kind::Action,
            TXN_COMMIT,
            kernel_origin(),
            Privacy::P1,
            vec![],
            body.to_string().as_bytes(),
        )?;

        meta.state = TxnState::Committed;
        self.save(&meta)?;
        fs::remove_dir_all(&ws).ok(); // workspace is disposable once applied
        Ok(changes)
    }

    /// Discard the workspace. The target was never touched — abort is free.
    pub fn abort(&self, sub: &mut Substrate, id: &str) -> Result<()> {
        let mut meta = self.load(id)?;
        if meta.state != TxnState::Open {
            bail!("transaction {id} is not open ({:?})", meta.state);
        }
        fs::remove_dir_all(self.workspace(id)).ok();
        meta.state = TxnState::Aborted;
        self.save(&meta)?;
        let body = crate::ledger::sign_body(sub, serde_json::json!({ "txn": meta.id }))?;
        sub.append(
            Kind::Action,
            TXN_ABORT,
            kernel_origin(),
            Privacy::P1,
            vec![],
            body.to_string().as_bytes(),
        )?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<TxnMeta>> {
        let mut out = Vec::new();
        for entry in fs::read_dir(&self.dir)? {
            let path = entry?.path().join("meta.json");
            if path.exists() {
                out.push(serde_json::from_str(&fs::read_to_string(path)?)?);
            }
        }
        out.sort_by(|a: &TxnMeta, b: &TxnMeta| a.created.cmp(&b.created));
        Ok(out)
    }
}

fn kernel_origin() -> Origin {
    Origin {
        adapter: "kernel.txn".into(),
        actor: "kernel".into(),
        trust: Trust::Local,
    }
}

/// Relative path → BLAKE3 hex for every file under `root` (skipping
/// SKIP_DIRS). Paths use forward slashes for cross-platform manifests.
fn hash_tree(root: &Path) -> Result<BTreeMap<String, String>> {
    let mut out = BTreeMap::new();
    walk(root, root, &mut |rel, full| {
        let hash = blake3::hash(&fs::read(full)?);
        out.insert(rel.to_string(), hash.to_hex().to_string());
        Ok(())
    })?;
    Ok(out)
}

fn copy_tree(from: &Path, to: &Path) -> Result<()> {
    walk(from, from, &mut |rel, full| {
        let dest = to.join(rel);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(full, dest)?;
        Ok(())
    })
}

fn walk(
    root: &Path,
    dir: &Path,
    f: &mut impl FnMut(&str, &Path) -> Result<()>,
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if path.is_dir() {
            if SKIP_DIRS.contains(&name.as_str()) {
                continue;
            }
            walk(root, &path, f)?;
        } else {
            let rel = path
                .strip_prefix(root)?
                .to_string_lossy()
                .replace('\\', "/");
            f(&rel, &path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use substrate::BodyState;

    fn setup() -> (tempfile::TempDir, Substrate, TxnManager, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let data = dir.path().join(".nexus");
        let sub = Substrate::init(&data, "pass").unwrap();
        let mgr = TxnManager::new(&data).unwrap();
        let target = dir.path().join("project");
        fs::create_dir_all(target.join("src")).unwrap();
        fs::write(target.join("a.txt"), "alpha").unwrap();
        fs::write(target.join("src/b.txt"), "beta").unwrap();
        (dir, sub, mgr, target)
    }

    #[test]
    fn begin_diff_commit_roundtrip() {
        let (_dir, mut sub, mgr, target) = setup();
        let meta = mgr.begin(&mut sub, &target).unwrap();
        let ws = mgr.workspace(&meta.id);

        fs::write(ws.join("a.txt"), "alpha v2").unwrap(); // modify
        fs::write(ws.join("c.txt"), "gamma").unwrap(); // create
        fs::remove_file(ws.join("src/b.txt")).unwrap(); // delete

        let (_, changes, drift) = mgr.diff(&meta.id).unwrap();
        assert!(drift.is_empty());
        assert_eq!(changes.len(), 3);

        let applied = mgr.commit(&mut sub, &meta.id).unwrap();
        assert_eq!(applied.len(), 3);
        assert_eq!(fs::read_to_string(target.join("a.txt")).unwrap(), "alpha v2");
        assert_eq!(fs::read_to_string(target.join("c.txt")).unwrap(), "gamma");
        assert!(!target.join("src/b.txt").exists());

        // Ledger: begin + 3 effects + commit = 5 kernel events, all decryptable.
        let (events, _) = sub.events(true).unwrap();
        let schemas: Vec<&str> = events.iter().map(|e| e.header.schema.as_str()).collect();
        assert_eq!(schemas.iter().filter(|s| **s == LEDGER_EFFECT).count(), 3);
        assert_eq!(schemas.iter().filter(|s| **s == TXN_BEGIN).count(), 1);
        assert_eq!(schemas.iter().filter(|s| **s == TXN_COMMIT).count(), 1);
        assert!(events.iter().all(|e| matches!(e.body, BodyState::Plain(_))));

        // Committed txn cannot be committed again.
        assert!(mgr.commit(&mut sub, &meta.id).is_err());
    }

    #[test]
    fn drift_refuses_commit() {
        let (_dir, mut sub, mgr, target) = setup();
        let meta = mgr.begin(&mut sub, &target).unwrap();
        fs::write(mgr.workspace(&meta.id).join("a.txt"), "ws edit").unwrap();
        // World moves under the transaction:
        fs::write(target.join("a.txt"), "concurrent edit").unwrap();
        let err = mgr.commit(&mut sub, &meta.id).unwrap_err();
        assert!(err.to_string().contains("drifted"));
        // Target keeps the concurrent edit; nothing was applied.
        assert_eq!(fs::read_to_string(target.join("a.txt")).unwrap(), "concurrent edit");
    }

    #[test]
    fn abort_leaves_target_untouched() {
        let (_dir, mut sub, mgr, target) = setup();
        let meta = mgr.begin(&mut sub, &target).unwrap();
        fs::write(mgr.workspace(&meta.id).join("a.txt"), "destroyed on abort").unwrap();
        mgr.abort(&mut sub, &meta.id).unwrap();
        assert_eq!(fs::read_to_string(target.join("a.txt")).unwrap(), "alpha");
        assert!(!mgr.workspace(&meta.id).exists());
        assert!(mgr.diff(&meta.id).is_err(), "aborted txn is closed");
    }

    // Regression for SECURITY_AND_FAILURE_REVIEW S4: the plaintext meta.json
    // is not authoritative. Any edit to its capability or target is caught by
    // cross-check against the signed begin event. (Capability expiry itself is
    // enforced at the capability layer — see capability::tests.)
    #[test]
    fn tampered_meta_capability_is_rejected() {
        let (_dir, mut sub, mgr, target) = setup();
        let meta = mgr.begin(&mut sub, &target).unwrap();
        fs::write(mgr.workspace(&meta.id).join("a.txt"), "late").unwrap();
        // Forge the on-disk capability (escalate effect, extend expiry).
        let mut m = mgr.load(&meta.id).unwrap();
        m.capability.max_effect = EffectClass::R3;
        m.capability.expiry = Utc::now() + Duration::days(3650);
        mgr.save(&m).unwrap();
        let err = mgr.commit(&mut sub, &meta.id).unwrap_err();
        assert!(format!("{err:#}").contains("signed record"), "{err:#}");
        assert_eq!(fs::read_to_string(target.join("a.txt")).unwrap(), "alpha");
    }

    // Regression for S4: forging meta.target to redirect the write is caught.
    #[test]
    fn tampered_meta_target_cannot_redirect_commit() {
        let (dir, mut sub, mgr, target) = setup();
        let victim = dir.path().join("victim");
        fs::create_dir_all(&victim).unwrap();
        let meta = mgr.begin(&mut sub, &target).unwrap();
        fs::write(mgr.workspace(&meta.id).join("payload.txt"), "pwned").unwrap();
        let mut m = mgr.load(&meta.id).unwrap();
        m.target = victim.canonicalize().unwrap();
        m.capability.scope = vec![victim.to_string_lossy().into_owned()];
        mgr.save(&m).unwrap();
        assert!(mgr.commit(&mut sub, &meta.id).is_err());
        assert!(!victim.join("payload.txt").exists(), "write must not reach victim");
    }
}
