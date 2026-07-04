//! Per-reasoner scorecards (COGNITIVE_ARCHITECTURE Part VIII): consultants
//! are measured. Folds `router.call/1` outcome events into per
//! (provider, model, operator) track records. v0 scores call success and
//! latency; judgment-quality dimensions arrive when bet resolutions can be
//! attributed to reasoners via derivation events.

use std::collections::BTreeMap;

use anyhow::Result;
use substrate::{BodyState, Substrate};

use crate::router::ROUTER_CALL_SCHEMA;

#[derive(Debug, Clone)]
pub struct ScoreRow {
    pub provider: String,
    pub model: String,
    pub operator: String,
    pub calls: u64,
    pub ok: u64,
    pub total_ms: u64,
    /// Judgment quality (deliberate rows only): how the human ruled on the
    /// effect lists this reasoner produced.
    pub approved: u64,
    pub rejected: u64,
}

impl ScoreRow {
    pub fn ok_rate(&self) -> f64 {
        if self.calls == 0 { 0.0 } else { self.ok as f64 / self.calls as f64 }
    }
    pub fn avg_ms(&self) -> u64 {
        if self.calls == 0 { 0 } else { self.total_ms / self.calls }
    }
}

pub fn reasoner_scorecard(sub: &Substrate) -> Result<Vec<ScoreRow>> {
    let (events, _) = sub.events(true)?;
    let mut map: BTreeMap<(String, String, String), ScoreRow> = BTreeMap::new();
    for e in events.iter().filter(|e| e.header.schema == ROUTER_CALL_SCHEMA) {
        let BodyState::Plain(b) = &e.body else { continue };
        let Ok(v) = serde_json::from_slice::<serde_json::Value>(b) else { continue };
        let provider = v["provider"].as_str().unwrap_or("?").to_string();
        let model = v["model"].as_str().unwrap_or("?").to_string();
        let operator = v["operator"].as_str().unwrap_or("unknown").to_string();
        let row = map
            .entry((provider.clone(), model.clone(), operator.clone()))
            .or_insert(ScoreRow {
                provider, model, operator,
                calls: 0, ok: 0, total_ms: 0, approved: 0, rejected: 0,
            });
        row.calls += 1;
        if v["ok"].as_bool().unwrap_or(false) {
            row.ok += 1;
        }
        row.total_ms += v["ms"].as_u64().unwrap_or(0);
    }

    // Judgment quality: attribute the human's Diff verdicts back to the
    // reasoner that produced the plan, via derivation events (the plan
    // artifact links task → Diff state on the blackboard).
    let views = crate::blackboard::Blackboard::load(sub)?;
    let mut task_of_artifact: BTreeMap<&str, &str> = BTreeMap::new();
    let mut diff_state_of_task: BTreeMap<&str, &str> = BTreeMap::new();
    for v in &views {
        task_of_artifact.insert(&v.artifact.id, &v.artifact.task);
        if v.artifact.atype == "Diff" {
            diff_state_of_task.insert(&v.artifact.task, &v.state);
        }
    }
    for e in events.iter().filter(|e| e.header.schema == crate::pipeline::DERIVATION_SCHEMA) {
        let BodyState::Plain(b) = &e.body else { continue };
        let Ok(v) = serde_json::from_slice::<serde_json::Value>(b) else { continue };
        if v["operator"] != "deliberate" {
            continue;
        }
        let (Some(output), Some(reasoner)) = (v["output"].as_str(), v["reasoner"].as_str()) else {
            continue;
        };
        let Some(task) = task_of_artifact.get(output) else { continue };
        let Some(state) = diff_state_of_task.get(*task) else { continue };
        for row in map.values_mut() {
            if row.model == reasoner && row.operator == "deliberate" {
                match *state {
                    "approved" => row.approved += 1,
                    "rejected" => row.rejected += 1,
                    _ => {}
                }
            }
        }
    }
    Ok(map.into_values().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::Router;

    #[test]
    fn scorecard_folds_router_outcomes() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        let router = Router::load(dir.path(), true).unwrap();
        router.complete(&mut sub, "worker.execute", "s", r#"{"intent":"x"}"#).unwrap();
        router.complete(&mut sub, "worker.execute", "s", r#"{"intent":"y"}"#).unwrap();
        router.complete(&mut sub, "critic.review", "s", "{}").unwrap();
        let rows = reasoner_scorecard(&sub).unwrap();
        assert_eq!(rows.len(), 2); // (mock, deliberate) and (mock, review)
        let deliberate = rows.iter().find(|r| r.operator == "deliberate").unwrap();
        assert_eq!(deliberate.calls, 2);
        assert_eq!(deliberate.ok, 2);
        assert_eq!(deliberate.provider, "mock");
        assert!((deliberate.ok_rate() - 1.0).abs() < f64::EPSILON);
    }
}
