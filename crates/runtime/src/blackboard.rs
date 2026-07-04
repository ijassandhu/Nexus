//! Blackboard v0 per specs/blackboard.md: typed artifacts as `bb.artifact/1`
//! events, state transitions as `bb.transition/1` events, contracts enforced
//! at write time, transition authority role-scoped. Artifacts are immutable;
//! the current view is folded from the log (personal scale — materialization
//! into the derived store comes later behind this interface).

use std::collections::HashMap;

use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use substrate::event::{Kind, Origin, Privacy, Trust};
use substrate::{BodyState, Substrate};

pub const ARTIFACT_SCHEMA: &str = "bb.artifact/1";
pub const TRANSITION_SCHEMA: &str = "bb.transition/1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub role: String,
    pub charter_version: String,
    pub model_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    #[serde(rename = "type")]
    pub atype: String,
    pub version: u32,
    pub task: String,
    pub author: Author,
    pub created: DateTime<Utc>,
    /// State at creation; current state is folded from transitions.
    pub state: String,
    pub refs: Vec<String>,
    pub body: serde_json::Value,
}

impl Artifact {
    pub fn new(
        atype: &str,
        task: &str,
        author: Author,
        state: &str,
        refs: Vec<String>,
        body: serde_json::Value,
    ) -> Self {
        Self {
            id: ulid::Ulid::new().to_string(),
            atype: atype.into(),
            version: 1,
            task: task.into(),
            author,
            created: Utc::now(),
            state: state.into(),
            refs,
            body,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Transition {
    artifact: String,
    from: String,
    to: String,
    by: String,
    reason: String,
}

/// Contracts, enforced at write time — a violating artifact is rejected,
/// not merely frowned at (specs/blackboard.md §2).
fn contract_check(a: &Artifact) -> Result<()> {
    match a.atype.as_str() {
        "Critique" => {
            let verdict = a.body.get("verdict").and_then(|v| v.as_str()).unwrap_or("");
            if !["approve", "revise", "veto"].contains(&verdict) {
                bail!("Critique verdict must be approve|revise|veto");
            }
            let issues = a.body.get("issues").and_then(|i| i.as_array());
            if verdict != "approve" {
                let issues = issues.ok_or_else(|| anyhow::anyhow!("Critique needs issues[]"))?;
                if issues.is_empty() {
                    bail!("a {verdict} Critique with zero issues is rejected");
                }
                for i in issues {
                    if i.get("target").and_then(|t| t.as_str()).unwrap_or("").is_empty() {
                        bail!("every Critique issue must cite a target");
                    }
                }
            }
        }
        "Finding" => {
            let ev = a.body.get("evidence").and_then(|e| e.as_array());
            if ev.map(|e| e.is_empty()).unwrap_or(true) {
                bail!("a Finding without evidence is rejected (Principle 4)");
            }
        }
        "Diff" => {
            if a.body.get("effect_list").and_then(|e| e.as_array()).is_none() {
                bail!("a Diff must carry effect_list (the approval object, D-015)");
            }
        }
        _ => {}
    }
    Ok(())
}

/// Role-scoped transition authority (specs/blackboard.md §3). v0 table for
/// the shipped types; ships with the charter set later.
fn allowed_roles(atype: &str, from: &str, to: &str) -> &'static [&'static str] {
    match (atype, from, to) {
        ("Intent", "open", "planned") => &["steward"],
        ("Intent", "planned", "review") => &["steward"],
        ("Intent", "review", "done") => &["user"],
        ("Intent", "review", "cancelled") => &["user"],
        ("Diff", "proposed", "approved") => &["user"],
        ("Diff", "proposed", "rejected") => &["user"],
        _ => &[],
    }
}

#[derive(Debug, Clone)]
pub struct ArtifactView {
    pub artifact: Artifact,
    pub state: String,
}

pub struct Blackboard;

impl Blackboard {
    /// Append an artifact (contract-checked). `trust` should be `User` for
    /// artifacts wrapping direct user input, `Derived` for agent output.
    pub fn append(sub: &mut Substrate, a: &Artifact, trust: Trust) -> Result<()> {
        contract_check(a)?;
        sub.append(
            Kind::Action,
            ARTIFACT_SCHEMA,
            Origin {
                adapter: "runtime.blackboard".into(),
                actor: a.author.role.clone(),
                trust,
            },
            Privacy::P1,
            a.refs.clone(),
            serde_json::to_string(a)?.as_bytes(),
        )?;
        Ok(())
    }

    /// Record a state transition; authority is role-scoped.
    pub fn transition(
        sub: &mut Substrate,
        artifact_id: &str,
        atype: &str,
        from: &str,
        to: &str,
        by: &str,
        reason: &str,
    ) -> Result<()> {
        if !allowed_roles(atype, from, to).contains(&by) {
            bail!("role '{by}' lacks authority for {atype} {from}→{to}");
        }
        sub.append(
            Kind::Action,
            TRANSITION_SCHEMA,
            Origin {
                adapter: "runtime.blackboard".into(),
                actor: by.into(),
                trust: if by == "user" { Trust::User } else { Trust::Derived },
            },
            Privacy::P1,
            vec![artifact_id.to_string()],
            serde_json::to_string(&Transition {
                artifact: artifact_id.into(),
                from: from.into(),
                to: to.into(),
                by: by.into(),
                reason: reason.into(),
            })?
            .as_bytes(),
        )?;
        Ok(())
    }

    /// Fold the log into the current artifact view (creation state +
    /// transitions in log order).
    pub fn load(sub: &Substrate) -> Result<Vec<ArtifactView>> {
        let (events, _) = sub.events(true)?;
        let mut order: Vec<String> = Vec::new();
        let mut views: HashMap<String, ArtifactView> = HashMap::new();
        for e in &events {
            let BodyState::Plain(body) = &e.body else { continue };
            match e.header.schema.as_str() {
                ARTIFACT_SCHEMA => {
                    if let Ok(a) = serde_json::from_slice::<Artifact>(body) {
                        order.push(a.id.clone());
                        views.insert(
                            a.id.clone(),
                            ArtifactView { state: a.state.clone(), artifact: a },
                        );
                    }
                }
                TRANSITION_SCHEMA => {
                    if let Ok(t) = serde_json::from_slice::<Transition>(body) {
                        if let Some(v) = views.get_mut(&t.artifact) {
                            if v.state == t.from {
                                v.state = t.to;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(order.into_iter().filter_map(|id| views.remove(&id)).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn author(role: &str) -> Author {
        Author { role: role.into(), charter_version: "1".into(), model_id: None }
    }

    #[test]
    fn artifact_roundtrip_and_transition_fold() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        let intent = Artifact::new(
            "Intent", "T1", author("steward"), "open", vec![],
            serde_json::json!({"text": "do the thing"}),
        );
        Blackboard::append(&mut sub, &intent, Trust::User).unwrap();
        Blackboard::transition(&mut sub, &intent.id, "Intent", "open", "planned", "steward", "plan made").unwrap();
        let views = Blackboard::load(&sub).unwrap();
        assert_eq!(views.len(), 1);
        assert_eq!(views[0].state, "planned");
        assert_eq!(views[0].artifact.body["text"], "do the thing");
    }

    #[test]
    fn critique_contract_enforced() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        // veto with zero issues → rejected at write time.
        let bad = Artifact::new(
            "Critique", "T1", author("critic"), "final", vec![],
            serde_json::json!({"verdict": "veto", "issues": []}),
        );
        assert!(Blackboard::append(&mut sub, &bad, Trust::Derived).is_err());
        // issue without a target → rejected.
        let bad2 = Artifact::new(
            "Critique", "T1", author("critic"), "final", vec![],
            serde_json::json!({"verdict": "revise", "issues": [{"argument": "vague unease"}]}),
        );
        assert!(Blackboard::append(&mut sub, &bad2, Trust::Derived).is_err());
        // proper veto → accepted.
        let good = Artifact::new(
            "Critique", "T1", author("critic"), "final", vec![],
            serde_json::json!({"verdict": "veto", "issues": [{"target": "a.py", "severity": "high", "argument": "deletes prod config"}]}),
        );
        assert!(Blackboard::append(&mut sub, &good, Trust::Derived).is_ok());
    }

    #[test]
    fn transition_authority_is_role_scoped() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        let diff = Artifact::new(
            "Diff", "T1", author("worker"), "proposed", vec![],
            serde_json::json!({"txn": "X", "effect_list": []}),
        );
        Blackboard::append(&mut sub, &diff, Trust::Derived).unwrap();
        // The critic cannot approve a Diff — only the user can (D-015).
        assert!(Blackboard::transition(&mut sub, &diff.id, "Diff", "proposed", "approved", "critic", "").is_err());
        assert!(Blackboard::transition(&mut sub, &diff.id, "Diff", "proposed", "approved", "user", "lgtm").is_ok());
    }
}
