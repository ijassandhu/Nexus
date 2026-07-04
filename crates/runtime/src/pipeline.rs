//! The first agent loop (ROADMAP Phase 1): Steward wraps the intent →
//! Worker plans and executes inside a workspace transaction → Critic reviews
//! the truthful effect list → everything lands on the blackboard for the
//! Delegation Inbox. Nothing touches the target until the user (or, later,
//! trust-ledger authority) approves the effect list.

use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::{bail, Context, Result};
use kernel::txn::TxnManager;
use serde::Deserialize;
use substrate::event::{Kind, Origin, Privacy, Trust};
use substrate::Substrate;

use crate::blackboard::{Artifact, Author, Blackboard};
use crate::charter;
use crate::router::Router;

pub const WORKINGSET_SCHEMA: &str = "memory.workingset/1";
pub const DERIVATION_SCHEMA: &str = "memory.derivation/1";

/// Record the atom of machine reasoning (specs/record.md §11.3–11.4): the
/// full working set as an encrypted event, and a derivation event carrying
/// its manifest hash — every judgment becomes attributable and replayable.
pub(crate) fn record_derivation(
    sub: &mut Substrate,
    operator: &str,
    inputs: &[String],
    output: &str,
    reasoner: &str,
    charter: &str,
    working_set: &str,
) -> Result<()> {
    let origin = Origin {
        adapter: "runtime.pipeline".into(),
        actor: operator.into(),
        trust: Trust::Local,
    };
    let ws_event = sub.append(
        Kind::System,
        WORKINGSET_SCHEMA,
        origin.clone(),
        Privacy::P1,
        vec![],
        working_set.as_bytes(),
    )?;
    sub.append(
        Kind::System,
        DERIVATION_SCHEMA,
        origin,
        Privacy::P1,
        vec![ws_event.id],
        serde_json::json!({
            "operator": operator,
            "inputs": inputs,
            "output": output,
            "reasoner": reasoner,
            "charter": charter,
            "manifest": substrate::hex(blake3::hash(working_set.as_bytes()).as_bytes()),
        })
        .to_string()
        .as_bytes(),
    )?;
    Ok(())
}

/// Snapshot caps: enough for MVP dev tasks, small enough to keep prompts sane.
const MAX_FILES: usize = 50;
const MAX_FILE_BYTES: u64 = 64 * 1024;

/// W v0.2 (COGNITIVE_ARCHITECTURE Part VI, first draft): pull in-scope live
/// bets into the working set — premortems as cautions, beliefs/predictions as
/// context — each carrying its earned score and justification status. Bet ids
/// are returned so the derivation event can list them (credit assignment).
fn relevant_bets(
    sub: &Substrate,
    target: &Path,
) -> Result<(Vec<serde_json::Value>, Vec<serde_json::Value>, Vec<String>)> {
    let target_n = normalize_path(&target.to_string_lossy());
    let mut cautions = Vec::new();
    let mut beliefs = Vec::new();
    let mut ids = Vec::new();
    for v in substrate::bets::views(sub)? {
        if v.status != "live" {
            continue;
        }
        let scope = normalize_path(&v.bet.scope);
        // v0 scope matching: "general" applies everywhere; otherwise path
        // containment either way (a bet scoped to a parent dir applies to
        // its children and vice versa). Spec 0.3: scope predicates.
        let in_scope =
            scope == "general" || target_n.contains(&scope) || scope.contains(&target_n);
        if !in_scope {
            continue;
        }
        let entry = serde_json::json!({
            "id": v.id,
            "statement": v.bet.statement,
            "score": { "held": v.held, "falsified": v.falsified_count },
            "unjustified": v.unjustified,
        });
        ids.push(v.id.clone());
        if v.bet.kind == substrate::bets::BetKind::Premortem {
            cautions.push(entry);
        } else {
            beliefs.push(entry);
        }
    }
    Ok((cautions, beliefs, ids))
}

fn normalize_path(s: &str) -> String {
    s.to_lowercase().replace('\\', "/").trim_start_matches("//?/").to_string()
}

#[derive(Debug, Deserialize)]
struct WorkerOutput {
    #[serde(default)]
    plan: Vec<String>,
    #[serde(default)]
    operations: Vec<WorkerOp>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "op", rename_all = "lowercase")]
enum WorkerOp {
    Write { path: String, content: String },
    Delete { path: String },
}

