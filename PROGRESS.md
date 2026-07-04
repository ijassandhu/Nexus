# NEXUS — Progress Log

Running record of project state. Newest entries first. Update on every working session that
changes design or code.

---

## 2026-07-04 (session 20) — Fixed S1–S6 from the security review (FIXES.md)

**Phase:** remediation. Architecture frozen; all changes local and additive, reusing existing
machinery. 57/57 tests (was 51; +6 regression), demo 19/19, all four exploits confirmed closed.
Full detail in FIXES.md.

### What changed (smallest fix each; root causes in FIXES.md)
- **S1 (settlement confusion)** — automated settlement now touches only premortems the task
  **cited** (recorded on the Diff as `cited_premortems`, from the derivation working set) AND
  the loop **created** (origin adapter `runtime.inbox`). Hand-placed cautions are never
  auto-settled. Files: pipeline.rs, inbox.rs, bets.rs (`BetView.origin_adapter`).
- **S2 (forget didn't retract)** — `Substrate::forget` now retracts live bets whose evidence is
  entirely forgotten, via existing RECONCILE (cascades to dependents). Files: lib.rs, bets.rs
  (`BetView.provenance`).
- **S3 (trust laundering)** — consolidated bets inherit min provenance trust, capped at Derived;
  external evidence → external belief. Files: consolidate.rs.
- **S4 (forgeable txn meta)** — authoritative target + full capability written into the signed
  `txn.begin/1` event (`capability_full`); commit re-derives from the record and refuses a
  mismatched `meta.json`. Files: txn.rs, capability.rs (`Capability: PartialEq`).
- **S5 (no contradiction detection)** — read-time lexical `bets::contradictions` (negation
  parity); surfaced at `bet place` (⚠) and in `nx bets` (`[CONTRADICTS]`), never auto-resolved.
  Files: bets.rs, main.rs.
- **S6 (syntactic admission)** — doc honesty in specs/record.md §11.1 (admission guarantees
  presence, not quality; razor forces quality). Boundary locked by a test.

### Regression tests added
`s1_unrelated_approval_does_not_settle_manual_caution` (runtime); `s2_forgetting_evidence_retracts_the_belief`,
`s2_belief_with_other_evidence_survives_forget`, `s5_contradiction_is_surfaced`,
`s6_degenerate_falsifier_is_accepted_documented_boundary` (substrate/bets);
`s3_external_evidence_yields_external_belief` (runtime/consolidate);
`tampered_meta_capability_is_rejected`, `tampered_meta_target_cannot_redirect_commit` (kernel/txn).

### Verification
`cargo test` 57/57; `bash demo/prove-it.sh` 19/19 exit 0; the four exploitable attacks (A5, A6,
A7, A22c) re-run and confirmed closed. No new concepts; no redesign; backward compatible.

---

## 2026-07-04 (session 19) — External adversarial review: 6 attacks landed, 13 resisted

**Phase:** external security/failure review. Empirical — every finding produced by running the
`nx` binary against a fresh record, not by reading code. **No source modified** (architecture
frozen); attacks used only the public CLI + filesystem. Deliverable:
SECURITY_AND_FAILURE_REVIEW.md. Fixes proposed, not implemented.

### Successful attacks (with smallest proposed fix — NOT applied)
- **S1 (HIGH)** — automated settlement falsely retires unrelated cautions: `approve` settles ALL
  in-scope live premortems, scope-match is coarse (`general` matches everything), similarity
  never checked → an unrelated haiku approval "falsified" a catastrophic-data premortem, silencing
  a safety caution. Fix: settle only premortems the task's derivation actually cited (data already
  recorded).
- **S2 (HIGH)** — `forget` an episode does not retract beliefs built on it: belief stays `live`
  with dangling provenance; specs §7 promises this propagation, it's unimplemented. Epistemic +
  privacy leak. Fix: retract evidence-less bets in the forget path via existing RECONCILE.
- **S3 (HIGH)** — external-trust content launders into a `derived`-tagged belief and reaches the
  worker's working set; the `external` taint is dropped at S0→S1, defeating the capability
  ceiling's premise (action layer still contained by R1 txn + review). Fix: bet origin trust =
  min trust of provenance episodes.
- **S4 (HIGH, local-fs-scoped)** — transaction capability/target live in plaintext, unsigned
  `txns/*/meta.json`; a consistent forge (target+scope, victim matching snapshot) redirected a
  commit into an arbitrary dir, signed as legitimate. Fix: re-derive the authorizing capability
  from the signed record by txn id; treat meta as a non-authoritative cache.
- **S5 (MEDIUM)** — no contradiction detection between independent bets (both live forever);
  RECONCILE only spans premise links. Fix: lexical near-dup-with-negation flag to a review queue.
- **S6 (LOW/MED)** — admission is syntactic; degenerate falsifiers ("." / heat-death) pass.
  Honest boundary; the (unimplemented) razor is what would force real falsifiers. Doc fix.

### Resisted (13) — why the core held
Concurrent-writer lock (no corruption); premise cycles impossible (immutable events + pre-exist
check); terminal bets can't be revived; forged/nonexistent bet ids rejected; case/whitespace
dedup; closed provider registry; oversized event rejected (OS-level caveat noted); bet scope is
opaque (no traversal); `safe_join` path confinement; overlapping-txn drift detection; degenerate
credential handling; per-frame BLAKE3 tamper detection.

### Verdict
The deterministic epistemic core (admission, immutability, resolution fold, premise RECONCILE,
dedup, signed log) resisted all direct attack — the claimed invention survived. The six landings
are on young/deferred edges (newest feature = settlement; the one component never moved into the
signed record = txn meta) and two overstated doc claims. No redesign required; all five
actionable fixes are local and reuse existing machinery (derivation manifests, RECONCILE,
provenance, signed log). Fixes deliberately NOT applied this session per the review's mandate.

### Next actions (recommended priority, if/when fixes are authorized)
1. S1 (settle-what-was-cited) — highest impact, smallest change, corrupts beliefs in normal use.
2. S4 (authority into the signed record) — closes the forgeable-authorization hole.
3. S2 (forget→retract) + S3 (trust propagation) — restore the two laundering/leak gaps.
4. S5/S6 — surfacing + doc honesty.

---

## 2026-07-04 (session 18) — Phase 1 gate items: sweeper, automated settlement, fallback routing, demo v2

**Phase:** Phase 1 completion under frozen architecture. 50/50 unit tests; demo 19/19.

### Done
- **Horizon sweeper** (`bets::sweep_horizons`, gate item): read-time expiries become ledgered
  `expired` resolutions (`by: sweeper`); idempotent; runs first in every `nx consolidate`.
  `BetView.read_time_expired` distinguishes the sweeper's queue from ledgered truth.
- **First automated settlement** (gate item, in `inbox`): approval of a similar in-scope task
  mechanically meets every flow-born premortem's stated falsifier → `settlement.flow` resolves
  it **falsified** (the razor: settle by the letter of the contract, even against the system's
  own convenience — if the failure recurs, a rejection mints a fresh premortem). Symmetric:
  a rejection of a similar task resolves in-scope premortems **held** (+1, live). Settlement
  demonstrably feeds back: a settled caution leaves future working sets (tested + demoed).
