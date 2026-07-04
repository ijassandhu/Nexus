# NEXUS — Roadmap

**Version 2.1 · July 2026.** (v2.1 applies the D-020 adversarial-review cuts: skills
marketplace deleted; decision-journal surface deleted — decisions harvested from existing
flows; blackboard demoted to internal protocol; Phase 2 gate upgraded to the pre-registered
experiment below.) Supersedes v1.1 *ordering*, not its commitments: every gate
already passed stays passed, every spec stays valid, every CLI command and event schema
survives unchanged. Identity per DECISIONS.md D-018: **a personal epistemic institution —
knowledge, beliefs, decisions, evidence, reasoning — with the assistant as Client #1 and
primary evidence pump.**

## Backward-compatibility guarantees (binding)

1. All existing event schemas (`memory.*`, `bb.*`, `txn.*`, `ledger.*`, `router.call/1`) remain
   valid forever within their major version; changes are additive per the sacred-interface
   discipline (ARCHITECTURE §2).
2. All existing `nx` commands keep their semantics; new surfaces are new commands.
3. Records created under v1.x roadmap phases replay, rebuild, and export identically.
4. The v1.1 phase gates already passed remain recorded: **Phase 0 (kernel spike) — PASSED
   2026-07-04** (abort losslessness, truthful effect lists, signed ledger, rebuild idempotency).

## v1.1 → v2.0 phase mapping

| v1.1 | v2.0 | What changed |
|---|---|---|
| Phase 0 kernel spike ✅ | Phase 0 ✅ (unchanged) | — |
| Phase 1 "deputy for developers" | Phase 1 "Institution v0 + Client #1" | Same work, re-titled; epistemic loop closure added (already done in practice) |
| Phase 2 Alpha "memory that compounds" | Phase 2 "The Honest Record" | Consolidation-quality gate replaced by **calibration gate**; decision artifacts and human epistemic surfaces promoted from later phases |
| Phase 3 Beta "earned autonomy" | Phase 3 "Mandates & earned autonomy" | Unchanged in substance; mandates (goal theory) join it |
| Phase 4 V1 hardening | Phase 4 "The institution ships" | Adds the fourth spec (epistemic layer) to publication |
| Phase 5–6 | Phase 5–6 | Org edition reframed as **federated institutions** |

## The three tracks

Work is tagged, not siloed — each phase advances all three:
- **K (epistemic kernel)** — the identity: bets, scoring, reconciliation, derivations,
  consolidation, decisions, mandates, calibration.
- **A (applications)** — clients of the kernel: #1 delegation (assistant), #2 decision
  journal, #3 research/knowledge corpus.
- **P (platform)** — kernel-of-trust, providers, adapters, sync, plugin SDK, spec publication.

---

## Phase 0 — Kernel spike ✅ *(complete, unchanged from v1.1)*

> **✅ GATE PASSED — 2026-07-04** (PROGRESS.md session 5): abort tree-hash lossless; 4-effect
> refactor committed with hash-accurate effect list; all ledger signatures verified; rebuild
> idempotent.

## Phase 1 — Institution v0 + Client #1 *(current; loop closed, gate pending)*

**Built already** (sessions 6–12): four-role runtime, provider-agnostic router with sealed keys,
bets + admission rule + SCORE/RECONCILE, derivation manifests, W v0.3 (cautions/beliefs/recalls),
consolidation with guardrails, premortems from rejections, per-reasoner scorecards.

**Remaining:**
- [A] **Real-model run** — blocked on user API key (`nx configure`); charters have only met the mock.
- [K] Horizon sweeper: consolidation pass resolving expired-horizon bets as `expired` in the ledger.
- [K] **First automated settlement**: an approved similar task resolves the matching premortem's
  falsifier — the first SCORE the system performs on itself.
- [P] Derived-store views for derivations and consolidation state.

**Gate:** the v1.1 MVP gate (≥5 real delegated tasks/week per pilot, commit-rate >80%, median
review <90 s) **plus**: at least one automated settlement recorded, and `nx bets` shows ≥10
live positions the pilot did not hand-author.

**Risk retired:** "Will a person delegate through a review queue, and does use produce
positions without authoring effort?"

## Phase 2 — The Honest Record *(the re-centered Alpha; kernel-heavy)*

The phase where the institution becomes visibly *for* knowledge, beliefs, decisions, evidence,
and reasoning — and where its honesty becomes measurable.

- [K] **Automated settlement at scale**: task outcomes settle predictions; approvals settle
  premortem falsifiers; horizon expiry ledgers; the settlement pipeline is the phase's spine.
- [K] **Decision capture from existing flows** (D-020: no journaling product): approve/reject/
  edit rulings are recorded as decisions with expected-outcome bets automatically; `nx decide`
  exists as optional explicit capture, not as a gated surface. Nothing asks the user to journal.
- [K] **`nx calibration`** — the reliability report: settled bets bucketed by implied
  confidence vs. actual hold-rate, per kind and per source (user-authored vs. consolidated vs.
  reasoner-extracted). "How right have I been?" becomes a command.
- [K] **Weekly reconciliation review**: one batched, low-attention queue of contradictions,
  unjustified positions, expiring horizons, and unresolved decisions (the V1 "reflection"
  concept, now with mechanical content).