#[derive(Debug, Deserialize)]
struct CriticOutput {
    verdict: String,
    #[serde(default)]
    issues: Vec<serde_json::Value>,
}

pub struct RunResult {
    pub task: String,
    pub txn: Option<String>,
    pub effects: usize,
    pub verdict: String,
    pub plan: Vec<String>,
}

pub fn run_intent(
    data: &Path,
    sub: &mut Substrate,
    intent_text: &str,
    target: &Path,
    mock: bool,
) -> Result<RunResult> {
    charter::ensure_defaults(data)?;
    let worker = charter::load(data, "worker")?;
    let critic = charter::load(data, "critic")?;
    let steward = charter::load(data, "steward")?;
    let router = Router::load(data, mock)?;

    // Steward: preserve the ask verbatim (specs/blackboard.md §2).
    let task = ulid::Ulid::new().to_string();
    let intent = Artifact::new(
        "Intent",
        &task,
        Author { role: "steward".into(), charter_version: steward.version.clone(), model_id: None },
        "open",
        vec![],
        serde_json::json!({ "text": intent_text }),
    );
    Blackboard::append(sub, &intent, Trust::User)?;

    // Worker working set (W v0.3): bounded snapshot + in-scope bets +
    // budget-ranked episode recalls (retrieval v1 as candidate generator).
    let files = snapshot(target)?;
    let (cautions, beliefs, bet_ids) = relevant_bets(sub, target)?;
    let recalls = crate::retrieval::recall_episodes(sub, intent_text, crate::retrieval::DEFAULT_BUDGET)?;
    let recall_ids: Vec<String> = recalls.iter().map(|r| r.id.clone()).collect();
    let recalls_json: Vec<serde_json::Value> = recalls
        .iter()
        .map(|r| serde_json::json!({ "id": r.id, "when": r.when, "excerpt": r.excerpt }))
        .collect();
    let worker_input = serde_json::json!({
        "intent": intent_text,
        "files": files,
        "memory": { "cautions": cautions, "beliefs": beliefs, "recalls": recalls_json },
    })
    .to_string();
    let completion = router.complete(sub, &worker.task_class, &worker.prompt, &worker_input)?;
    let out: WorkerOutput = serde_json::from_str(extract_json(&completion.text))
        .with_context(|| format!("worker output was not the contracted JSON: {}", completion.text))?;

    let plan_art = Artifact::new(
        "Plan",
        &task,
        Author {
            role: "worker".into(),
            charter_version: worker.version.clone(),
            model_id: Some(completion.model_id.clone()),
        },
        "draft",
        vec![intent.id.clone()],
        serde_json::json!({ "steps": out.plan }),
    );
    Blackboard::append(sub, &plan_art, Trust::Derived)?;
    let mut worker_inputs = vec![intent.id.clone()];
    worker_inputs.extend(bet_ids.iter().cloned());
    worker_inputs.extend(recall_ids.iter().cloned());
    record_derivation(
        sub,
        "deliberate",
        &worker_inputs,
        &plan_art.id,
        &completion.model_id,
        &format!("worker@{}", worker.version),
        &worker_input,
    )?;
    Blackboard::transition(sub, &intent.id, "Intent", "open", "planned", "steward", "worker planned")?;

    if out.operations.is_empty() {
        Blackboard::transition(sub, &intent.id, "Intent", "planned", "review", "steward", "no operations proposed")?;
        return Ok(RunResult {
            task,
            txn: None,
            effects: 0,
            verdict: "no-op".into(),
            plan: out.plan,
        });
    }

    // Execute speculatively inside a transaction (D-005).
    let mgr = TxnManager::new(data)?;
    let meta = mgr.begin(sub, target)?;
    let ws = mgr.workspace(&meta.id);
    for op in &out.operations {
        match op {
            WorkerOp::Write { path, content } => {
                let dest = safe_join(&ws, path)?;
                if let Some(parent) = dest.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(dest, content)?;
            }
            WorkerOp::Delete { path } => {
                fs::remove_file(safe_join(&ws, path)?)
                    .with_context(|| format!("delete of missing file: {path}"))?;
            }
        }
    }
    let (_, changes, _) = mgr.diff(&meta.id)?;

    // The Diff's approval object is the effect list; the plan is advisory (D-015).
    let diff_art = Artifact::new(
        "Diff",
        &task,
        Author {
            role: "worker".into(),
            charter_version: worker.version.clone(),
            model_id: Some(completion.model_id.clone()),
        },
        "proposed",
        vec![plan_art.id.clone()],
        serde_json::json!({
            "txn": meta.id,
            "effect_list": changes,
            "advisory_summary": out.plan.join("; "),
        }),
    );
    Blackboard::append(sub, &diff_art, Trust::Derived)?;

    // Critic: adversarial review of effects + resulting contents.
    let mut changed_files = Vec::new();
    for c in &changes {
        if c.after.is_some() {
            if let Ok(content) = fs::read_to_string(ws.join(&c.path)) {
                changed_files.push(serde_json::json!({ "path": c.path, "content": content }));
            }
        }
    }
    let critic_input = serde_json::json!({
        "intent": intent_text,
        "effects": changes,
        "files": changed_files,
        "memory": { "cautions": cautions },
    })
    .to_string();
    let critique_done = router.complete(sub, &critic.task_class, &critic.prompt, &critic_input)?;
    let cout: CriticOutput = serde_json::from_str(extract_json(&critique_done.text))
        .with_context(|| format!("critic output was not the contracted JSON: {}", critique_done.text))?;
    let critique_art = Artifact::new(
        "Critique",
        &task,
        Author {
            role: "critic".into(),
            charter_version: critic.version.clone(),
            model_id: Some(critique_done.model_id.clone()),
        },
        "final",
        vec![diff_art.id.clone()],
        serde_json::json!({ "verdict": cout.verdict, "issues": cout.issues }),
    );
    Blackboard::append(sub, &critique_art, Trust::Derived)?; // contract enforced here
    let mut critic_inputs = vec![diff_art.id.clone()];
    critic_inputs.extend(bet_ids.iter().cloned());
    record_derivation(
        sub,
        "review",
        &critic_inputs,
        &critique_art.id,
        &critique_done.model_id,
        &format!("critic@{}", critic.version),
        &critic_input,
    )?;

    Blackboard::transition(sub, &intent.id, "Intent", "planned", "review", "steward", "awaiting user review")?;

    Ok(RunResult {
        task,
        txn: Some(meta.id),
        effects: changes.len(),
        verdict: cout.verdict,
        plan: out.plan,
    })
}