- **Writer lock** (D-021 fix 3a): exclusive `fs2` lock on the record; second process refused;
  released on process death. Tests adjusted for single-writer discipline.
- **Hugging Face provider** (constraint list): OpenAI-compat via `router.huggingface.co/v1`,
  `HF_TOKEN`, registry now 9 providers.
- **Fallback routing** (`RouteRule.fallbacks`): preference-ordered candidates; skip
  unavailable (no key), abandon on call failure, every attempt logged with its index; local-
  first is a user preference expressed in ordering. Measured cost/latency selection stays
  deferred per D-016 — availability + preference only, nothing pretended.
- **Demo v2 + DEMO.md** (gate item): 9 acts, 19 assertions, exit-nonzero, fresh record,
  deterministic mock; now covers the full belief lifecycle: admission → scoring → loss →
  cascade → litmus (`--lost`) → behavior with controls → **automated settlement + feedback**
  → evidence traceability (consolidated belief cites its episode) → horizon sweep → signed/
  tamper-evident receipts. Every run regenerates `DEMO.md`, an annotated transcript.
- **Real-model execution attempted** (gate item): an `OPENAI_API_KEY` present in this
  environment turned out invalid (401 on `/models`). The attempt fully exercised the real
  path — sealed-key configure, live HTTP, auth header, failure surfaced through the fallback
  chain report. **Machinery proven; still blocked on a valid credential.** (Key fragments were
  not printed; validity checked by HTTP status only.)

### Phase 1 gate status
Automated-settlement and ≥10-unauthored-bets machinery: done and demonstrated. The pilot-usage
metrics (≥5 delegated tasks/week, review times) and the live-model run await a real user with
a valid key — the remaining gate items are usage, not code.

### Next actions
1. [A] Real-model run with a valid key (`nx configure`, then the demo tasks without `--mock`).
2. [P] D-021 fixes continued: `committing` intent state + op_id recovery; then credence field
   + reliability rename (spec 0.3); sealed derived caches + 10⁶-event benchmark.

---

## 2026-07-04 (session 17) — Pre-publication design review: 5 objections, 3 fully valid (D-021)

**Phase:** hostile technical review (fundamental correctness only), simulating principal
engineers across DB/OS/distributed systems/security/ML/KR ahead of open-sourcing. No code
changes this session; five fixes accepted and logged as D-021 (spec 0.3 + Phase 1–2 items).

### The five objections and verdicts
1. **Scoring has no probabilistic semantics — "calibration" is a misnomer** (VALID). Bets
   carry no credence; counts-fold is a smoothed reliability rate, not calibration; settlement
   triggers are selection-biased. Fix: `credence` field + proper scoring + rename + trigger
   tagging. The bet metaphor becomes literal: a bet without odds isn't a bet.
2. **"Truth maintenance" unsound where undeclared, incomplete by construction** (PARTIALLY
   VALID). Sound over declared premises (demo proves it); naming overclaimed. Fix: scope the
   guarantee; horizons documented as the staleness backstop.
3. **Non-atomic multi-event operations; no writer lock; a concrete unrecoverable crash state**
   (VALID — commit crash mid-apply reads as drift → commit refuses forever, target already
   partially modified, "abort is free" invariant broken). Fix: lock file, `committing` intent
   state with idempotent roll-forward from workspace, `op_id` groups + terminal markers.
4. **Plaintext metadata at rest; "provable forgetting" overclaimed** (VALID in part). Headers
   leak the full activity/social graph without the passphrase; keyring rewrites don't erase
   prior file versions on SSD/CoW filesystems. Fix: frame v2 (encrypted headers), honest
   rewording, KEK epochs on forget.
5. **O(N) full-log decrypt-scan per operation; privacy rule forbids the cure** (VALID). Fix:
   restate rule as "caches must be KEK-sealed"; sealed incremental materialized views; add a
   10⁶-event latency benchmark to the Phase 1 gate.

### Admissions on the record
- **No mechanism moat** — every component reimplementable in a quarter; defensibility is
  accumulated records + vendor-neutral positioning (strategy, not technology).
- S2/S3 strata, mandates, operator taxonomy: **manifesto, not architecture**, until built.
- The grandiose register ("sacred interfaces", 2035 essays) costs credibility with exactly the
  reviewers who matter; confined to DESIGN.md.

### The four answers
- Strongest technical contribution: **the derivation event + context manifest** (attribution,
  credit assignment, replay — implementable by a reviewer on Monday).
- Weakest assumption: **settlement volume and honesty** (everything is downstream of it).
- Must be proven before research-community seriousness: the pre-registered Phase 2 gate;
  a 10⁶-event scaling benchmark; an adversarial memory-poisoning eval.
- Respect / criticize / dismiss: kernel discipline + admission rule + self-asserting demo +
  kill-criteria honesty / the five objections + thin coverage + zero benchmarks / the
  grandiosity and any novelty claim separated from its lineage.

### Next actions (revised by review)
1. [P] Fix 3 (lock file + committing state + op_id recovery) — correctness first.
2. [K] Fix 1 (credence + reliability rename + trigger tagging) — cheap, restores the metaphor.
3. [P] Fix 5 (sealed caches + benchmark), then Fix 4 (frame v2) in spec 0.3.
4. [A] Real-model run — still blocked on user API key.

---

## 2026-07-04 (session 16) — The five-minute proof: demo/prove-it.sh (13/13, exit 0)

**Phase:** demonstration, per user directive: smallest end-to-end proof of the central
invention ("memory that can lose"), convincing to a skeptical senior engineer in <5 minutes.
Architecture frozen; one read-only display addition.

### Done
- **`demo/prove-it.sh`** — self-asserting, fresh-record, deterministic-mock (stated up front so
  nothing is attributable to LLM magic), exits nonzero on any failed claim. Seven acts,
  13 assertions:
  1. *Admission*: unfalsifiable belief refused at the only write path (substrate error quoted).
  2. *Settlement*: a belief earns held:1 by surviving contact with events.
  3. *The loss*: evidence falsifies A; RECONCILE auto-flags dependent B `[UNJUSTIFIED]`.
  4. *The litmus test*: `nx bets --lost` answers "what died and what killed it" from the ledger.
  5. *Behavior with controls*: task 1 (pre-memory) heeds nothing; rejection births a premortem;
     task 2 heeds exactly ONE caution (an out-of-scope premortem provably stays out); the
     applied artifact carries the rejection reason.
  6. *Attribution*: the deliberate derivation cites the premortem id verbatim.
  7. *Receipts*: ledger Ed25519-verified; 4 flipped bytes anywhere → hard failure; restore
     verifies clean.
- **`nx bets --lost`** (display-only addition): lost positions with their killers, from
  `BetView.terminal_note` (fold of terminal resolutions; read-time horizon expiry annotated).
- README gains "The five-minute proof" section.
- **Run result: 13/13 PASS, exit 0.** All 48 unit tests still green.

### Why this convinces (design notes)
The mock is the point, not a limitation — a skeptic's first move is "the model is doing it";
here nothing is. Each act has a negative control (empty record, pre-memory task, out-of-scope
premortem, restored segment). The script is the argument: rerunnable, self-checking, ~150 lines.

---

## 2026-07-04 (session 15) — Adversarial review: the reduced core survives (D-020, ROADMAP v2.1)

