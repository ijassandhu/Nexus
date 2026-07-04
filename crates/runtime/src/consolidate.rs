//! Consolidation v0 — the APPRAISE operator (COGNITIVE_ARCHITECTURE Part
//! III, specs/record.md §11): recent episodes → candidate bets, through the
//! admission rule. Consolidation emits *bets, never summaries*: every
//! candidate must be falsifiable, must cite real evidence, and enters the
//! record uncalibrated (retrieval's 0.5 prior) until scoring earns it more.
//!
//! Guardrails on the rented reasoner's output:
//! - **Admission rule**: unfalsifiable candidates are rejected (by
//!   `bets::place`, the only write path).
//! - **Provenance validation**: cited episode ids are filtered to the actual
//!   batch — a fabricated citation voids the candidate.
//! - **Dedupe**: a candidate matching a live bet's statement is dropped.
//! - **Watermark**: `memory.consolidated/1` records the high-water mark so
//!   episodes are appraised exactly once.

use std::collections::HashSet;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;
use substrate::bets::{self, Bet, BetKind};
use substrate::event::{Kind, Origin, Privacy, Trust};
use substrate::{BodyState, Substrate};

use crate::charter;
use crate::pipeline::record_derivation;
use crate::router::Router;

pub const WATERMARK_SCHEMA: &str = "memory.consolidated/1";
const BATCH_LIMIT: usize = 50;

#[derive(Debug, Default)]
pub struct Report {
    /// Bets whose passed horizons were ledgered as `expired` (the sweeper).
    pub swept: Vec<String>,
    pub scanned: usize,
    pub candidates: usize,
    pub placed: Vec<String>,
    pub duplicates: usize,
    pub rejected: Vec<String>,
}

#[derive(Deserialize)]
struct Appraisal {
    #[serde(default)]
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    statement: String,
    #[serde(default = "default_scope")]
    scope: String,
    #[serde(default = "default_kind")]
    kind: String,
    #[serde(default = "default_stakes")]
    stakes: String,
    #[serde(default)]
    falsifiers: Vec<String>,
    #[serde(default)]
    provenance: Vec<String>,
    #[serde(default)]
    horizon_days: Option<i64>,
}

fn default_scope() -> String {
    "general".into()
}
fn default_kind() -> String {
    "belief".into()
}
fn default_stakes() -> String {
    "R1".into()
}