/// Bounded target snapshot for the Worker's context.
fn snapshot(target: &Path) -> Result<Vec<serde_json::Value>> {
    let mut out = Vec::new();
    collect(target, target, &mut out)?;
    Ok(out)
}

fn collect(root: &Path, dir: &Path, out: &mut Vec<serde_json::Value>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        if out.len() >= MAX_FILES {
            return Ok(());
        }
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if path.is_dir() {
            if name == ".nexus" || name == ".git" {
                continue;
            }
            collect(root, &path, out)?;
        } else if entry.metadata()?.len() <= MAX_FILE_BYTES {
            let rel = path.strip_prefix(root)?.to_string_lossy().replace('\\', "/");
            if let Ok(content) = fs::read_to_string(&path) {
                out.push(serde_json::json!({ "path": rel, "content": content }));
            }
        }
    }
    Ok(())
}

/// Confine model-supplied paths to the workspace: relative, no `..`, no roots.
fn safe_join(ws: &Path, rel: &str) -> Result<PathBuf> {
    let p = Path::new(rel);
    if p.is_absolute() {
        bail!("absolute path refused: {rel}");
    }
    for comp in p.components() {
        if !matches!(comp, Component::Normal(_)) {
            bail!("unsafe path component in: {rel}");
        }
    }
    Ok(ws.join(p))
}

/// Models sometimes wrap JSON in prose or fences; take the outermost object.
pub(crate) fn extract_json(text: &str) -> &str {
    match (text.find('{'), text.rfind('}')) {
        (Some(a), Some(b)) if b > a => &text[a..=b],
        _ => text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_join_confines_paths() {
        let ws = Path::new("/ws");
        assert!(safe_join(ws, "a/b.txt").is_ok());
        assert!(safe_join(ws, "../escape.txt").is_err());
        assert!(safe_join(ws, "a/../../escape.txt").is_err());
        assert!(safe_join(ws, "C:/Windows/system32/evil").is_err());
        assert!(safe_join(ws, "/etc/passwd").is_err());
    }

    #[test]
    fn extract_json_handles_fences() {
        assert_eq!(extract_json("```json\n{\"a\":1}\n```"), "{\"a\":1}");
        assert_eq!(extract_json("{\"a\":1}"), "{\"a\":1}");
    }
}
