use std::collections::HashMap;

use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use substrate::event::Trust;

/// Effect classes per specs/capability.md §1. Closed set; derived `Ord` gives
/// R0 < R1 < R2 < R3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EffectClass {
    R0, // Observe
    R1, // Reversible (kernel-restorable)
    R2, // Compensable (registered compensator)
    R3, // Irreversible (queue-only; fires at commit)
}

/// Capability ceiling from the minimum trust present in a task's assembled
/// context (specs/capability.md §5). Monotonic downward over a task's life.
pub fn ceiling_for(min_trust: Trust) -> EffectClass {
    match min_trust {
        Trust::User | Trust::Local => EffectClass::R3, // R3 = queueing permitted, never direct execution
        Trust::Derived => EffectClass::R2,
        Trust::External => EffectClass::R1,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: String,
    pub task: String,
    pub adapter: String,
    pub ops: Vec<String>,
    /// v0 scope: exact resource strings. Per-adapter scope schemas are a spec
    /// 0.2 question; the subset rule below is what's normative.
    pub scope: Vec<String>,
    pub max_effect: EffectClass,
    pub expiry: DateTime<Utc>,
    pub parent: Option<String>,
    pub revoked: bool,
}

#[derive(Default)]
pub struct CapabilityTable {
    caps: HashMap<String, Capability>,
}

impl CapabilityTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mint a root capability for a task. Default-deny lives in the caller:
    /// nothing is minted except from an approved plan step (§3).
    pub fn mint(
        &mut self,
        task: &str,
        adapter: &str,
        ops: Vec<String>,
        scope: Vec<String>,
        max_effect: EffectClass,
        expiry: DateTime<Utc>,
    ) -> String {
        let id = ulid::Ulid::new().to_string();
        self.caps.insert(
            id.clone(),
            Capability {
                id: id.clone(),
                task: task.into(),
                adapter: adapter.into(),
                ops,
                scope,
                max_effect,
                expiry,
                parent: None,
                revoked: false,
            },
        );
        id
    }

    /// Derive a child ⊆ parent on every axis (§4). Broadening any axis fails.
    pub fn attenuate(
        &mut self,
        parent_id: &str,
        ops: Vec<String>,
        scope: Vec<String>,
        max_effect: EffectClass,
        expiry: DateTime<Utc>,
    ) -> Result<String> {
        let parent = match self.caps.get(parent_id) {
            Some(p) => p.clone(),
            None => bail!("no such capability: {parent_id}"),
        };
        if parent.revoked {
            bail!("parent capability is revoked");
        }
        if !ops.iter().all(|o| parent.ops.contains(o)) {
            bail!("attenuation cannot broaden ops");
        }
        if !scope.iter().all(|s| parent.scope.contains(s)) {
            bail!("attenuation cannot broaden scope");
        }
        if max_effect > parent.max_effect {
            bail!("attenuation cannot raise max_effect");
        }
        if expiry > parent.expiry {
            bail!("attenuation cannot extend expiry");
        }
        let id = ulid::Ulid::new().to_string();
        self.caps.insert(
            id.clone(),
            Capability {
                id: id.clone(),
                task: parent.task.clone(),
                adapter: parent.adapter.clone(),
                ops,
                scope,
                max_effect,
                expiry,
                parent: Some(parent_id.to_string()),
                revoked: false,
            },
        );
        Ok(id)
    }

    /// Revoke a capability and its entire descendant chain (§7).
    pub fn revoke(&mut self, id: &str) {
        let mut frontier = vec![id.to_string()];
        while let Some(current) = frontier.pop() {
            if let Some(c) = self.caps.get_mut(&current) {
                c.revoked = true;
            }
            frontier.extend(
                self.caps
                    .values()
                    .filter(|c| c.parent.as_deref() == Some(current.as_str()) && !c.revoked)
                    .map(|c| c.id.clone()),
            );
        }
    }

    /// Revoke everything for a task (task completion / `nx stop --all`).
    pub fn revoke_task(&mut self, task: &str) {
        let ids: Vec<String> = self
            .caps
            .values()
            .filter(|c| c.task == task)
            .map(|c| c.id.clone())
            .collect();
        for id in ids {
            self.revoke(&id);
        }
    }

    /// Exercise-time check (§7: checked at every exercise, not at mint).
    /// `context_min_trust` applies the ceiling (§5): effective limit is
    /// min(capability.max_effect, ceiling), and R3 under any ceiling means
    /// queue-only — direct R3 execution never passes this check.
    pub fn check(
        &self,
        id: &str,
        op: &str,
        resource: &str,
        class: EffectClass,
        context_min_trust: Trust,
        now: DateTime<Utc>,
    ) -> Result<()> {
        let cap = match self.caps.get(id) {
            Some(c) => c,
            None => bail!("no such capability"),
        };
        if cap.revoked {
            bail!("capability revoked");
        }
        if now > cap.expiry {
            bail!("capability expired");
        }
        if !cap.ops.iter().any(|o| o == op) {
            bail!("op '{op}' not granted");
        }
        if !cap.scope.iter().any(|s| s == resource) {
            bail!("resource '{resource}' out of scope");
        }
        if class == EffectClass::R3 {
            bail!("R3 is queue-only: no direct execution path exists (D-005)");
        }
        let ceiling = ceiling_for(context_min_trust);
        let limit = cap.max_effect.min(ceiling);
        if class > limit {
            bail!(
                "effect class {class:?} exceeds limit {limit:?} (capability max {:?}, context ceiling {ceiling:?})",
                cap.max_effect
            );
        }
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Capability> {
        self.caps.get(id)
    }

    /// Reload a persisted capability (e.g. a transaction's grant from its
    /// meta) so exercise-time checks run against it. Restoring never creates
    /// authority: the capability was minted and persisted by the kernel.
    pub fn restore(&mut self, cap: Capability) {
        self.caps.insert(cap.id.clone(), cap);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    // One shared expiry timestamp: evaluating `Utc::now() + 1h` per call
    // would put each child's expiry microseconds past its parent's, tripping
    // the no-extend-expiry rule in tests that aren't exercising it.
    fn table_with_root() -> (CapabilityTable, String, DateTime<Utc>) {
        let exp = Utc::now() + Duration::hours(1);
        let mut t = CapabilityTable::new();
        let id = t.mint(
            "task-1",
            "fs",
            vec!["read".into(), "write".into()],
            vec!["/ws/a".into(), "/ws/b".into()],
            EffectClass::R2,
            exp,
        );
        (t, id, exp)
    }

    #[test]
    fn check_allows_within_grant_and_denies_outside() {
        let (t, id, _) = table_with_root();
        assert!(t.check(&id, "read", "/ws/a", EffectClass::R0, Trust::User, Utc::now()).is_ok());
        assert!(t.check(&id, "delete", "/ws/a", EffectClass::R0, Trust::User, Utc::now()).is_err());
        assert!(t.check(&id, "read", "/etc/shadow", EffectClass::R0, Trust::User, Utc::now()).is_err());
    }

    #[test]
    fn attenuation_cannot_broaden() {
        let (mut t, id, exp) = table_with_root();
        assert!(t
            .attenuate(&id, vec!["read".into()], vec!["/ws/a".into()], EffectClass::R1, exp)
            .is_ok());
        assert!(t
            .attenuate(&id, vec!["execute".into()], vec!["/ws/a".into()], EffectClass::R1, exp)
            .is_err());
        assert!(t
            .attenuate(&id, vec!["read".into()], vec!["/other".into()], EffectClass::R1, exp)
            .is_err());
        assert!(t
            .attenuate(&id, vec!["read".into()], vec!["/ws/a".into()], EffectClass::R3, exp)
            .is_err());
        assert!(t
            .attenuate(&id, vec!["read".into()], vec!["/ws/a".into()], EffectClass::R1, exp + Duration::hours(2))
            .is_err());
    }

    #[test]
    fn revoke_cascades_to_descendants() {
        let (mut t, root, exp) = table_with_root();
        let child = t
            .attenuate(&root, vec!["read".into()], vec!["/ws/a".into()], EffectClass::R1, exp)
            .unwrap();
        let grandchild = t
            .attenuate(&child, vec!["read".into()], vec!["/ws/a".into()], EffectClass::R0, exp)
            .unwrap();
        t.revoke(&root);
        for id in [&root, &child, &grandchild] {
            assert!(t.check(id, "read", "/ws/a", EffectClass::R0, Trust::User, Utc::now()).is_err());
        }
    }

    #[test]
    fn external_context_caps_at_r1() {
        let (t, id, _) = table_with_root();
        // R2 fine with trusted context…
        assert!(t.check(&id, "write", "/ws/a", EffectClass::R2, Trust::User, Utc::now()).is_ok());
        // …capped to R1 once external content is in context (D-014).
        assert!(t.check(&id, "write", "/ws/a", EffectClass::R2, Trust::External, Utc::now()).is_err());
        assert!(t.check(&id, "write", "/ws/a", EffectClass::R1, Trust::External, Utc::now()).is_ok());
    }

    #[test]
    fn r3_never_executes_directly() {
        let mut t = CapabilityTable::new();
        let id = t.mint(
            "task-1",
            "mail",
            vec!["send".into()],
            vec!["thread:anna".into()],
            EffectClass::R3,
            Utc::now() + Duration::hours(1),
        );
        assert!(t.check(&id, "send", "thread:anna", EffectClass::R3, Trust::User, Utc::now()).is_err());
    }

    #[test]
    fn expiry_enforced_at_exercise() {
        let mut t = CapabilityTable::new();
        let id = t.mint(
            "task-1",
            "fs",
            vec!["read".into()],
            vec!["/ws/a".into()],
            EffectClass::R0,
            Utc::now() - Duration::seconds(1),
        );
        assert!(t.check(&id, "read", "/ws/a", EffectClass::R0, Trust::User, Utc::now()).is_err());
    }
}