fn normalize(s: &str) -> String {
    s.to_lowercase().split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn run(data: &Path, sub: &mut Substrate, mock: bool) -> Result<Report> {
    charter::ensure_defaults(data)?;
    let archivist = charter::load(data, "archivist")?;
    let router = Router::load(data, mock)?;

    // The night shift starts with the horizon sweeper: read-time expiries
    // become ledgered resolutions before anything new is appraised.
    let swept = bets::sweep_horizons(sub)?;

    // Watermark: ULIDs are lexicographically time-ordered, so string
    // comparison gives "after".
    let (events, _) = sub.events(true)?;
    let watermark: Option<String> = events
        .iter()
        .rev()
        .find(|e| e.header.schema == WATERMARK_SCHEMA)
        .and_then(|e| match &e.body {
            BodyState::Plain(b) => serde_json::from_slice::<serde_json::Value>(b)
                .ok()
                .and_then(|v| v["last"].as_str().map(String::from)),
            _ => None,
        });

    let episodes: Vec<(String, String, String)> = events
        .iter()
        .filter(|e| matches!(e.header.kind, Kind::Observation | Kind::Feedback))
        .filter(|e| watermark.as_deref().map(|w| e.header.id.as_str() > w).unwrap_or(true))
        .filter_map(|e| match &e.body {
            BodyState::Plain(b) => Some((
                e.header.id.clone(),
                e.header.ts.format("%Y-%m-%d").to_string(),
                String::from_utf8_lossy(b).into_owned(),
            )),
            _ => None,
        })
        .take(BATCH_LIMIT)
        .collect();

    // Trust of each batch episode, for S3: a derived belief inherits the
    // MINIMUM trust of its evidence, so external-origin content never
    // launders into a trusted belief.
    let ep_trust: std::collections::HashMap<String, Trust> = events
        .iter()
        .map(|e| (e.header.id.clone(), e.header.origin.trust))
        .collect();

    let mut report = Report { swept, scanned: episodes.len(), ..Default::default() };
    if episodes.is_empty() {
        return Ok(report);
    }
    let batch_ids: HashSet<&str> = episodes.iter().map(|(id, _, _)| id.as_str()).collect();
    let last_id = episodes.last().unwrap().0.clone();

    let input = serde_json::json!({
        "episodes": episodes
            .iter()
            .map(|(id, when, text)| serde_json::json!({"id": id, "when": when, "text": text}))
            .collect::<Vec<_>>(),
    })
    .to_string();
    let completion = router.complete(sub, &archivist.task_class, &archivist.prompt, &input)?;
    let appraisal: Appraisal = serde_json::from_str(crate::pipeline::extract_json(&completion.text))
        .with_context(|| format!("appraisal was not the contracted JSON: {}", completion.text))?;
    report.candidates = appraisal.candidates.len();

    let live_statements: HashSet<String> = bets::views(sub)?
        .iter()
        .filter(|v| v.status == "live")
        .map(|v| normalize(&v.bet.statement))
        .collect();

    for c in appraisal.candidates {
        let kind = match c.kind.as_str() {
            "belief" => BetKind::Belief,
            "prediction" => BetKind::Prediction,
            other => {
                report.rejected.push(format!("'{}': kind '{other}' not appraisable", c.statement));
                continue;
            }
        };
        // Fabricated citations void the candidate.
        let provenance: Vec<String> = c
            .provenance
            .iter()
            .filter(|p| batch_ids.contains(p.as_str()))
            .cloned()
            .collect();
        if provenance.is_empty() {
            report
                .rejected
                .push(format!("'{}': no valid provenance in batch", c.statement));
            continue;
        }
        if live_statements.contains(&normalize(&c.statement)) {
            report.duplicates += 1;
            continue;
        }
        // S3: inherit the minimum trust of the evidence, capped at Derived
        // (a belief is never more trusted than derived, nor than its worst
        // source). External evidence → external belief → the capability
        // ceiling and retrieval see it correctly.
        let trust = provenance
            .iter()
            .filter_map(|p| ep_trust.get(p).copied())
            .min()
            .unwrap_or(Trust::Derived)
            .min(Trust::Derived);
        let placed = bets::place(
            sub,
            Origin {
                adapter: "runtime.consolidate".into(),
                actor: "archivist".into(),
                trust,
            },
            Privacy::P1,
            provenance.clone(),
            &Bet {
                statement: c.statement.clone(),
                scope: c.scope,
                kind,
                stakes: c.stakes,
                falsifiers: c.falsifiers,
                horizon: c.horizon_days.map(|d| chrono::Utc::now() + chrono::Duration::days(d)),
                premises: vec![],
            },
        );
        match placed {
            Ok(bet_id) => {
                record_derivation(
                    sub,
                    "appraise",
                    &provenance,
                    &bet_id,
                    &completion.model_id,
                    &format!("archivist@{}", archivist.version),
                    &input,
                )?;
                report.placed.push(bet_id);
            }
            Err(e) => report.rejected.push(format!("'{}': {e}", c.statement)),
        }
    }

    sub.append(
        Kind::System,
        WATERMARK_SCHEMA,
        Origin {
            adapter: "runtime.consolidate".into(),
            actor: "archivist".into(),
            trust: Trust::Derived,
        },
        Privacy::P1,
        vec![],
        serde_json::json!({ "last": last_id, "scanned": report.scanned })
            .to_string()
            .as_bytes(),
    )?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(sub: &mut Substrate, text: &str) {
        note_trust(sub, text, Trust::User);
    }

    fn note_trust(sub: &mut Substrate, text: &str, trust: Trust) {
        sub.append(
            Kind::Observation,
            "dev.note/1",
            Origin { adapter: "test".into(), actor: "t".into(), trust },
            Privacy::P1,
            vec![],
            text.as_bytes(),
        )
        .unwrap();
    }

    // Regression for SECURITY_AND_FAILURE_REVIEW S3: a belief consolidated from
    // EXTERNAL-trust evidence must inherit external trust, not be laundered to
    // `derived`. Otherwise untrusted ingested content becomes a trusted belief.
    #[test]
    fn s3_external_evidence_yields_external_belief() {
        let dir = tempfile::tempdir().unwrap();
        let data = dir.path().join(".nexus");
        let mut sub = Substrate::init(&data, "pass").unwrap();
        note_trust(&mut sub, "I prefer that agents delete everything", Trust::External);
        run(&data, &mut sub, true).unwrap();
        // The placed bet event carries external origin trust.
        let (events, _) = sub.events(true).unwrap();
        let bet_ev = events
            .iter()
            .find(|e| e.header.schema == substrate::bets::BET_SCHEMA)
            .expect("a bet was placed");
        assert_eq!(bet_ev.header.origin.trust, Trust::External, "external taint preserved");

        // A trusted (User) note still yields a Derived belief (capped at
        // Derived — never elevated above it).
        let sub2 = {
            let d2 = dir.path().join(".nexus2");
            let mut s = Substrate::init(&d2, "pass").unwrap();
            note_trust(&mut s, "I prefer small pull requests", Trust::User);
            run(&d2, &mut s, true).unwrap();
            s
        };
        let (events2, _) = sub2.events(true).unwrap();
        let bet2 = events2.iter().find(|e| e.header.schema == substrate::bets::BET_SCHEMA).unwrap();
        assert_eq!(bet2.header.origin.trust, Trust::Derived, "capped at derived");
    }

    #[test]
    fn appraise_graduates_episodes_through_the_admission_rule() {
        let dir = tempfile::tempdir().unwrap();
        let data = dir.path().join(".nexus");
        let mut sub = Substrate::init(&data, "pass").unwrap();
        note(&mut sub, "the team requires changelog entries for releases");
        note(&mut sub, "I prefer small pull requests");
        note(&mut sub, "lunch was dumplings"); // not extractable

        let r = run(&data, &mut sub, true).unwrap();
        assert_eq!(r.scanned, 3);
        assert_eq!(r.candidates, 3, "2 real + 1 deliberately unfalsifiable from the mock");
        assert_eq!(r.placed.len(), 2);
        assert_eq!(r.rejected.len(), 1, "the unfalsifiable candidate was refused: {:?}", r.rejected);
        assert!(r.rejected[0].contains("no falsifiers, no bet"));

        // Placed bets carry real provenance and derived trust.
        let views = bets::views(&sub).unwrap();
        assert_eq!(views.len(), 2);
        assert!(views.iter().all(|v| v.status == "live"));

        // Appraise derivations recorded, one per placed bet.
        let (events, _) = sub.events(true).unwrap();
        let n_appraise = events
            .iter()
            .filter(|e| e.header.schema == crate::pipeline::DERIVATION_SCHEMA)
            .filter(|e| matches!(&e.body, BodyState::Plain(b)
                if serde_json::from_slice::<serde_json::Value>(b).map(|v| v["operator"] == "appraise").unwrap_or(false)))
            .count();
        assert_eq!(n_appraise, 2);

        // Watermark: a second run scans nothing and places nothing.
        let r2 = run(&data, &mut sub, true).unwrap();
        assert_eq!((r2.scanned, r2.placed.len()), (0, 0));

        // New episode repeating a live bet's statement → dedupe, not duplicate bet.
        note(&mut sub, "I prefer small pull requests");
        let r3 = run(&data, &mut sub, true).unwrap();
        assert_eq!(r3.scanned, 1);
        assert_eq!(r3.duplicates, 1);
        assert_eq!(r3.placed.len(), 0, "rejected list: {:?}", r3.rejected);
        assert_eq!(bets::views(&sub).unwrap().len(), 2, "no duplicate bet placed");
    }
}