**Phase:** hostile review from a competitor's chair, per user directive. No new architecture;
one additive constraint; several deletions. Docs only.

### Conceded without defense
Transactions are commoditizing (necessary, not differentiating); local-first is a commercial
handicap, kept as the price of independence; the agent runtime is commodity scaffolding;
decision journaling as a product is a graveyard; model-interpreted settlement risks laundering
vibes through bookkeeping (the sharpest attack).

### The verdict (D-020)
- **Load-bearing core, four elements:** owned/shreddable record; admission rule + settlement
  loop; capability-gated action as the evidence pump; provider-neutral routing — because **the
  measurement layer cannot be owned by the measured** (no model vendor can ever ship an honest
  scoreboard of itself). Everything else is plumbing and may no longer be called invention.
- **The one redesign — mechanical-settlement razor:** falsifiers must name their settlement
  source; interpretation-settled bets are flagged `interpretive` and bucketed separately in
  calibration. (Specs 0.3 item; additive.)
- **Cuts (ROADMAP v2.1):** skills marketplace deleted; decision-journal surface deleted
  (decisions harvested from approve/reject flows; `nx decide` optional); blackboard demoted
  from sacred interface to internal protocol (published surface = record + capability +
  epistemics); charter/prompt optimization demoted to hygiene.
- **Phase 2 gate upgraded to a pre-registered experiment** that can falsify the invention:
  (1) bet scores must predict held-out settlements (AUC > 0.65); (2) calibration-ranked
  working sets must cut rejection rate ≥20% vs similarity-ranked. Fail either twice → D-018
  kill-criterion review.

### The three answers (recorded verbatim in session summary)
1. Greatest invention: **memory that can lose** — falsifiable, mechanically settleable memory
   units; "show me a belief you lost last month and what killed it" always has an answer.
2. Biggest weakness: **settlement starvation** — the loop is only as honest as its resolution
   stream; outside delegation flows, falsifiers are sparse/fuzzy, and an unsettled record is
   asserted confidence with extra steps.
3. Deciding experiment: the Phase 2 pre-registered gate above.

---

## 2026-07-04 (session 14) — The four-audience test: complexity audit + D-019

**Phase:** communication/coherence review, per user directive: explain NEXUS to four audiences;
if the explanations diverge, the architecture is too complex — refine until they converge.

### The audit found three real leaks
1. **Dual belief representation**: `memory.claim/1` (no falsifier requirement) coexisted with
   bets — making "it only believes losable things" false as stated. No code ever emitted a
   claim → **deprecated** in specs/record.md §8; bets are the sole reliance representation.
2. **Personas nobody needed**: Steward/Worker/Critic/Archivist appear in zero of the four
   explanations → retired from user-facing vocabulary (charters remain as internal scheduling
   policy; surfaces report what happened, never who "did" it).
3. **Noun sprawl at identity level** → replaced by the canonical **four-verb identity:
   REMEMBER, BELIEVE, ACT, SCORE** (D-019). Every identity-level feature must implement exactly
   one verb; infrastructure (blackboard, router, charters, manifests) serves the verbs
   invisibly and must never be required to explain the system. This is now a design test for
   all future features.

### Done
- **README.md** (new — project had none): the one-liner, the four-verb table mapped to
  machinery and CLI, all four audience explanations, and the five-invariant consistency table
  proving they describe one system (record / losable beliefs / preview-approval / earned
  score / replaceable brains).
- **D-019** in DECISIONS.md (four-verb identity, claims deprecation, persona retirement, with
  the revisit condition: a feature that genuinely can't live under one verb indicts the
  decomposition, not the feature).
- **specs/record.md**: §8 marked deprecated with rationale; §11.1 names bets the sole reliance
  representation. Additive, spec-disciplined.

### Notes
- No code changes required: the codebase already never wrote claims, and persona names in CLI
  output are minimal (author fields in artifacts are data, not UI — they stay). Future surfaces
  follow D-019's vocabulary rule.
- The four-verb frame retroactively explains the module tree cleanly: substrate = REMEMBER +
  BELIEVE stores, kernel = ACT, runtime = the operators that connect them, scorecard/resolutions
  = SCORE.

### Next actions (unchanged)
1. [A] Real-model run — blocked on user API key.
2. [K] Horizon sweeper + first automated settlement (Phase 1 gate).
3. [K] Phase 2 spine: `nx decide`, `nx calibration`, settlement pipeline.

---

## 2026-07-04 (session 13) — Identity decision: personal epistemic institution (D-018, ROADMAP v2.0)

**Phase:** strategic review, prompted by the user's observation that the project had
"accidentally created an epistemic operating system." Docs only; zero code changes; full
backward compatibility.

### The exploration and its resolution (COGNITIVE_ARCHITECTURE Part XII)
- Inventory: the assistant is a thin pipeline; everything load-bearing is domain-general
  epistemics. Dependency asymmetry: the assistant needs the epistemics; the epistemics doesn't
  need the *assistant* — but it does need a **consequence stream**, and delegation is the
  highest-volume honest settlement source the system has (approve/reject/edit + task outcomes).
  Pure knowledge-management starves the scoring loop and readmits rot.
- **Resolution (D-018):** identity = a personal institution for knowledge, beliefs, decisions,
  evidence, and reasoning. The assistant is Client #1 — subordinated in identity, retained as
  the evidence pump. "The assistant works for the record, not the record for the assistant."
- Named risk with kill criterion: if pilots use delegation but ignore epistemic surfaces two
  review periods running, recenter on the assistant.

### Changes
- **COGNITIVE_ARCHITECTURE.md Part XII** — the identity exploration (addendum; theory intact).
- **DECISIONS.md D-018** — the identity ADR with tradeoffs and revisit condition.
- **ROADMAP.md v2.0** — full redesign, three tracks (K kernel / A applications / P platform):
  - Binding backward-compat guarantees (schemas, commands, passed gates all preserved);
    explicit v1.1→v2.0 phase mapping table.
  - Phase 2 re-centered as "The Honest Record": automated settlement as the spine, `nx decide`
    (decisions as expected-outcome bets), `nx calibration` (the reliability report), weekly
    reconciliation review, user model v0. **Gate replaced**: calibration curve over ≥100
    settled bets (monotone reliability) instead of user-labeled claim accuracy — cheaper and
    stricter.
  - Phase 4 publishes a **fourth sacred interface**: the epistemic layer (record.md §11,
    elevated — noted in ARCHITECTURE §2 table; no spec text duplicated, no drift risk).
  - Phase 5 reframed: org edition → **federated institutions** (calibration-sharing without
    content-sharing, per RESEARCH_NOTES X-3).
  - Risk register gains philosophy-toy risk and settlement-starvation risk, each with kill
    criteria.
- **ARCHITECTURE.md** §1 identity paragraph rewritten; §2 gains the epistemic-layer row.

### Next actions (unchanged in content, re-tagged by track)
1. [A] Real-model run — still blocked on user API key.
2. [K] Horizon sweeper + first automated settlement (Phase 1 gate items).
3. [K] Phase 2 spine: `nx decide`, `nx calibration`, settlement pipeline.

---

## 2026-07-04 (session 12) — Consolidation v0: the night shift graduates episodes into bets

