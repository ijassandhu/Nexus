//! The Delegation Inbox (ARCHITECTURE §11), CLI-first: pending Diffs with
//! their effect lists and Critic verdicts. Approval binds to the effect list
//! and commits the transaction; rejection aborts it. Only the user holds
//! this authority in v1 (trust-ledger auto-commit arrives in Beta).

use std::path::Path;

use anyhow::{bail, Result};
use kernel::txn::{TxnManager, TxnState};
use substrate::bets::{BetKind, Outcome};
use substrate::Substrate;

use crate::blackboard::{ArtifactView, Blackboard};

/// Adapter that marks a premortem as born from the delegation loop. Automated
/// settlement only touches the loop's own premortems (S1) — a user's
/// hand-placed caution is theirs to settle.
const FLOW_ADAPTER: &str = "runtime.inbox";

/// The premortems this task actually relied on AND that the loop itself
/// created, still live. This is the exact set automated settlement may touch
/// (S1): causal (the derivation cited them) and owned (flow-born), so an
/// unrelated approval can never silence a caution it never used.
fn settleable_premortems(sub: &Substrate, cited: &[String]) -> Result<Vec<String>> {
    let live: std::collections::HashMap<String, substrate::bets::BetView> =
        substrate::bets::views(sub)?
            .into_iter()
            .filter(|v| v.status == "live" && v.bet.kind == BetKind::Premortem)
            .filter(|v| v.origin_adapter == FLOW_ADAPTER)
            .map(|v| (v.id.clone(), v))
            .collect();
    Ok(cited.iter().filter(|id| live.contains_key(*id)).cloned().collect())
}

pub struct InboxItem {
    pub task: String,
    pub txn: String,
    pub diff_id: String,
    pub intent: String,
    pub effects: Vec<serde_json::Value>,
    pub advisory_summary: String,
    pub verdict: String,
    pub issues: Vec<serde_json::Value>,
    /// Premortem ids this task relied on (S1) — the only bets settlement touches.
    pub cited_premortems: Vec<String>,
}

fn views_by_task<'a>(views: &'a [ArtifactView], task: &str, atype: &str) -> Option<&'a ArtifactView> {
    views.iter().find(|v| v.artifact.task == task && v.artifact.atype == atype)
}

pub fn list(data: &Path, sub: &Substrate) -> Result<Vec<InboxItem>> {
    let views = Blackboard::load(sub)?;
    let mgr = TxnManager::new(data)?;
    let open_txns: Vec<String> = mgr
        .list()?
        .into_iter()
        .filter(|m| m.state == TxnState::Open)
        .map(|m| m.id)
        .collect();

    let mut items = Vec::new();
    for v in views.iter().filter(|v| v.artifact.atype == "Diff" && v.state == "proposed") {
        let Some(txn) = v.artifact.body.get("txn").and_then(|t| t.as_str()) else { continue };
        if !open_txns.contains(&txn.to_string()) {
            continue;
        }
        let task = &v.artifact.task;
        let intent = views_by_task(&views, task, "Intent")
            .and_then(|i| i.artifact.body.get("text").and_then(|t| t.as_str()).map(String::from))
            .unwrap_or_default();
        let (verdict, issues) = views_by_task(&views, task, "Critique")
            .map(|c| {
                (
                    c.artifact.body.get("verdict").and_then(|x| x.as_str()).unwrap_or("?").to_string(),
                    c.artifact.body.get("issues").and_then(|i| i.as_array()).cloned().unwrap_or_default(),
                )
            })
            .unwrap_or(("(no critique)".into(), vec![]));
        items.push(InboxItem {
            task: task.clone(),
            txn: txn.to_string(),
            diff_id: v.artifact.id.clone(),
            intent,
            effects: v.artifact.body.get("effect_list").and_then(|e| e.as_array()).cloned().unwrap_or_default(),
            advisory_summary: v.artifact.body.get("advisory_summary").and_then(|s| s.as_str()).unwrap_or("").to_string(),
            verdict,
            issues,
            cited_premortems: v
                .artifact
                .body
                .get("cited_premortems")
                .and_then(|a| a.as_array())
                .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
                .unwrap_or_default(),
        });
    }
    Ok(items)
}

fn find_item(data: &Path, sub: &Substrate, txn: &str) -> Result<InboxItem> {
    list(data, sub)?
        .into_iter()
        .find(|i| i.txn == txn)
        .ok_or_else(|| anyhow::anyhow!("no pending inbox item for txn {txn}"))
}

