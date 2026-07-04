//! NEXUS agent runtime v0 (Phase 1): the blackboard, charters-as-data, the
//! static model router with outcome logging, and the first agent loop —
//! Steward → Worker (inside a transaction) → Critic → Delegation Inbox.
//!
//! Agents are stateless (D-006): every function here wakes, reads the
//! substrate, does bounded work, writes artifacts, and returns. There is no
//! in-memory continuity to lose.

pub mod blackboard;
pub mod charter;
pub mod consolidate;
pub mod inbox;
pub mod onboarding;
pub mod pipeline;
pub mod providers;
pub mod retrieval;
pub mod router;
pub mod scorecard;

#[cfg(test)]
mod loop_tests {
    use crate::blackboard::Blackboard;
    use crate::{inbox, pipeline};
    use std::fs;
    use substrate::Substrate;

    #[test]
    fn full_mock_loop_approve_applies_to_target() {
        let dir = tempfile::tempdir().unwrap();
        let data = dir.path().join(".nexus");
        let mut sub = Substrate::init(&data, "pass").unwrap();
        let target = dir.path().join("proj");
        fs::create_dir_all(&target).unwrap();
        fs::write(target.join("existing.txt"), "hello").unwrap();

        let result =
            pipeline::run_intent(&data, &mut sub, "capture this note", &target, true).unwrap();
        assert_eq!(result.effects, 1);
        assert_eq!(result.verdict, "approve");
        let txn = result.txn.clone().unwrap();

        // Nothing applied yet — the target is untouched pre-approval.
        assert!(!target.join("NOTES.md").exists());

        // Inbox shows the pending item with intent, effects, and verdict.
        let items = inbox::list(&data, &sub).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].intent, "capture this note");
        assert_eq!(items[0].effects.len(), 1);
        assert_eq!(items[0].verdict, "approve");

        // Approve → committed to target; inbox drains; states fold to done.
        let applied = inbox::approve(&data, &mut sub, &txn).unwrap();
        assert_eq!(applied, 1);
        assert!(target.join("NOTES.md").exists());
        assert!(fs::read_to_string(target.join("NOTES.md")).unwrap().contains("capture this note"));
        assert!(inbox::list(&data, &sub).unwrap().is_empty());

        let views = Blackboard::load(&sub).unwrap();
        let intent = views.iter().find(|v| v.artifact.atype == "Intent").unwrap();
        assert_eq!(intent.state, "done");
        let diff = views.iter().find(|v| v.artifact.atype == "Diff").unwrap();
        assert_eq!(diff.state, "approved");

        // Every judgment left a derivation event with a manifest and its
        // working-set preimage (specs/record.md §11.3–11.4).
        let (events, _) = sub.events(true).unwrap();
        let derivations: Vec<_> = events
            .iter()
            .filter(|e| e.header.schema == crate::pipeline::DERIVATION_SCHEMA)
            .collect();
        assert_eq!(derivations.len(), 2, "one per model judgment (worker + critic)");
        for d in &derivations {
            let substrate::BodyState::Plain(b) = &d.body else { panic!() };
            let v: serde_json::Value = serde_json::from_slice(b).unwrap();
            assert_eq!(v["manifest"].as_str().unwrap().len(), 64, "BLAKE3 hex manifest");
            assert_eq!(d.header.refs.len(), 1, "references its working-set event");
        }
        assert_eq!(
            events.iter().filter(|e| e.header.schema == crate::pipeline::WORKINGSET_SCHEMA).count(),
            2
        );
    }

    #[test]
    fn memory_influences_judgment_and_derivations_cite_bets() {
        let dir = tempfile::tempdir().unwrap();
        let data = dir.path().join(".nexus");
        let mut sub = Substrate::init(&data, "pass").unwrap();
        let target = dir.path().join("proj");
        fs::create_dir_all(&target).unwrap();

        // Round 1: reject → premortem bet placed with the reason.
        let r1 = pipeline::run_intent(&data, &mut sub, "add a config file", &target, true).unwrap();
        inbox::reject(&data, &mut sub, &r1.txn.unwrap(), "config lives in repo root").unwrap();
        let bet_id = substrate::bets::views(&sub).unwrap()[0].id.clone();

        // Round 2: the caution enters the working set and shapes the output.
        let r2 = pipeline::run_intent(&data, &mut sub, "add a config file again", &target, true).unwrap();
        assert!(
            r2.plan.iter().any(|s| s.contains("heeding 1 caution")),
            "worker plan must acknowledge the premortem: {:?}",
            r2.plan
        );
        inbox::approve(&data, &mut sub, &r2.txn.unwrap()).unwrap();
        let notes = fs::read_to_string(target.join("NOTES.md")).unwrap();
        assert!(notes.contains("config lives in repo root"), "caution surfaced in the artifact");

        // The derivation event lists the bet id as an input (credit assignment).
        let (events, _) = sub.events(true).unwrap();
        let last_deliberate = events
            .iter()
            .filter(|e| e.header.schema == pipeline::DERIVATION_SCHEMA)
            .filter_map(|e| match &e.body {
                substrate::BodyState::Plain(b) => serde_json::from_slice::<serde_json::Value>(b).ok(),
                _ => None,
            })
            .filter(|v| v["operator"] == "deliberate")
            .next_back()
            .unwrap();
        let inputs: Vec<&str> = last_deliberate["inputs"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|i| i.as_str())
            .collect();
        assert!(inputs.contains(&bet_id.as_str()), "derivation must cite the bet it relied on");

        // An out-of-scope bet does NOT enter the working set.
        let other_scope = substrate::bets::place(
            &mut sub,
            substrate::event::Origin {
                adapter: "test".into(), actor: "tester".into(), trust: substrate::event::Trust::User,
            },
            substrate::event::Privacy::P1,
            vec![],
            &substrate::bets::Bet {
                statement: "irrelevant elsewhere-bet".into(),
                scope: "/completely/unrelated/path".into(),
                kind: substrate::bets::BetKind::Premortem,
                stakes: "R1".into(),
                falsifiers: vec!["n/a".into()],
                horizon: None,
                premises: vec![],
            },
        )
        .unwrap();
        let r3 = pipeline::run_intent(&data, &mut sub, "third task", &target, true).unwrap();
        assert!(
            r3.plan.iter().any(|s| s.contains("heeding 1 caution")),
            "still exactly one in-scope caution — the unrelated bet stayed out"
        );
        let _ = other_scope;
    }

    #[test]
    fn full_mock_loop_reject_leaves_target_untouched() {
        let dir = tempfile::tempdir().unwrap();
        let data = dir.path().join(".nexus");
        let mut sub = Substrate::init(&data, "pass").unwrap();
        let target = dir.path().join("proj");
        fs::create_dir_all(&target).unwrap();

        let result = pipeline::run_intent(&data, &mut sub, "note", &target, true).unwrap();
        let txn = result.txn.unwrap();

        // Rejection requires a reason (training signal).
        assert!(inbox::reject(&data, &mut sub, &txn, "  ").is_err());
        inbox::reject(&data, &mut sub, &txn, "not what I meant").unwrap();
        assert!(!target.join("NOTES.md").exists());
        assert!(inbox::list(&data, &sub).unwrap().is_empty());

        let views = Blackboard::load(&sub).unwrap();
        assert_eq!(views.iter().find(|v| v.artifact.atype == "Intent").unwrap().state, "cancelled");
        assert_eq!(views.iter().find(|v| v.artifact.atype == "Diff").unwrap().state, "rejected");

        // Failures are memory: the rejection filed an Incident and placed a
        // premortem bet carrying the user's reason.
        let incident = views.iter().find(|v| v.artifact.atype == "Incident").unwrap();
        assert_eq!(incident.artifact.body["narrative"], "not what I meant");
        let bets = substrate::bets::views(&sub).unwrap();
        assert_eq!(bets.len(), 1);
        assert_eq!(bets[0].bet.kind, substrate::bets::BetKind::Premortem);
        assert!(bets[0].bet.statement.contains("not what I meant"));
        assert_eq!(bets[0].status, "live");
        assert!(!bets[0].bet.falsifiers.is_empty(), "premortems are falsifiable like any bet");
    }
}
