//! Stratum 1 of the memory architecture: bets (specs/record.md §11,
//! COGNITIVE_ARCHITECTURE.md Part IV).
//!
//! A bet is a position the system may rely on. Admission rule: **no
//! falsifiers, no bet** — enforced here, at the only write path. Confidence
//! is never stored; it is the fold of resolution events (SCORE). When a bet
//! is falsified or retracted, every transitive dependent is marked
//! unjustified (RECONCILE) — a conclusion never silently outlives its
//! premises.

use std::collections::HashMap;

use anyhow::{bail, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::event::{Kind, Origin, Privacy};
use crate::{BodyState, Substrate};

pub const BET_SCHEMA: &str = "memory.bet/1";
pub const RESOLUTION_SCHEMA: &str = "memory.resolution/1";

pub const STAKES: [&str; 4] = ["R0", "R1", "R2", "R3"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bet {
    pub statement: String,
    pub scope: String,
    pub kind: BetKind,
    /// Consequence class if acted on and wrong — reuses the capability
    /// taxonomy (stringly here to keep substrate free of a kernel dep).
    pub stakes: String,
    pub falsifiers: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizon: Option<DateTime<Utc>>,
    #[serde(default)]
    pub premises: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BetKind {
    Belief,
    Prediction,
    Premortem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Outcome {
    Held,
    Falsified,
    Expired,
    Retracted,
    Unjustified,
}

impl Outcome {
    pub fn parse(s: &str) -> Result<Self> {
        Ok(match s {
            "held" => Outcome::Held,
            "falsified" => Outcome::Falsified,
            "expired" => Outcome::Expired,
            "retracted" => Outcome::Retracted,
            "unjustified" => Outcome::Unjustified,
            _ => bail!("outcome must be held|falsified|expired|retracted|unjustified"),
        })
    }
    fn terminal(self) -> bool {
        matches!(self, Outcome::Falsified | Outcome::Expired | Outcome::Retracted)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Resolution {
    bet: String,
    outcome: Outcome,
    note: String,
    by: String,
}

/// The folded state of one bet: earned score, never asserted confidence.
#[derive(Debug, Clone)]
pub struct BetView {
    pub id: String,
    pub created: DateTime<Utc>,
    pub bet: Bet,
    /// "live" or the terminal outcome name.
    pub status: String,
    pub held: u32,
    pub falsified_count: u32,
    /// A premise died; retrievable only with an explicit warning label.
    pub unjustified: bool,
    /// For terminal bets: what killed it (the resolution note + who resolved).
    pub terminal_note: Option<String>,
}

/// Place a bet. The admission rule and stakes taxonomy are enforced here —
/// there is no other write path.
pub fn place(
    sub: &mut Substrate,
    origin: Origin,
    privacy: Privacy,
    provenance: Vec<String>,
    bet: &Bet,
) -> Result<String> {
    if bet.statement.trim().is_empty() {
        bail!("a bet needs a statement");
    }
    if bet.falsifiers.iter().filter(|f| !f.trim().is_empty()).count() == 0 {
        bail!(
            "no falsifiers, no bet: a position that cannot say what would kill it \
             may be an episode, never a reliance (specs/record.md §11.1)"
        );
    }
    if !STAKES.contains(&bet.stakes.as_str()) {
        bail!("stakes must be one of {STAKES:?}");
    }
    // Premises must exist and be bets (justification links are load-bearing).
    if !bet.premises.is_empty() {
        let known = views(sub)?;
        for p in &bet.premises {
            if !known.iter().any(|v| &v.id == p) {
                bail!("premise {p} is not a known bet");
            }
        }
    }
    let header = sub.append(
        Kind::System,
        BET_SCHEMA,
        origin,
        privacy,
        provenance,
        serde_json::to_string(bet)?.as_bytes(),
    )?;
    Ok(header.id)
}

/// Resolve a bet (SCORE). Falsified/retracted triggers RECONCILE: every
/// transitive dependent gets an `unjustified` resolution.
pub fn resolve(
    sub: &mut Substrate,
    bet_id: &str,
    outcome: Outcome,
    note: &str,
    by: &str,
) -> Result<usize> {
    let all = views(sub)?;
    let Some(target) = all.iter().find(|v| v.id == bet_id) else {
        bail!("no such bet: {bet_id}");
    };
    if target.status != "live" {
        bail!("bet {bet_id} is already terminal ({})", target.status);
    }
    append_resolution(sub, bet_id, outcome, note, by)?;

    let mut cascaded = 0usize;
    if matches!(outcome, Outcome::Falsified | Outcome::Retracted) {
        // dependents map: premise id → bets that cite it
        let mut dependents: HashMap<&str, Vec<&BetView>> = HashMap::new();
        for v in &all {
            for p in &v.bet.premises {
                dependents.entry(p.as_str()).or_default().push(v);
            }
        }
        let mut frontier = vec![bet_id.to_string()];
        let mut seen = std::collections::HashSet::new();
        while let Some(current) = frontier.pop() {
            for dep in dependents.get(current.as_str()).into_iter().flatten() {
                if !seen.insert(dep.id.clone()) {
                    continue;
                }
                if dep.status == "live" && !dep.unjustified {
                    append_resolution(
                        sub,
                        &dep.id,
                        Outcome::Unjustified,
                        &format!("premise {current} was {outcome:?}"),
                        "reconcile",
                    )?;
                    cascaded += 1;
                }
                frontier.push(dep.id.clone());
            }
        }
    }
    Ok(cascaded)
}

fn append_resolution(
    sub: &mut Substrate,
    bet: &str,
    outcome: Outcome,
    note: &str,
    by: &str,
) -> Result<()> {
    sub.append(
        Kind::System,
        RESOLUTION_SCHEMA,
        Origin {
            adapter: "substrate.bets".into(),
            actor: by.into(),
            trust: crate::event::Trust::Derived,
        },
        Privacy::P1,
        vec![bet.to_string()],
        serde_json::to_string(&Resolution {
            bet: bet.into(),
            outcome,
            note: note.into(),
            by: by.into(),
        })?
        .as_bytes(),
    )?;
    Ok(())
}

/// Fold the log into current bet views. Horizon expiry is applied at read
/// time (a bet past its horizon reads as expired even before a sweeper runs).
pub fn views(sub: &Substrate) -> Result<Vec<BetView>> {
    let (events, _) = sub.events(true)?;
    let mut order: Vec<String> = Vec::new();
    let mut map: HashMap<String, BetView> = HashMap::new();
    for e in &events {
        let BodyState::Plain(body) = &e.body else { continue };
        match e.header.schema.as_str() {
            BET_SCHEMA => {
                if let Ok(bet) = serde_json::from_slice::<Bet>(body) {
                    order.push(e.header.id.clone());
                    map.insert(
                        e.header.id.clone(),
                        BetView {
                            id: e.header.id.clone(),
                            created: e.header.ts,
                            bet,
                            status: "live".into(),
                            held: 0,
                            falsified_count: 0,
                            unjustified: false,
                            terminal_note: None,
                        },
                    );
                }
            }
            RESOLUTION_SCHEMA => {
                if let Ok(r) = serde_json::from_slice::<Resolution>(body) {
                    if let Some(v) = map.get_mut(&r.bet) {
                        match r.outcome {
                            Outcome::Held => v.held += 1,
                            Outcome::Unjustified => v.unjustified = true,
                            o if o.terminal() => {
                                if o == Outcome::Falsified {
                                    v.falsified_count += 1;
                                }
                                v.status = format!("{o:?}").to_lowercase();
                                v.terminal_note = Some(format!("{} (by {})", r.note, r.by));
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
    let now = Utc::now();
    let mut out: Vec<BetView> = order.into_iter().filter_map(|id| map.remove(&id)).collect();
    for v in &mut out {
        if v.status == "live" {
            if let Some(h) = v.bet.horizon {
                if h < now {
                    v.status = "expired".into();
                    v.terminal_note = Some("horizon passed unresolved (read-time expiry)".into());
                }
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Trust;

    fn origin() -> Origin {
        Origin { adapter: "test".into(), actor: "tester".into(), trust: Trust::User }
    }

    fn bet(statement: &str, falsifiers: Vec<&str>, premises: Vec<String>) -> Bet {
        Bet {
            statement: statement.into(),
            scope: "test".into(),
            kind: BetKind::Belief,
            stakes: "R1".into(),
            falsifiers: falsifiers.into_iter().map(String::from).collect(),
            horizon: None,
            premises,
        }
    }

    #[test]
    fn admission_rule_no_falsifiers_no_bet() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        let err = place(&mut sub, origin(), Privacy::P1, vec![], &bet("unfalsifiable", vec![], vec![]))
            .unwrap_err();
        assert!(err.to_string().contains("no falsifiers, no bet"));
        // Whitespace falsifiers don't count.
        assert!(place(&mut sub, origin(), Privacy::P1, vec![], &bet("x", vec!["  "], vec![])).is_err());
        // Bad stakes refused.
        let mut b = bet("x", vec!["y happens"], vec![]);
        b.stakes = "R9".into();
        assert!(place(&mut sub, origin(), Privacy::P1, vec![], &b).is_err());
    }

    #[test]
    fn score_is_the_fold_of_resolutions() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        let id = place(&mut sub, origin(), Privacy::P1, vec![], &bet("meetings run long", vec!["a week of on-time meetings"], vec![]))
            .unwrap();
        resolve(&mut sub, &id, Outcome::Held, "held this week", "score").unwrap();
        resolve(&mut sub, &id, Outcome::Held, "held again", "score").unwrap();
        let v = views(&sub).unwrap();
        assert_eq!(v[0].held, 2);
        assert_eq!(v[0].status, "live");
        resolve(&mut sub, &id, Outcome::Falsified, "on-time week observed", "score").unwrap();
        let v = views(&sub).unwrap();
        assert_eq!(v[0].status, "falsified");
        // Terminal bets refuse further resolution.
        assert!(resolve(&mut sub, &id, Outcome::Held, "", "score").is_err());
    }

    #[test]
    fn reconcile_cascades_unjustified_through_premises() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        let a = place(&mut sub, origin(), Privacy::P1, vec![], &bet("A", vec!["not-A observed"], vec![])).unwrap();
        let b = place(&mut sub, origin(), Privacy::P1, vec![], &bet("B (from A)", vec!["not-B"], vec![a.clone()])).unwrap();
        let c = place(&mut sub, origin(), Privacy::P1, vec![], &bet("C (from B)", vec!["not-C"], vec![b.clone()])).unwrap();
        let cascaded = resolve(&mut sub, &a, Outcome::Retracted, "user retracted", "user").unwrap();
        assert_eq!(cascaded, 2, "B and C both marked unjustified");
        let v = views(&sub).unwrap();
        let get = |id: &str| v.iter().find(|x| x.id == *id).unwrap();
        assert_eq!(get(&a).status, "retracted");
        assert!(get(&b).unjustified);
        assert!(get(&c).unjustified);
        // Held resolutions do NOT cascade.
        let d = place(&mut sub, origin(), Privacy::P1, vec![], &bet("D", vec!["not-D"], vec![])).unwrap();
        assert_eq!(resolve(&mut sub, &d, Outcome::Held, "", "score").unwrap(), 0);
    }

    #[test]
    fn unknown_premise_is_refused_and_horizon_expires_at_read() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        assert!(place(&mut sub, origin(), Privacy::P1, vec![], &bet("x", vec!["y"], vec!["01FAKE".into()])).is_err());
        let mut b = bet("stale", vec!["z"], vec![]);
        b.horizon = Some(Utc::now() - chrono::Duration::hours(1));
        place(&mut sub, origin(), Privacy::P1, vec![], &b).unwrap();
        assert_eq!(views(&sub).unwrap()[0].status, "expired");
    }
}
