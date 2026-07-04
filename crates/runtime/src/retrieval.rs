//! Retrieval v1 — W's candidate generator (COGNITIVE_ARCHITECTURE Part VI).
//!
//! Deliberately in-memory: events decrypt at query time and score in RAM, so
//! no plaintext ever rests outside the envelope model (the constraint that
//! kept FTS out of the derived store). Personal scale makes this fine for a
//! long time; when it isn't, an encrypted index replaces the scan behind
//! this interface.
//!
//! Two rankers:
//! - `recall_episodes`: lexical (idf-weighted overlap) × recency over
//!   user-content events (observations, feedback). Similarity *proposes*.
//! - `rank_bets`: lexical × **calibration** — earned score modulates rank,
//!   unjustified positions are discounted hard. Similarity never outranks a
//!   track record (RESEARCH_NOTES E-1, in miniature).

use std::collections::{HashMap, HashSet};

use anyhow::Result;
use chrono::Utc;
use substrate::bets::BetView;
use substrate::event::Kind;
use substrate::{BodyState, Substrate};

pub const DEFAULT_BUDGET: usize = 8;
const EXCERPT_LEN: usize = 240;
const RECENCY_HALF_LIFE_DAYS: f64 = 30.0;

#[derive(Debug, Clone)]
pub struct Recall {
    pub id: String,
    pub when: String,
    pub source: String,
    pub excerpt: String,
    pub score: f64,
}

fn tokens(s: &str) -> HashSet<String> {
    s.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| t.len() > 2)
        .map(String::from)
        .collect()
}

fn recency(age_days: f64) -> f64 {
    0.5f64.powf(age_days.max(0.0) / RECENCY_HALF_LIFE_DAYS)
}

/// Rank user-content episodes against a query. Internal machinery events
/// (ledger, artifacts, derivations, router telemetry) never surface here —
/// recall is for what the user saw and said, not what the system did.
pub fn recall_episodes(sub: &Substrate, query: &str, budget: usize) -> Result<Vec<Recall>> {
    let (events, _) = sub.events(true)?;
    let q = tokens(query);
    if q.is_empty() {
        return Ok(vec![]);
    }

    struct Doc {
        id: String,
        when: String,
        source: String,
        text: String,
        toks: HashSet<String>,
        age_days: f64,
    }
    let now = Utc::now();
    let docs: Vec<Doc> = events
        .iter()
        .filter(|e| matches!(e.header.kind, Kind::Observation | Kind::Feedback))
        .filter_map(|e| match &e.body {
            BodyState::Plain(b) => {
                let text = String::from_utf8_lossy(b).into_owned();
                Some(Doc {
                    id: e.header.id.clone(),
                    when: e.header.ts.format("%Y-%m-%d").to_string(),
                    source: e.header.schema.clone(),
                    toks: tokens(&text),
                    age_days: (now - e.header.ts).num_seconds() as f64 / 86_400.0,
                    text,
                })
            }
            _ => None,
        })
        .collect();

    let n = docs.len().max(1) as f64;
    let mut df: HashMap<&str, f64> = HashMap::new();
    for d in &docs {
        for t in &d.toks {
            *df.entry(t.as_str()).or_default() += 1.0;
        }
    }

    let mut out: Vec<Recall> = docs
        .iter()
        .filter_map(|d| {
            let lex: f64 = q
                .iter()
                .filter(|t| d.toks.contains(*t))
                .map(|t| (n / df.get(t.as_str()).copied().unwrap_or(1.0)).ln().max(0.1))
                .sum::<f64>()
                / (d.toks.len() as f64 + 1.0).sqrt();
            if lex <= 0.0 {
                return None;
            }
            let score = lex * (0.3 + 0.7 * recency(d.age_days));
            let mut excerpt: String = d.text.chars().take(EXCERPT_LEN).collect();
            if d.text.chars().count() > EXCERPT_LEN {
                excerpt.push('…');
            }
            Some(Recall {
                id: d.id.clone(),
                when: d.when.clone(),
                source: d.source.clone(),
                excerpt,
                score,
            })
        })
        .collect();
    out.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    out.truncate(budget);
    Ok(out)
}

