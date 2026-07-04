//! The Delegation Inbox (ARCHITECTURE §11), CLI-first: pending Diffs with
//! their effect lists and Critic verdicts. Approval binds to the effect list
//! and commits the transaction; rejection aborts it. Only the user holds
//! this authority in v1 (trust-ledger auto-commit arrives in Beta).

use std::path::Path;

use anyhow::{bail, Result};
use kernel::txn::{TxnManager, TxnState};
use substrate::Substrate;

use crate::blackboard::{ArtifactView, Blackboard};

pub struct InboxItem {
    pub task: String,
    pub txn: String,
    pub diff_id: String,
    pub intent: String,
    pub effects: Vec<serde_json::Value>,
    pub advisory_summary: String,
    pub verdict: String,
    pub issues: Vec<serde_json::Value>,
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
/// records the decision as blackboard transitions.
pub fn approve(data: &Path, sub: &mut Substrate, txn: &str) -> Result<usize> {
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
    Ok(applied.len())
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
    let mgr = TxnManager::new(data)?;
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