/// User approval: binds to the effect list (D-015), commits the transaction,
/// records the decision as blackboard transitions — then performs
/// **automated settlement** (Phase 1 gate): approving a task that relied on a
/// flow-born premortem meets that premortem's stated falsifier ("a similar
/// task is later approved"). The system settles against its own convenience —
/// the caution that just helped is retired by the letter of its contract
/// (mechanical-settlement razor, D-021); if the failure recurs, a rejection
/// mints a fresh premortem. Settlement touches ONLY premortems this task cited
/// and the loop created (S1) — never an unrelated caution. Resolver is
/// `settlement.flow`, never a human.
///
/// Returns (effects applied, premortem ids auto-settled).
pub fn approve(data: &Path, sub: &mut Substrate, txn: &str) -> Result<(usize, Vec<String>)> {
    let item = find_item(data, sub, txn)?;
    let mgr = TxnManager::new(data)?;
    let applied = mgr.commit(sub, txn)?;
    Blackboard::transition(sub, &item.diff_id, "Diff", "proposed", "approved", "user", "approved from inbox")?;
    let views = Blackboard::load(sub)?;
    if let Some(intent) = views_by_task(&views, &item.task, "Intent") {
        if intent.state == "review" {
            Blackboard::transition(sub, &intent.artifact.id, "Intent", "review", "done", "user", "effects applied")?;
        }
    }

    let mut settled = Vec::new();
    for id in settleable_premortems(sub, &item.cited_premortems)? {
        substrate::bets::resolve(
            sub,
            &id,
            Outcome::Falsified,
            &format!(
                "falsifier met mechanically: relied-upon task \"{}\" approved (txn {txn})",
                item.intent
            ),
            "settlement.flow",
        )?;
        settled.push(id);
    }
    Ok((applied.len(), settled))
}

/// User rejection: aborts the transaction (target untouched), records why —
/// and the why becomes memory: an Incident artifact plus a **premortem bet**
/// (COGNITIVE_ARCHITECTURE Part IV: failures are memory), retrievable into
/// future working sets and scored like any other position.
pub fn reject(data: &Path, sub: &mut Substrate, txn: &str, reason: &str) -> Result<()> {
    let item = find_item(data, sub, txn)?;
    if reason.trim().is_empty() {
        bail!("a rejection needs a reason — it is training signal (ARCHITECTURE §12)");
    }
    // Symmetric automated settlement: the failure pattern recurred, so each
    // relied-upon flow-born premortem earns a `held` (score +1, stays live).
    for id in settleable_premortems(sub, &item.cited_premortems)? {
        substrate::bets::resolve(
            sub,
            &id,
            Outcome::Held,
            &format!("relied-upon task \"{}\" rejected again", item.intent),
            "settlement.flow",
        )?;
    }
    let mgr = TxnManager::new(data)?;
    // The failing task's target scopes the new premortem (so a later
    // same-target task can settle it). Read it before the abort.
    let target = mgr
        .list()?
        .into_iter()
        .find(|m| m.id == txn)
        .map(|m| m.target.to_string_lossy().into_owned())
        .unwrap_or_default();
    mgr.abort(sub, txn)?;
    Blackboard::transition(sub, &item.diff_id, "Diff", "proposed", "rejected", "user", reason)?;
    let views = Blackboard::load(sub)?;
    if let Some(intent) = views_by_task(&views, &item.task, "Intent") {
        if intent.state == "review" {
            Blackboard::transition(sub, &intent.artifact.id, "Intent", "review", "cancelled", "user", reason)?;
        }
    }

    let incident = crate::blackboard::Artifact::new(
        "Incident",
        &item.task,
        crate::blackboard::Author {
            role: "steward".into(),
            charter_version: "0.1.0".into(),
            model_id: None,
        },
        "open",
        vec![item.diff_id.clone()],
        serde_json::json!({
            "failure_class": "specification",
            "narrative": reason,
            "intent": item.intent,
        }),
    );
    Blackboard::append(sub, &incident, substrate::event::Trust::Derived)?;

    substrate::bets::place(
        sub,
        substrate::event::Origin {
            adapter: "runtime.inbox".into(),
            actor: "user".into(),
            trust: substrate::event::Trust::User,
        },
        substrate::event::Privacy::P1,
        vec![incident.id.clone()],
        &substrate::bets::Bet {
            statement: format!("tasks like \"{}\" fail: {}", item.intent, reason),
            scope: target,
            kind: substrate::bets::BetKind::Premortem,
            stakes: "R1".into(),
            falsifiers: vec!["a similar task is later approved without edits".into()],
            horizon: None,
            premises: vec![],
        },
    )?;
    Ok(())
}