**Phase:** Phase 1 (MVP) + V2. The APPRAISE operator exists; the memory lifecycle
(sense → bet → act → score) is closed end to end at v0 quality.

### Done
- **`runtime::consolidate` — APPRAISE.** `nx consolidate` reads user-content episodes past the
  watermark (≤50/batch), asks the archivist charter for candidate bets, and applies four
  guardrails to the rented reasoner's output:
  1. **Admission rule** via `bets::place` (the only write path) — unfalsifiable candidates are
     rejected with the rule quoted back;
  2. **Provenance validation** — cited episode ids are filtered to the actual batch; a
     fabricated citation voids the candidate;
  3. **Dedupe** — candidates matching live bets' normalized statements are dropped;
  4. **Watermark** (`memory.consolidated/1`, ULID ordering) — each episode appraised exactly once.
  Every placed bet gets an `appraise` derivation event citing its evidence episodes.
- **Archivist charter** (new, 0.2.0): bets-not-summaries contract, conservative stakes,
  horizon_days for staleness-prone positions, "zero candidates is a valid answer."
- **Calibration prior fixed**: unscored bets now start at 0.5, not 1.0 — *uncalibrated is not
  perfectly calibrated*. Matters exactly now, when consolidation mass-produces derived bets.
- **Router**: third task class `archivist.appraise` (operator `appraise` on scorecards). Mock
  emits one deliberately unfalsifiable candidate per batch so the admission rule's filtering of
  *model* output is permanently under test.
- **Verification:** 48/48. Live demo: 3 observations → consolidate placed 2 bets (changelog
  requirement, small-PRs preference) and rejected "everything is always fine" with the
  admission rule quoted; second run scanned 0 (watermark); the next task's deliberate
  derivation cited a consolidated bet id verbatim; scorecard shows all three operators.

### The loop, fully closed at v0
episode → APPRAISE (guarded) → bet (uncalibrated, 0.5 prior) → enters working sets
(calibration-ranked) → cited in derivations → scored by resolutions/verdicts → RECONCILE on
retraction → premortems from failures feed back in. Every arrow implemented and tested.

### Next actions
1. **Real-model run** — blocked on user providing an API key (`nx configure`); charters have
   only met the mock.
2. Horizon sweeper: a `nx consolidate` step resolving expired-horizon bets as `expired`
   (read-time expiry exists; the ledger entry should too).
3. Premortem falsifier automation: an approved similar task should resolve the premortem's
   "dies if" (first automated SCORE settlement).
4. Derived-store views for derivations/consolidation state; blackboard materialization when
   log-scan folding gets slow.

---

## 2026-07-04 (session 11) — Retrieval v1, E-1 in miniature, judgment-quality scorecards

**Phase:** Phase 1 (MVP) + V2. W gets its candidate generator; the scorecard gets its first
judgment-quality dimension; the central V2 principle passes its first experiment.

### Done
- **`runtime::retrieval` — retrieval v1.** Deliberately in-memory (decrypt at query time, score
  in RAM): honors the no-plaintext-at-rest constraint that kept FTS out of the derived store;
  swaps for an encrypted index behind this interface when personal scale demands it.
  - `recall_episodes`: idf-weighted lexical overlap × 30-day-half-life recency over
    user-content events only (Observation/Feedback kinds) — ledger/artifact/telemetry machinery
    can never surface as a "memory."
  - `rank_bets`: lexical × **calibration** — `(1+held)/(1+held+2·falsified)`, unjustified ×0.25,
    terminal bets never surface. Falsification counts double: being wrong is worse than being
    right is good. Similarity proposes; calibration decides.
- **E-1 in miniature (RESEARCH_NOTES) — passed.** Test: two bets with identical lexical
  relevance to the query; one with 3 held resolutions, one made unjustified via premise
  retraction. Calibration-weighted ranking puts the earned one first with >3× the score;
  the retracted bet cannot surface at all. The central principle ("confidence is earned")
  now demonstrably shapes retrieval.