/// Calibration factor: earned score modulates retrievability. The +2
/// denominator prior means an unscored bet starts at 0.5, not 1.0 —
/// **uncalibrated is not the same as perfectly calibrated** (this matters
/// once consolidation mass-produces derived bets). Falsifications count
/// double (being wrong is worse than being right is good), and an
/// unjustified position — its premises died — is discounted to a whisper.
pub fn calibration(v: &BetView) -> f64 {
    let base = (1.0 + v.held as f64) / (2.0 + v.held as f64 + 2.0 * v.falsified_count as f64);
    if v.unjustified {
        base * 0.25
    } else {
        base
    }
}

/// Rank live bets by lexical relevance × calibration. The additive base on
/// the lexical term means equal-relevance bets are ordered purely by track
/// record — similarity proposes, calibration decides.
pub fn rank_bets(views: &[BetView], query: &str, budget: usize) -> Vec<(BetView, f64)> {
    let q = tokens(query);
    let mut out: Vec<(BetView, f64)> = views
        .iter()
        .filter(|v| v.status == "live")
        .map(|v| {
            let doc = tokens(&format!("{} {}", v.bet.statement, v.bet.scope));
            let lex = q.iter().filter(|t| doc.contains(*t)).count() as f64
                / (q.len() as f64 + 1.0);
            let score = (0.2 + lex) * calibration(v);
            (v.clone(), score)
        })
        .collect();
    out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    out.truncate(budget);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use substrate::bets::{self, Bet, BetKind, Outcome};
    use substrate::event::{Origin, Privacy, Trust};

    fn origin() -> Origin {
        Origin { adapter: "test".into(), actor: "t".into(), trust: Trust::User }
    }

    #[test]
    fn episodes_rank_by_relevance_and_machinery_is_excluded() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        sub.append(Kind::Observation, "dev.note/1", origin(), Privacy::P1, vec![],
            b"the deploy pipeline uses blue-green rollout on fridays").unwrap();
        sub.append(Kind::Observation, "dev.note/1", origin(), Privacy::P1, vec![],
            b"grocery list: apples, coffee").unwrap();
        // Machinery event with matching words must NOT surface.
        sub.append(Kind::Action, "ledger.effect/1", origin(), Privacy::P1, vec![],
            b"deploy pipeline rollout effect").unwrap();

        let recalls = recall_episodes(&sub, "how does the deploy rollout work", 5).unwrap();
        assert_eq!(recalls.len(), 1, "only the relevant observation: {recalls:?}");
        assert!(recalls[0].excerpt.contains("blue-green"));
        assert_eq!(recall_episodes(&sub, "", 5).unwrap().len(), 0);
    }

    #[test]
    fn calibration_outranks_similarity_e1_miniature() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        let mk = |s: &str| Bet {
            statement: s.into(), scope: "work".into(), kind: BetKind::Belief,
            stakes: "R1".into(), falsifiers: vec!["evidence otherwise".into()],
            horizon: None, premises: vec![],
        };
        // Identical lexical relevance to the query, different track records.
        let good = bets::place(&mut sub, origin(), Privacy::P1, vec![], &mk("standup meeting starts late")).unwrap();
        let bad = bets::place(&mut sub, origin(), Privacy::P1, vec![], &mk("standup meeting starts early")).unwrap();
        for _ in 0..3 {
            bets::resolve(&mut sub, &good, Outcome::Held, "", "score").unwrap();
        }
        // `bad` accrues falsifications via a premise chain: make it unjustified instead.
        let dep = bets::place(&mut sub, origin(), Privacy::P1, vec![],
            &Bet { premises: vec![bad.clone()], ..mk("standup meeting scheduling belief") }).unwrap();
        bets::resolve(&mut sub, &bad, Outcome::Retracted, "wrong", "user").unwrap();

        let views = bets::views(&sub).unwrap();
        let ranked = rank_bets(&views, "when does the standup meeting start", 10);
        // The well-scored bet ranks first; the unjustified dependent ranks
        // below it despite identical lexical signal; the retracted bet is
        // gone entirely (not live).
        assert_eq!(ranked[0].0.id, good);
        assert!(ranked.iter().all(|(v, _)| v.id != bad), "terminal bets never surface");
        let dep_rank = ranked.iter().position(|(v, _)| v.id == dep).unwrap();
        assert!(dep_rank > 0);
        let good_cal = calibration(&views.iter().find(|v| v.id == good).unwrap().clone());
        let dep_cal = calibration(&views.iter().find(|v| v.id == dep).unwrap().clone());
        assert!(good_cal > 3.0 * dep_cal, "earned score dominates: {good_cal} vs {dep_cal}");
    }
}