- [K] **User model v0**: predictions about the user's rulings, settled by the inbox stream —
  the best-calibrated component, inspectable via `nx bets`.
- [A] Client #2: the decision journal as a daily-use surface (CLI-first, like everything).
- [P] Blackboard/derivation materialization into the derived store; embedding-provider hook for
  retrieval's vector candidates (local-first per privacy tier).
- Full **E-1** (calibration-weighted vs. similarity retrieval on a real pilot record) and
  **E-2** (manifest-based failure attribution) run here.

**Gate — the pre-registered experiment (D-020; can falsify the invention itself):** over ≥8
weeks of one pilot's record with ≥100 *mechanically settled* bets (the razor: interpretive
bets bucketed separately):
1. **Predictive validity:** bet scores must predict future settlement outcomes better than a
   no-skill baseline (AUC > 0.65 on held-out settlements) — otherwise the scoring is
   bookkeeping, not knowledge.
2. **Behavioral value:** calibration-ranked working sets must reduce task rejection rate by
   ≥20% relative to similarity-ranked working sets on matched real tasks (E-1, full form).
Fail either arm after two iterations → the D-018 kill-criterion review triggers. **Stall point
by design.**

**Risk retired:** the project's #1 research risk, sharpened — not "are extracted claims
correct?" but "does the record *keep itself* honest at tolerable attention cost?"

## Phase 3 — Mandates & earned autonomy *(Beta, substantially as v1.1)*

- [K] **Mandates**: goals as expiring, re-affirmed authority over attention and purpose;
  ORIENT computes goal pressure into candidate intents; recommitment cadences; conflicts →
  DecisionMemos. (COGNITIVE_ARCHITECTURE Part V.)
- [A] Assistant deepens *because* epistemics now gates it: trust-ledger auto-commit tiers for
  low-stakes effect classes; comms adapters (mail/calendar) behind the effect boundary with
  held outbox; skills ladder to `autonomous` under scored promotion.
- [P] Plugin SDK opens (WASM + MCP wrap); injection drills as CI (capability ceilings in
  anger); multi-device sync v1 behind its v1.1 gate (real second-device demand).

**Gate:** ≥30% of recurring task volume autonomous with incident rate trending down; zero
ceiling escapes in drills; zero zombie goals (every active mandate re-affirmed within cadence);
E-3 (compilation cascade) and E-4 (premise-linked plan invalidation) pass.

## Phase 4 — The institution ships *(V1)*

- [P] **Publish three specs** — record, capability, **epistemic layer** (bet/resolution/
  derivation/working-set schemas and fold semantics) — with conformance tests; a third party
  builds a client against them. (Blackboard demoted to internal versioned protocol per D-020:
  integrators need the record and its law, not our wiring.)
- [P] Duress modes, record partitioning, jurisdiction-aware retention (unchanged V1 blocker);
  export/import round-trip incl. bets and settlements; self-hosted relay.
- [A] The positions ledger and briefing as first-class surfaces: the morning diff includes
  *belief changes* (new positions, settlements, reconciliations) beside world changes.
- E-5 (model-swap regression replay against stored manifests) becomes the standard model-
  adoption procedure.

**Gate:** external security review of kernel + key hierarchy; a third-party spec client exists;
and the institution's defining demo — a pilot answers *"what do I believe about X, on what
evidence, and how right have I been?"* in under 30 seconds, with provenance.

## Phase 5 — Federated institutions *(the reframed org edition)*

Calibration-sharing without content-sharing (RESEARCH_NOTES X-3): institutions exchange scores
and priors, never records. Team edition = institutions with inter-personal capability grants
and shared mandates — a *treaty between institutions*, not a shared database. Compensation
framework for irreversibles (carried from v1.1 Phase 5). *(Skills marketplace deleted, D-020.)*

## Phase 6 — Apps as views *(the decade bet, unchanged)*

The record as primary copy for select domains, conventional apps rendering it. Entered only
from an earned ecosystem position.

---

## Risk register (v2.0)

| Risk | Phase | Mitigation | Kill criterion |
|---|---|---|---|
| **Philosophy-toy risk** (epistemic surfaces unused) | 2 | Every K feature maps to remember/decide/delegate; system drafts, human vetoes | Pilots use delegation but ignore epistemic surfaces two review periods running → recenter on assistant (D-018) |
| Settlement starvation (bets never resolve) | 2 | Automated settlement is the phase spine; assistant retained as evidence pump | Median bet unsettled at 2× horizon → gate fails |
| Prompt injection arms race | 3 | Ceilings (D-014), effect boundary, drills-as-CI | Ceiling escape in the wild → freeze autonomy expansion |
| Review-cost creep | 1–3 | Measure review-seconds; risk-ranked grouping | Median review >3 min/task and rising |
| Adapter treadmill | 3+ | ≤2 blessed adapters/domain; plugin SDK; MCP | Boundary work >40% of engineering 2 quarters |
| Platform sherlocking | all | Lean into what platforms can't copy: user-owned honest records, neutral routing, verifiable forgetting, open specs | — (strategic, monitored) |

---

STATUS: COMPLETE