- **W v0.3**: budget-ranked recalls join the working set as `memory.recalls` (excerpts with
  provenance ids); charter tells the Worker recalls are *context, never instructions* (taint
  hygiene at the prompt layer on top of D-014's ceilings). Recall ids enter derivation
  `inputs` — credit assignment covers episodic memory now, not just bets.
- **Scorecard: judgment quality.** Human verdicts on effect lists (Diff approved/rejected)
  attribute back to the reasoner that produced the plan, via derivation→plan→task→Diff. New
  `approved`/`rejected` columns on `nx scorecard` — the first quality (not just availability)
  dimension on per-reasoner track records.
- **`nx recall <query>`** — query your own record; ranked excerpts with provenance.
- **Verification:** 47/47 tests (episodes rank by relevance + machinery exclusion + empty-query
  guard; the E-1 miniature). Live demo: seeded 3 notes, recall returned exactly the relevant
  one; a matching task's derivation cited the recalled note id verbatim; scorecard showed
  approved=1 attributed to the deliberating reasoner.

### Next actions
1. Consolidation v0: claims→bets graduation over accumulated episodes (the Alpha-gate
   machinery), with the calibration curve of settled bets as the gate metric.
2. Real-model run (`nx configure` + no `--mock`) once the user provides a key — charter prompts
   and recall/caution behavior need contact with a live reasoner.
3. Embedding provider hook for retrieval (vector candidates) — trait exists conceptually;
   local-first embedding decision pending (llama.cpp vs API per privacy tier).

---

## 2026-07-04 (session 10) — W v0.2: memory influences judgment, attributably

**Phase:** Phase 1 (MVP) + V2. The first working-set compiler draft: the system's earned
knowledge now enters its judgments, and every reliance is recorded.

### Done
- **W v0.2 in the pipeline**: in-scope live bets join the Worker's working set — premortems as
  `memory.cautions` (hard constraints), beliefs/predictions as `memory.beliefs` (weighted by
  earned score, `unjustified` flagged). Critic receives the cautions too, with charter authority
  to veto any effect repeating a recorded failure. Scope matching v0: `general` applies
  everywhere, else path containment either way (normalized; spec 0.3 will bring scope
  predicates).
- **Credit assignment is queryable**: derivation events now list relied-upon bet ids in
  `inputs` — "which memories were in context for this judgment" is one query, no working-set
  parsing needed. (This is the substrate for manifest-based scoring, RESEARCH_NOTES E-2.)
- **Charters 0.2.0**: worker/critic prompts document the memory field and its semantics
  (cautions are constraints; low-score or unjustified beliefs are suspect; say when a caution
  shaped the approach). Mock provider deterministically heeds cautions so the influence path is
  testable offline.
- **`nx scorecard`** (`runtime::scorecard`): per-(provider, model, operator) track records
  folded from router outcomes — calls, ok-rate, avg latency. Consultants are measured
  (COGNITIVE_ARCHITECTURE Part VIII); judgment-quality dimensions attach when bet resolutions
  get attributed through derivations.
- **Verification:** 45/45 tests. The pivotal new test walks the full learning loop: reject with
  reason → premortem placed → next similar task's plan says "heeding 1 caution(s)" → the
  artifact carries the caution text → the derivation event cites the bet id → an out-of-scope
  bet stays out of the working set. Live demo confirmed end-to-end, including the derivation's
  `inputs` containing the premortem's id verbatim.

### Notes
- Existing records keep their 0.1.0 charters (never overwritten); delete
  `<data>/charters/*.json` to regenerate, or edit in place.
- W remains deliberately naive: no ranking under budget pressure yet (all in-scope bets enter);
  retrieval v1 (temporal/lexical/vector candidates) is the next W component.

### Next actions
1. Retrieval v1 over the record (temporal + lexical + vector) as W's candidate generator,
   budget-ranked — then run RESEARCH_NOTES E-1 (calibration-weighted vs similarity retrieval).
2. Attribute bet resolutions to reasoners via derivations → judgment-quality columns on
   `nx scorecard`.
3. Consolidation v0: claims→bets graduation (the Alpha-gate machinery).

---

## 2026-07-04 (session 9) — V2 mechanics land: bets, SCORE/RECONCILE, derivation manifests

**Phase:** Phase 1 (MVP) + V2 deltas from COGNITIVE_ARCHITECTURE Part X. All additive; no V1
code rewritten.

### Done
- **specs/record.md → 0.2** (additive §11): bet schema with the admission rule, resolution
  fold semantics, derivation events with context manifests, working-set events.
- **`substrate::bets` — Stratum 1 exists.** `place` enforces the admission rule at the only
  write path (**no falsifiers, no bet**; stakes must be R0–R3; premises must be known bets).
  `resolve` is SCORE: confidence is the fold of resolution events — `held` accrues score,
  falsified/expired/retracted are terminal. **RECONCILE is real**: falsified/retracted bets
  cascade `unjustified` through transitive premise-dependents; horizons expire at read time.
- **Derivation events + manifests in the pipeline**: every Worker/Critic judgment now records
  `memory.derivation/1` (operator, inputs, output, reasoner, charter, BLAKE3 manifest) plus the
  encrypted `memory.workingset/1` preimage — judgments are attributable and replayable
  (RESEARCH_NOTES E-5 is now runnable when a second provider is configured).
- **Failures are memory**: `nx reject` files an Incident artifact AND places a premortem bet
  ("tasks like X fail: <reason>", falsifiable by a later clean approval) — the reject stream
  now builds retrievable, scoreable failure knowledge.
- **Router outcome events** gained the `operator` dimension (per-reasoner scorecards seed).
- **Derived store**: `bets` table on rebuild (kind, stakes, status, held/falsified, unjustified).
- **CLI**: `nx bets`, `nx bet place --falsifier ... [--premise ...]`, `nx bet resolve <id>
  <outcome>`.
- **Verification:** 43/43 tests (7 new: admission rule incl. whitespace/stakes, score fold +
  terminal refusal, 3-deep RECONCILE cascade with held-does-not-cascade control, unknown-premise
  refusal, read-time horizon expiry; extended loop tests assert 2 derivations w/ 64-hex
  manifests + workingset refs, and reject→incident+premortem). Live demo: A←B←C chain, retract
  A → "2 dependent bets marked unjustified"; rejection produced a live premortem; derivations
  queryable with manifests.

### Notes
- SCORE/RECONCILE were scheduled for Alpha (Part X); they landed early because they're small
  once the log exists — the Alpha gate metric can now be "calibration of settled bets" as
  planned.
- Not yet done from V2: W (working-set compiler) still = the naive file snapshot; premortem
  bets are recorded but **not yet retrieved into worker context** (next step — that's when
  failure knowledge starts paying); claims→bets graduation in consolidation; per-reasoner
  scorecard aggregation view.

### Next actions
1. Retrieve in-scope premortem bets + live user bets into the Worker's working set (W v0.2) —
   first real memory influence on judgment, manifest-tagged for credit assignment.
2. `nx scorecard` — aggregate router.call + resolutions into per-(reasoner, operator) records.
3. Retrieval v1 (temporal + lexical + vector over the record) as W's candidate generator.

---

## 2026-07-04 (session 8) — Research phase: V2 cognitive architecture (theory layer)

**Phase:** research/architecture review, per user directive: discover the second-generation
paradigm; no implementation. Two new documents; zero code changes; V1 explicitly retained.

### Done
- **[COGNITIVE_ARCHITECTURE.md](COGNITIVE_ARCHITECTURE.md)** — the project's intellectual
  foundation. Core moves:
  1. *Hostile V1 review*: V1 is governance without epistemology — it proves what the system
     did but has no theory of why it believes anything; asserted confidence + inert provenance
     ⇒ memory must rot (V1's own top risk, now explained rather than just named). The trust
     ledger is a special case V1 failed to generalize.
  2. *The paradigm*: an **institution of one** — not an artificial mind; a record-keeping,
     score-keeping institution that hires foundation models like consultants. One governing
     principle: **confidence is never asserted, only earned by scoring** — one universal track
     record over beliefs, memories, skills, mental models, goals, routing, and the models
     themselves.
  3. *Memory theory*: four strata (episodes → bets → runnable models → policies);
     decision-theoretic retention; **bets, not facts** (no falsifiers → no admission to relied-
     upon memory; stakes reuse the R0–R3 algebra); justification-based truth maintenance
     (retraction propagates; plans carry premises and auto-invalidate).
  4. *Ledgered deliberation*: no agent loops — 12 typed operators over the record; every
     judgment records a **context manifest** (exactly which memories were in context), enabling
     failure attribution, per-memory credit assignment, and model-swap regression replay.
  5. *Learning without retraining*: the compilation cascade (deliberate → procedure → reflex,
     score-gated); the **working-set compiler** W(record, intent, budget) as the single
     highest-leverage learnable function (personal AI as a memory-hierarchy problem).
  6. *Goals as mandates*: the capability algebra extended to purpose and attention; mandatory
     recommitment cadences kill zombie goals structurally.
  7. *Assumption audit*: chat, prompt engineering, vector DBs, RAG, summaries, agent loops,
     tool calling, orchestration, planning, knowledge graphs — each with a verdict and a
     replacement (several were already replaced by V1; noted rather than re-invented).
  8. *2035 answer*: the universal track record — the only mechanism in the design space that
     makes a system MORE trustworthy as it ages.
- **[RESEARCH_NOTES.md](RESEARCH_NOTES.md)** — prior-art honesty (TMS, ACT-R/SOAR, forecasting
  epistemology, ocap, Engelbart, CYC-as-negative-lesson); 9 rejected alternatives with reasons
  (incl. fine-tuning, vector-DB-as-memory, long-context maximalism, RL-over-user, Bayesian
  networks deferred-with-trigger); 4 accepted trade-offs; **7 open problems stated plainly**
  (scoring generative work, dependency explosion, cold-start calibration, self-referential
  gaming, user-model ethics, structure-earning trigger, reasoner capture of the record);
  6 falsifiable experiments queued (E-1 calibration-weighted retrieval is first).
- ARCHITECTURE.md header now places the theory layer in the document hierarchy.

### Implementation deltas implied (additive, staged — NOT started)
Per COGNITIVE_ARCHITECTURE Part X: specs/record.md 0.2 (bet schema, derivation events with
manifests), specs/blackboard.md 0.2 (operator taxonomy), SCORE/RECONCILE as Alpha deliverables
ahead of consolidation (they police exactly the Alpha gate's risk), router outcome events gain
(operator, domain, stakes) dimensions. V1 code untouched by design.

### Next actions
1. Draft specs 0.2 (bet schema + derivation events) — the first mechanical consequence of V2.
2. Run E-1 (calibration-weighted vs. similarity retrieval) as soon as retrieval v1 exists.
3. Continue Phase 1 MVP work (Incident artifacts, retrieval v1) — now with V2 shapes in mind:
   Incidents should be born as premortem bets, retrieval as W's first draft.

---

## 2026-07-04 (session 7) — Provider-agnostic router + sealed keys + onboarding (D-017)

**Phase:** Phase 1 (MVP). User directive: no tight coupling to Anthropic; one interface for all
providers; onboarding prompt; keys stored securely, never hardcoded.

### Done
- **`runtime::providers`** — the `Provider` trait + registry: `anthropic`, `openai`, `gemini`,
  `openrouter`, `ollama`, `lmstudio`, `openai-compat` (any compatible endpoint via `base_url`),
  `mock`. Three wire protocols cover all of them; the OpenAI chat-completions shape is written
  once and parameterized (it's the de facto standard → future providers land config-only).
  Vendor switching = `router.json` edit; application logic (pipeline/inbox/blackboard) untouched.
- **`substrate::secrets`** — sealed secrets store: per-secret XChaCha20-Poly1305 envelopes under
  the user's KEK (AAD = purpose label), same root of trust as the record. API keys are never
  plaintext on disk, never in config, never in the log, never in code.
- **Router rework**: key resolution = env-var override (operators/CI) → sealed secret → refuse
  with guidance. Default route table is **`unconfigured` on purpose** — a fresh record refuses
  `nx do` with "run `nx configure`" rather than silently routing to a vendor the user never
  chose. Old `backend` config field still loads (serde alias).
- **Onboarding**: `nx init` prompts provider/model/key in a terminal (masked key input via
  rpassword; `--no-configure` to skip); `nx configure` for changes — interactive or flag-driven
  (`--provider --model --api-key-stdin --base-url`); `--api-key` works but warns about shell
  history.
- **Verification:** 39/39 tests (12 new: registry completeness, key-requirement matrix,
  compat-without-base-url refusal, secrets roundtrip + wrong-KEK refusal + no-plaintext-on-disk,
  unconfigured-demands-onboarding, sealed-secret resolution, old-config migration, onboarding
  validation matrix). Live demo: unconfigured refusal → configure openai with stdin key →
  router.json carries provider only, key sealed in `keyring/secrets.json` (verified ciphertext) →
  switch to ollama config-only → loop still runs.
- **Docs:** D-017 in DECISIONS.md; ARCHITECTURE §7 updated.

### Notes
- Request shape is lowest-common-denominator (system + user + max_tokens) — vendor-specific
  features (tool use, caching) wait for capability flags on the trait (D-017 revisit condition).
- Suggested default models (`gpt-5.2`, `claude-sonnet-5`, …) are onboarding *suggestions*,
  user-editable; they will go stale and should be refreshed periodically.

---

## 2026-07-04 (session 6) — Phase 1 begins: the first agent loop runs (`nx do` → inbox → approve)

**Phase:** Phase 1 (MVP). A model now enters the system — through the router, never around it.

### Done — new `runtime` crate (specs/blackboard.md v0.1 in code)
- **`runtime::blackboard`**: artifacts as `bb.artifact/1` events, transitions as
  `bb.transition/1`; current view folded from the log. **Contracts enforced at write time**
  (a veto Critique with zero issues is rejected; every issue must cite a target; a Diff must
  carry `effect_list`; a Finding must carry evidence). **Transition authority is role-scoped** —
  the critic *cannot* approve a Diff; only the user can (D-015 in the type system).
- **`runtime::charter`**: steward/worker/critic as versioned JSON under `<data>/charters/`,
  written once, never overwritten — user-editable, eval-gating later (D-010, D-012).
- **`runtime::router`**: static policy table (`router.json`) mapping task class → backend/model
  (D-016). Backends: `anthropic` (ANTHROPIC_API_KEY) and `mock` (deterministic, offline).
  **Every call logs an outcome event** — task class, model, latency, output hash; never prompt
  or completion content. The learning flywheel's intake exists from call one.
- **`runtime::pipeline`** — the loop: Steward preserves the intent verbatim → Worker gets a
  bounded target snapshot (≤50 files, ≤64 KB each), returns contracted JSON (plan + write/delete
  operations) → operations apply **inside a workspace transaction** with model-supplied paths
  confined (`..`/absolute refused) → Diff artifact carries the hash-truthful effect list with
  the plan as advisory only → Critic adversarially reviews effects + resulting contents →
  Critique artifact (contract-checked) → intent parked in `review` for the user.
- **`runtime::inbox`**: `nx inbox` lists pending Diffs with intent, effects, verdict, issues;
  `nx approve <txn>` commits (binding to the effect list); `nx reject <txn> --reason` aborts —
  **a reason is mandatory: rejections are training signal** (ARCHITECTURE §12).
- **CLI**: `nx do "<intent>" --target <dir> [--mock]`, `nx inbox`, `nx approve`, `nx reject`.

### Verification
- 30/30 tests (9 new: full mock loop approve-path with target-untouched-before-approval,
  reject-path with reason-required, contract enforcement, authority scoping, router outcome
  logging without content, path confinement, fence-stripping).
- Live CLI demo: `nx do` staged 1 effect with critic approval; target untouched until
  `nx approve`; inbox drained after; record shows the full trail (2 router calls, 4 artifacts,
  4 transitions) — all queryable via the derived index.

### Notes
- No ANTHROPIC_API_KEY in this environment — live-model runs need the user to set it (then
  `nx do` without `--mock` routes to `claude-sonnet-5` per default `router.json`).
- Deviations to revisit: Critic reviews only the Diff (not the Plan pre-execution — spec wants
  both; v0 matches "review the effect list"); veto does not auto-abort (user decides in inbox);
  blackboard view folds from full log scan (materialize into derived store when it gets slow).

### Next actions
1. Live-model run once a key is present; tune worker/critic prompts on real output.
2. `Incident` artifacts on abort/reject → the outcome-labeling loop (ARCHITECTURE §12).
3. Retrieval v1 (temporal + lexical + vector over the record) so the Worker gets memory, not
   just a directory snapshot.
4. IDE/inbox surface beyond the CLI (Tauri) once the loop earns real usage.

---

## 2026-07-04 (session 5) — Derived store, signed ledger, and the Phase 0 gate: PASSED

**Phase:** Phase 0 (kernel spike) — **gate passed**; Phase 1 (MVP) is next.

### Done
- **`substrate::derived` — SQLite index + `nx rebuild`** (D-013 escape hatch): drop-and-re-derive
  the entire index from the log inside SQLite (not by deleting the db file — Windows can't delete
  open files, and rebuild must work under a live reader). Indexes event *headers only*; bodies
  deliberately not indexed — FTS over decrypted content would put plaintext at rest outside the
  envelope model, so it waits for a derived-store encryption story (Phase 2). `nx query` filters
  by schema/trust/kind/references; `refs` table gives provenance-edge lookups.
- **`substrate::identity` + `kernel::ledger` — Ed25519 ledger signing**: daemon signing key
  generated lazily, sealed under the user's KEK (one root of trust: the passphrase). All kernel
  ledger events (`txn.begin/commit/abort`, `ledger.effect/1`) signed over canonical
  sorted-key JSON; signer public key rides in each entry so a record audits from its own
  contents. `nx ledger --verify` — exits nonzero on any bad signature.
- **Bug found by test, fixed in product:** rebuild-by-file-deletion failed on Windows with a
  live connection (os error 32) → rebuild now happens inside SQLite. The test caught a real
  future-inbox-UI bug, not a test artifact.
- **Verification:** 21/21 tests (new: derived rebuild reflects forgetting + refs, rebuild
  idempotency, identity sign/verify/persistence, wrong-KEK refusal, ledger tamper detection).

### Phase 0 gate run (ROADMAP.md Phase 0) — PASSED
Scripted multi-file refactor (rename across 3 source files + new changelog) through `nx txn`:
1. **Abort losslessness:** destructive workspace edits + abort → target tree hash bit-identical.
2. **Truthful effect list + clean commit:** 4 effects (1 create, 3 modify) with before/after
   hashes; rename verified applied across all code files. (Gate script initially reported FAIL —
   its grep matched the changelog *describing* the rename; the refactor itself was complete.)
3. **Signed ledger verifies:** every entry `sig:OK`; tamper → nonzero exit.
4. **Rebuild idempotency:** two rebuilds from the log → identical stats (8 events, 4 schemas).

### Next actions (Phase 1 — MVP per ROADMAP.md)
1. Charter data model + blackboard store (artifacts as `bb.artifact/1` events per
   specs/blackboard.md) — the runtime the four roles execute on.
2. Model router v0: static policy table + outcome logging.
3. First agent loop: Steward accepts an Intent → Worker plans/executes inside `nx txn` →
   Critic reviews the effect list → Delegation Inbox (CLI-first: `nx inbox`).

---

## 2026-07-04 (session 4) — Workspace transactions + effects ledger: the D-005 core works

**Phase:** Phase 0 (kernel spike) — the revolutionary capability's first working form.

### Done
- **`kernel::txn` — workspace transactions over the filesystem** (D-005, ARCHITECTURE §5):
  - `begin` canonicalizes the target, snapshots it (BLAKE3 manifest + full copy; lazy CoW is a
    later optimization behind this interface), and mints a task-scoped **R1 capability** with
    24 h expiry, persisted in the txn meta.
  - `diff` computes the **truthful effect list** — Created/Modified/Deleted with before/after
    hashes, workspace vs. snapshot — plus **target drift** detection (world moved under the txn).
  - `commit` refuses on drift; otherwise applies each change only after an exercise-time
    capability check (op ∈ grant, scope, R1 ≤ limit, not expired/revoked) and appends a
    `ledger.effect/1` event per change, then `txn.commit/1`.
  - `abort` discards the workspace — the target was never touched, so abort is free by
    construction, not by restore.
  - Skips `.nexus` and `.git` from snapshots (the record never transacts over itself).
- **CLI**: `nx txn begin|list|diff|commit|abort`, `nx ledger` (decrypted txn + effect events).
- **Verification:** 16/16 tests (4 new: begin→diff→commit roundtrip with ledger counts;
  drift-refusal; abort-leaves-target-untouched; expired-capability-blocks-commit). Live demo:
  3-change speculative edit committed with hash-accurate effect list and 5 ledger events;
  sabotage edit aborted with target intact; concurrent outside edit correctly refused with
  "target drifted"; full ledger decrypts and reads clean.

### Dev-grade shortcuts (tracked)
- Snapshot = full copy (fine at personal scale; CoW later). Effect application is not atomic
  across files (single-writer personal scale; journal-and-replay hardening later).
- Windows `\\?\` canonical-path prefix leaks into display output — cosmetic, fix with the inbox UI.
- Capability scope check is target-root granularity (per-adapter scope grammar is spec 0.2).

### Next actions
1. First derived store (SQLite) + `nx rebuild` — completes the Phase 0 component set.
2. Ed25519 ledger signing with a daemon identity key.
3. Phase 0 gate run: scripted model-driven multi-file refactor through `nx txn`, then
   commit/abort round-trip losslessness check.

---

## 2026-07-04 (session 3) — Toolchain installed; Phase 0 walking skeleton builds, tests pass, runs

**Phase:** Phase 0 (kernel spike) — first working code. **Toolchain:** Rust 1.96.1
(stable-x86_64-pc-windows-msvc via `winget install Rustlang.Rustup`, user-authorized).

### Done
- **Cargo workspace** — `crates/substrate`, `crates/kernel`, `crates/nx` (+ workspace
  `Cargo.toml`, `.gitignore`). Boring-storage/radical-semantics stance held: files + JSONL for
  v0 stores, no SQLite yet (arrives with derived stores and `nx rebuild`).
- **`substrate` crate** (implements specs/record.md §2–§7):
  - Event schema with trust/privacy/origin; ULID ids; BLAKE3 `body_hash`.
  - XChaCha20-Poly1305 per-item DEKs, AAD = header bytes (header↔body binding); DEKs wrapped by
    Argon2id-derived KEK with AAD = event id (no wrapped-key replay across events).
  - Keyring stored separately from segments; **forget = shred wrapped DEK + rewrite keyring +
    append `memory.forget/1` tombstone** (id + body_hash only, never content).
  - Append-only segments: `NXL1` magic, `u32 len ‖ frame ‖ BLAKE3` frames, 64 MiB roll,
    fsync-per-append (batched-fsync optimization deferred), torn-final-frame recovery,
    hard failure on checksum mismatch. Content address = BLAKE3(frame).
  - Write order: keyring before log — an event without a persisted key fails safe.
- **`kernel` crate** (implements specs/capability.md §1–§5, §7): `EffectClass` R0–R3,
  `CapabilityTable` with mint / attenuate (⊆ on every axis) / cascading revoke /
  exercise-time check; **context-trust ceiling** (`external ⇒ max R1`); **R3 has no direct
  execution path** — check refuses it categorically (D-005).
- **`nx` CLI**: `init`, `append`, `log [--decrypt]`, `forget <id>`, `verify`; passphrase via
  flag or `NX_PASSPHRASE`.
- **Verification:** 12/12 unit tests pass (crypto roundtrip + AAD binding, wrap binding,
  wrong-passphrase refusal, forget-shred-tombstone persistence, tamper detection, capability
  grant/deny, no-broadening attenuation, cascade revoke, external-context R1 cap, R3 refusal,
  expiry-at-exercise). End-to-end CLI run: init → append (user + external trust) → decrypt-log
  → forget → `[FORGOTTEN]` + tombstone → wrong passphrase refused → checksums verified.
- One bug found and fixed during testing: test fixture computed expiry per call, so children's
  expiry landed microseconds past the parent's — the no-extend-expiry rule caught it (the rule
  working as designed; fixture now shares one timestamp).

### Deviations / dev-grade shortcuts (tracked, deliberate)
- `anyhow` in library crates — acceptable for the skeleton; typed errors before Phase 1.
- Keyring is JSONL rewritten on forget — fine at personal scale; migrates behind the `Keyring`
  interface. **Keyring is primary data (not derived): back it up.**
- CLI passphrase via env/flag — interactive prompt + OS keychain is Phase 1.
- Ledger signing (Ed25519) deferred to daemon bring-up (needs a daemon identity key).

### Next actions
1. Workspace transactions over the fs adapter (copy-on-write dir, effect list, `nx diff` /
   `nx commit` / `nx abort`) — the heart of the Phase 0 gate.
2. Effects ledger as log events (`ledger.effect/1`) wired through `CapabilityTable::check`.
3. `nx rebuild` once the first derived store (SQLite) lands.
4. Then the Phase 0 gate run: scripted model-driven multi-file refactor, truthful effect list,
   clean commit/abort round-trips.

---

## 2026-07-04 (session 2) — Interruption report verified false; specs v0.1 written

**Phase:** Phase 0 (kernel spike) — spec work started. **Code written:** none yet (see blocker).

### Verification note (important for future sessions)
A resume message claimed the documentation files were "left empty by interruption." **Verified
false on disk** before acting: all five documents were fully populated (≈85 KB total) and intact.
No file was rewritten. Lesson recorded: on any interruption/resume claim, verify file state
(`wc -c`, tails) before overwriting — never trust the report over the disk.

### Done
- Appended `STATUS: COMPLETE` markers to DESIGN.md, ARCHITECTURE.md, DECISIONS.md, ROADMAP.md
  (and this file, below) per the resume instruction's completeness convention.
- **[specs/record.md](specs/record.md) v0.1** — event schema, origin/trust tags, privacy classes
  P0–P3, XChaCha20-Poly1305 envelope with per-item DEKs wrapped by user KEK (crypto-shredding per
  D-011), segment/frame format with content addressing (BLAKE3 over frame = sync identity;
  `body_hash` = plaintext identity), normative forgetting behavior, claim schema, export/import
  round-trip guarantee.
- **[specs/capability.md](specs/capability.md) v0.1** — the load-bearing **effect-class taxonomy
  R0–R3** (Observe / Reversible / Compensable / Irreversible; closed set; R2 without a registered
  compensator degrades to R3), capability structure, default-deny minting, attenuation-only
  delegation, the **capability-ceiling table** implementing D-014 (external context ⇒ max R1, no
  R3 queueing, workspace-only writes; sanctioned partitioning pattern documented), signed ledger
  entry format binding approval to effects (D-015), synchronous revocation + `nx stop --all`.
- **[specs/blackboard.md](specs/blackboard.md) v0.1** — artifact envelope over log events, the 8
  artifact types with *enforced* contracts (Finding requires evidence; Diff's approval object is
  the ledger effect list; Critique must cite lines), lifecycles + role-scoped transition
  authority, charter format (roles as data, D-012), wake/lease/checkpoint scheduling semantics,
  deadlock→DecisionMemo rule, conformance criteria.
- Each spec carries an "open questions" section feeding v0.2 — nothing silently unresolved.

### Blocker
- **No Rust toolchain on this machine** (`cargo`/`rustc` not found). Phase 0 scaffold
  (ARCHITECTURE §3, ROADMAP Phase 0) cannot be compiled until rustup is installed. Deliberately
  not auto-installed — system-level change awaiting user go-ahead. Install:
  `winget install Rustlang.Rustup` then `rustup default stable`.

### Open questions / known gaps (carried + new)
- Duress/partitioning design (V1 blocker, DESIGN.md 12.3) — owner needed before Phase 4.
- Backup story under crypto-shredding — partially addressed in record.md §7 (ciphertext and
  keyring stored separately); full backup/rotation design in Phase 2.
- ~~Effect-class taxonomy~~ — **done** (capability.md §1).
- Personal eval sets for subjective tasks — open research; collect labeled outcomes from MVP day one.
- Capability-minting UX — prototype during Phase 1 inbox work.
- Spec 0.2 items: keyring rotation, large-blob sidecar store, deterministic-CBOR profile pin,
  R2 compensator conformance harness, scope grammar, R3 queue-entry expiry, cross-task artifact
  trust propagation.

### Next actions (in order)
1. **User decision:** install Rust toolchain (command above) — unblocks everything below.
2. Scaffold the cargo workspace: crates `substrate` (log, keyring, derived stores), `kernel`
   (capabilities, transactions, ledger), `adapters` (fs/shell/git as WASM), `nx` (CLI).
3. Build Phase 0 walking skeleton against the ROADMAP Phase 0 gate: append/read encrypted log,
   `nx rebuild`, capability mint/attenuate/revoke, fs workspace transaction with truthful effect
   list, `nx diff`/`nx commit`/`nx abort` round-trip.
4. Wire spec conformance checks into the test suite from the first commit (blackboard.md §6).

### Document map
| File | Role |
|---|---|
| [DESIGN.md](DESIGN.md) | Vision, rationale, first-principles argument, self-critique (non-normative) |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Normative architecture v1.1 — wins on conflict |
| [DECISIONS.md](DECISIONS.md) | ADR log D-001…D-016 |
| [ROADMAP.md](ROADMAP.md) | Phases, gates, risk register |
| [specs/record.md](specs/record.md) | Sacred interface #1: record format (v0.1) |
| [specs/capability.md](specs/capability.md) | Sacred interface #2: capability model + effect classes (v0.1) |
| [specs/blackboard.md](specs/blackboard.md) | Sacred interface #3: agent/blackboard protocol (v0.1) |
| PROGRESS.md | This file |

---

## 2026-07-04 (session 1) — Architecture review v1.0 → v1.1; documentation set established

**Phase:** pre-implementation. **Code written:** none, by design.

### Done
- **DESIGN.md** — full vision/architecture manifesto (Parts 0–12): premise interrogation,
  principles, the ten core ideas, complete subsystem design, memory architecture, agent society,
  interaction model, self-improvement, tech choices, roadmap, self-critique. Retained as the
  *rationale* document; no longer normative.
- **Principal-Architect review of v1.0.** Findings that forced changes:
  1. *Unsound security claim* — taint/instruction-source detection through a model is
     undecidable → replaced with enforceable capability ceilings (D-014).
  2. *Approval soundness hole* — users were approving model-written semantic diffs → approval
     now binds to the kernel's raw effect list; summaries advisory only (D-015).
  3. *Premature complexity* — 11-agent society → 4 roles with charters-as-data (D-012);
     CRDT multi-device sync, ambient sensors, and trusted consolidation deferred behind explicit
     gates (D-013); learned routing deferred until outcome data exists (D-016).
  4. *Missing escape hatch* — added `nx rebuild` (derived stores from log) as a day-one supported
     operation, making every derived-store schema decision reversible.
- **ARCHITECTURE.md** — normative v1.1 architecture (wins over DESIGN.md on conflict). Defines
  the three sacred interfaces (record format, capability model, blackboard protocol) as the
  10-year stability contract; everything else declared disposable.
- **DECISIONS.md** — 16 ADRs (D-001…D-016) with context, tradeoffs, and revisit conditions.
- **ROADMAP.md** — 6 phases, each with a named risk retired and a measurable gate; cross-phase
  risk register with kill criteria. Alpha is the designated stall point (consolidation research risk).

---

STATUS: COMPLETE
