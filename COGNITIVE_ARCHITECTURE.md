# NEXUS — Cognitive Architecture (Version 2 Theory Layer)

**Status: intellectual foundation · July 2026.** This document sits *above*
[ARCHITECTURE.md](ARCHITECTURE.md): that file governs what is built; this file governs what the
system *is*. Nothing here discards V1 — the review below concludes that V1 is a correct
special case of a more general theory, and the deltas are additive. Companion:
[RESEARCH_NOTES.md](RESEARCH_NOTES.md) (rejected alternatives, prior art, open problems).

---

## Part I — Hostile Review of Version 1

Version 1 was designed by asking "what makes delegation safe?" It answered with governance:
capabilities, transactions, effect classes, provenance, a trust ledger. Rereading every V1
document as an adversary, three findings survive scrutiny and three do not.

### What V1 got right (and must not be unwound)

1. **The verification-gap analysis** (DESIGN.md §0.3–0.4). Delegation is gated by verification
   cost and unrecoverable harm, not intelligence. Still the best sentence in the project.
2. **The ownership inversion.** The record as the durable center; models as peripherals.
3. **Mediation, provenance, and the append-only record.** The audit substrate is correct.

### Where V1 is intellectually weak

**Finding 1 — V1 has governance without epistemology.** The system can prove what it *did*
(signed ledger) but has no theory of why it *believes* anything. A V1 "claim" is a
subject–predicate–object triple with a confidence float and provenance edges. Interrogate that
confidence number: where does it come from? A model asserted it. What would change it?
Unspecified. What happens to conclusions built on it when it dies? Nothing — V1 has provenance
*edges* but no propagation *semantics*. V1's own top-ranked risk ("confidently-wrong
autobiography," DESIGN.md 12.2) is not an implementation risk; it is the direct consequence of
this missing theory. A memory with asserted confidences and inert provenance **must** rot.

**Finding 2 — the trust ledger is a special case that V1 failed to generalize.** V1's deepest
mechanism is: *autonomy is earned per skill from a track record of outcomes, and contracts on
failure.* That is a calibration system — applied only to actions. V1 never asks why beliefs,
memories, retrieval policies, mental models, and the foundation models themselves shouldn't be
governed by exactly the same mechanism. They should. That generalization is the core of V2.

**Finding 3 — V1's knowledge representation is storage-shaped, not decision-shaped.** Triples
answer "what is true?" A system that *acts* needs its memory to answer "what should I rely on,
here, at what risk?" Facts don't carry reliance semantics. Bets do (Part IV).

### The assumption audit demanded by the brief

| First-generation convention | Verdict | Second-generation replacement |
|---|---|---|
| Chat as primary interaction | **Replaced** (V1 already) | Shared working state; utterances become evidence events in the record; chat survives as debugger |
| Prompt engineering | **Replaced** | Prompts are *compiler output*: assembled from the record by a learned working-set function (Part VI), never hand-authored per task; charters are versioned, eval-gated code (V1) |
| Vector databases | **Demoted** | Similarity is a candidate generator, never an authority; candidates are reranked by calibration and justification status |
| Traditional RAG | **Replaced** | Working-set compilation: planned, multi-index, provenance-tagged, budgeted, with a recorded manifest (Part VI) |
| Memory summaries | **Rejected** (V1 already) | Consolidation emits *bets* (falsifiable positions), never prose summaries; summaries exist only as disposable views |
| Agent loops | **Replaced** | Ledgered deliberation: stateless typed operators over the record; every inference step is an event (Part III) |
| Tool calling | **Replaced** (V1 already) | Capability-mediated effects with declared effect classes; V2 adds per-tool track records |
| Workflow orchestration | **Replaced** | Nothing is authored as a workflow; workflows are *compiled from deliberation traces* that recur (Part VII) |
| Planning systems | **Upgraded** | Plans carry their premises as first-class links; a plan is auto-invalidated when a premise-bet is retracted (Part V.4) |
| Knowledge graphs | **Demoted** | Entity–relation structure survives as an *index*; the knowledge itself is bets and runnable models, not edges (Part IV) |

Nothing above is rewritten for the sake of rewriting: rows marked "V1 already" are retained;
the genuinely new commitments are the epistemic ones, and each is justified below.

---

## Part II — The Paradigm: An Institution of One

### What we are actually building

First-generation systems tried to build an artificial *mind* — a persona that chats, remembers,
and acts. The persona framing fails structurally: minds are supposed to be trusted on vibes,
and no one should trust one with their life. There is an older technology for exactly this
problem — sustaining reliable judgment and action across decades, across the turnover of every
individual member, under audit: the **institution**.

Institutions outlive their members. They keep records. They allocate authority by track record,
not by charisma. They have procedures for revising positions, for dissent, for post-mortems.
They survive the replacement of every human in them — which is precisely the property a
personal intelligence layer needs, because *every* foundation model in it will be replaced,
repeatedly, for decades.

**NEXUS is not an artificial mind. It is an institution of one: a governed, record-keeping,
score-keeping institution whose sole beneficiary is one person, which hires foundation models
the way an institution hires consultants — measured, interchangeable, and never owning the
files.** V1 built this institution's law (capabilities, transactions, audit). V2 builds its
epistemology (how it knows, how it learns, how it keeps itself honest).

This is not a metaphor for marketing. It is a design generator, and every section below is
derived from it: consultants ⇒ per-model scorecards (Part VIII); records ⇒ the strata (Part
IV); procedures ⇒ operators (Part III); track records ⇒ the universal ledger of predictions
(Part II.2); mandates ⇒ goal theory (Part V.3); law ⇒ V1's kernel, unchanged.

### The one governing principle

> **Confidence is never asserted; it is earned by scoring.**

Every entity in the system that influences a decision — a belief, a memory's retrieval
priority, a skill, a mental model, a routing rule, a foundation model, an agent charter — must
carry a **track record**: a history of checkable predictions it licensed and how they resolved.
Influence over future decisions is proportional to that record. No exceptions; anything without
a track record is quarantined as *uncalibrated* and must say so when used.

This single principle is the generalization Finding 2 demanded. The V1 trust ledger becomes one
instance (track records over *actions*). Belief confidence becomes another (track records over
*claims*). Model routing becomes another (track records over *reasoners*). Retrieval tuning
becomes another (track records over *what was worth remembering*). One mechanism, uniformly
applied, is what lets a system compound for a decade without rotting — because everything that
degrades gets *caught by its own score*.

### The second reframing: personal AI is a memory-hierarchy problem

The foundation model is a fixed, rented CPU: enormous, stateless, and unchangeable by us. The
record is the disk: vast, personal, permanent. Between them sits a narrow channel — the context
window — through which everything the system "is" must pass on every single judgment.

Fifty years of systems research says what this is: a **memory hierarchy**, and the interesting
object is the thing that decides what goes in the fast layer. All personal intelligence in a
frozen-weights world lives in exactly one function:

```
W : (record, intent, budget) → working set
```

the **working-set compiler** (Part VI). Not "retrieval" — compilation: it plans, selects,
orders, compresses, tags provenance, enforces privacy tiers and capability ceilings, and emits
a *manifest* of what it chose. Improving W from outcome data is the single highest-leverage
learning channel the architecture has. Long-context models do not abolish W; they change its
budget constant (cost, attention dilution, verification burden, and privacy tiering all still
bind — see RESEARCH_NOTES §R-3).

---

## Part III — The Cognitive Architecture: Ledgered Deliberation

### Cognition as typed operators over the record

There is no "agent loop." There is a **record**, a small set of **typed inference operators**,
and a **scheduler** that decides which operator fires next. Every operator is stateless
(V1 D-006), consumes typed inputs from the record, and appends typed outputs *with a derivation
event* linking outputs to inputs. Cognition is the record rewriting itself under law.

The operator set (each maps onto V1 machinery where it exists):

| Operator | Signature | V1 anchor |
|---|---|---|
| **SENSE** | world → episodes | ingestion/quarantine |
| **APPRAISE** | episodes → candidate bets; contradiction flags | consolidation |
| **ORIENT** | bets × mandates → goal pressure → intents | *(new — Part V.3)* |
| **COMPILE** | (record, intent, budget) → working set + manifest | retrieval planner, generalized |
| **DELIBERATE** | working set → judgment (plan, draft, analysis) via a rented reasoner | Worker/Critic calls |
| **REFLEX** | preconditions → effects via compiled skill, no reasoner | skill execution |
| **STAGE** | plan → speculative effects in a transaction | D-005 |
| **REVIEW** | effect list × intent → verdict | Critic + inbox |
| **COMMIT / COMPENSATE** | verdict → applied effects / rollback | kernel |
| **SCORE** | outcomes → track-record updates for every entity that licensed the decision | *(new — the universal ledger)* |
| **RECONCILE** | contradiction / retraction → belief revision, propagated | *(new — Part IV.4)* |
| **CONDENSE** | recurring deliberation traces → compiled skills | skill ladder, formalized |

### The derivation event — the atom of machine reasoning

Every DELIBERATE and APPRAISE emits:

```
derivation: {
  operator, inputs: [record ids], output: record id,
  reasoner: (model id, version), charter: (name, version),
  manifest: hash of the exact working set compiled for this judgment,
}
```

The **context manifest** is the load-bearing novelty: for every judgment the system ever makes,
it is recorded *exactly which memories were in context*. Consequences: post-mortems can
distinguish "wrong belief" from "right belief, wrong working set" (a failure-attribution
ability first-generation systems categorically lack); SCORE can assign credit and blame to
individual memories (the retrieval policy itself becomes calibratable); and reasoning is
replayable — the same working set can be re-run under a new model to regression-test a
model swap.

### Two processes: deliberation and reflex

Behavior migrates down a **compilation hierarchy** as track records accumulate:

```
DELIBERATE (rented reasoner, expensive, flexible, uncalibrated at first)
   ↓ CONDENSE: recurring trace + accumulated score
PROCEDURE (typed program, reasoner only at declared judgment points)
   ↓ further scoring, narrowing preconditions
REFLEX (deterministic, kernel-checked, no reasoner, near-zero cost)
```

Demotion down the hierarchy is *earned* (score threshold per V1's skill ladder); any incident
promotes the behavior back up and the failure becomes a premortem bet (Part V.5). This is how
the system gets faster, cheaper, and more reliable simultaneously for years, with frozen
weights: intelligence growth = **migration of behavior down the compilation hierarchy**, gated
by the universal track record. (Lineage: ACT-R knowledge compilation, SOAR chunking — see
RESEARCH_NOTES §P-2 — but gated by calibration and bounded by capability law, which the
classical architectures lacked.)

---

## Part IV — A Theory of Long-Term Machine Memory

### What should be remembered? The decision-theoretic answer

A system that remembers in order to *act* should retain items by expected decision value:

```
retention value ≈ P(future retrieval) × E[influence on choice] × cost(being wrong without it)
```

Episodes are exempt (they are evidence, kept per V1's crypto-shredding law — the audit
substrate is not a cache). Everything derived is a managed **portfolio of positions**, and
forgetting is portfolio management: positions that stop paying (never retrieved, or retrieved
and then implicated in failures) are decayed by their own scores. "What should the system
remember?" thus has a non-arbitrary answer for the first time: *whatever its track record says
has been worth remembering.*

### The four strata

```
S0 EPISODES   what happened      immutable, provenance-complete, crypto-shreddable   (V1, unchanged)
S1 BETS       what we rely on    falsifiable positions with stakes and scores        (replaces "claims")
S2 MODELS     what we can run    runnable simulations with calibration records       (new)
S3 POLICIES   how we behave      compiled dispositions: skills, routing, attention   (V1 skills, generalized)
```

Consolidation is **upward compilation with scoring gates** (S0→S1: APPRAISE; S1→S2: model
induction; S2→S3: CONDENSE). Correction is **downward propagation** (a retracted bet flags the
models and policies compiled from it). Confidence at every stratum is computed from scoring
history, never asserted.

### S1 — Bets, not facts

A belief the system may *rely on* must be shaped like a position someone could lose:

```
bet: {
  statement,                    // the position
  scope,                        // where it applies (context predicate — beliefs are local)
  horizon,                      // when it should be re-earned or expire
  falsifiers: [observable conditions that would kill it],   // REQUIRED
  stakes: consequence class if acted on and wrong (reuses V1's R0–R3 taxonomy),
  provenance: [episode ids],    // V1, unchanged
  score: {n, brier-style record, last_resolved},             // earned, never asserted
  status: live | dormant | retracted | superseded
}
```

Three commitments distinguish this from every knowledge-graph lineage:

1. **No falsifiers, no bet.** A statement that cannot say what would kill it may be stored as
   an episode but can never enter a working set as something to rely on. This is Popper as an
   admission rule, and it is the structural cure for confidently-wrong autobiography: unkillable
   beliefs are exactly the ones that rot.
2. **Stakes gate reliance.** Acting on a bet requires its score history to clear a bar set by
   the effect class of the action — the same R0–R3 algebra that governs the kernel. A
   low-scored bet may inform an R0 read; it may not license an R3 send. Epistemology and
   authority meet in one taxonomy.
3. **Scores come from resolutions.** Falsifiers and horizons make bets *resolvable*; SCORE
   settles them against subsequent episodes. Confidence is the statistic of those settlements.

### S1 semantics — truth maintenance, revived

Provenance edges plus derivation events form a justification graph, which finally gets
*semantics* (Finding 1): when a bet is retracted — falsified, horizon-expired, or its evidence
crypto-shredded — RECONCILE walks the graph and marks every dependent bet, plan premise, and
compiled skill **unjustified**, queueing re-derivation or demotion. This is justification-based
truth maintenance (Doyle 1979) applied at LLM-era granularity: the reasoner is rented, but the
*justification structure* is ours, permanent, and mechanical. First-generation memory forgets a
fact and keeps every conclusion drawn from it; this architecture cannot.

### S2 — Mental models: knowledge that runs

Some knowledge is not a set of positions but a *simulator*: "how my manager reacts to
surprises," "how this codebase deploys," "what my Thursdays look like." S2 entries are small
runnable predictors — typically a versioned prompt-program with declared inputs/outputs, or
compiled code — whose every prediction is a bet that SCORE settles. A mental model is exactly
as trustworthy as its prediction record, and the system can *say so numerically*.

The **user model is the flagship S2 entry**, and it enjoys a property no other component has: a
continuous, honest, free scoring stream. Every inbox decision — approve, reject-with-reason,
edit-then-approve — resolves predictions the user model made ("she will reject diffs touching
CI config"). The user model therefore becomes the *best-calibrated* component in the system,
which is the correct place for maximum calibration to live. It is also inspectable and
contestable: the user can read every bet the system holds about them, with its evidence and its
score — the "why do you believe this about me?" guarantee, now with a number attached.

### Plans carry premises

A plan is a conditional structure: *given* these bets, these steps reach this goal. V2 makes
the premises first-class links. When RECONCILE retracts a premise, every open plan depending on
it is flagged stale before execution resumes — the standing failure mode of long-horizon agents
(acting on a world model that silently expired) becomes a mechanical impossibility rather than
a hoped-against outcome.

### Failures are memory

Every incident compiles into a **premortem bet**: "tasks shaped like X fail when Y" — scoped,
falsifiable, scored like any other bet, and *retrievable into working sets* for future tasks
that match its scope. Institutions that survive are the ones whose post-mortems change future
behavior; this is the mechanism form of that maxim.

---

## Part V — Goals: Mandates, Not Tasks

Long-horizon goals fail in first-generation systems because they are represented as tasks
(too brittle) or prompt text (too vague). V2 represents a goal as a **mandate** — the same
capability algebra that governs action, now governing *purpose and attention*:

```
mandate: {
  desired_state: predicate over the record,     // satisfaction is checkable, not vibes
  scope, provenance (why this goal — user-stated, always),
  authority: attention budget + autonomy ceilings it may spend,
  review_cadence, expiry,                       // NO IMMORTAL MANDATES
  parent: optional (mandates form a justification tree, like bets)
}
```

- **ORIENT** computes *goal pressure* — divergence between the record's current state and each
  mandate's predicate, weighted by deadline and authority — and converts sustained pressure
  into candidate intents. Goals act as standing fields, not queued tasks; this is how "care
  about my health" or "ship the book" persists across months without living in any context
  window.
- **Recommitment is mandatory.** At each review cadence the mandate must be re-affirmed by the
  user or it demotes to dormant. This is drift-control by design: the system may never pursue a
  goal the *present* user hasn't recently endorsed. (The "still optimizing for 2026-you" failure
  is an expiry bug, and mandates make expiry structural.)
- Mandate conflicts produce DecisionMemos (V1), never silent arbitration.

One algebra now governs the institution's three scarce resources: **action** (capabilities),
**attention** (budgets), **purpose** (mandates). All three are granted, attenuable, expiring,
and auditable.

---

## Part VI — The Working-Set Compiler

W(record, intent, budget) assembles every context the rented reasoner sees:

1. **Plan** — decompose the intent's information needs (V1's retrieval planner, generalized).
2. **Generate** — candidates from all indexes: temporal, entity, lexical, similarity, *and*
   premortem bets in scope, *and* mandate context. Similarity proposes; it never decides.
3. **Filter by law** — privacy tiers, capability ceilings, taint tags (V1, unchanged).
4. **Rank by earned confidence** — a bet's score, freshness, and justification status
   (unjustified items enter only with an explicit flag); an uncalibrated item must be labeled so.
5. **Fit the budget** — compress losslessly where possible (structure, not summaries), cite
   pointers for the rest.
6. **Emit the manifest** — the exact selection, hashed into the derivation event.

W is the system's single most improvable function, and it improves by *credit assignment
through manifests*: when SCORE settles an outcome, items in the manifest share the credit or
blame; retrieval policy updates from that signal (which items earn their context slots is
itself a set of bets). Prompt engineering does not survive contact with this design — nobody
hand-writes what W assembles; humans write *charters* (rare, versioned, eval-gated) and W does
the rest.

---

## Part VII — Workflows Are Learned, Never Authored

No orchestration DSL, no workflow builder, no hand-drawn DAGs. The pipeline is: deliberation
traces (fully recorded, with manifests) → recurrence detection over trace structure → CONDENSE
drafts a procedure with declared preconditions and judgment points → supervised runs inside
transactions → scored promotion toward reflex (Part III). The user experiences this as the
system "picking up how I do things"; mechanically it is trace compilation with a track-record
gate, and it is *reversible* — any incident demotes the skill and re-opens deliberation. Guards
against ossification (frozen habits outliving their fitness): skills carry horizons like bets
and must re-earn their status on schedule; every skill's preconditions are falsifiers by
construction.

---

## Part VIII — Model Independence as Accountability

Abstraction layers (V1's provider registry, D-017) make models *swappable*; they do not make
the system *safe to swap*. V2 adds the institutional stance: consultants are measured.

- Every (reasoner, operator, domain) triple carries a track record, fed by SCORE like
  everything else. Routing (D-016's learned phase) becomes: allocate judgment to the reasoner
  with the best record for this operator in this domain at this stake level, under this budget.
- **Model swaps are regression-tested against the record**: replay a sample of stored working
  sets (manifests make this exact) through the candidate model and diff its judgments against
  settled outcomes before granting it authority. A new model earns its way in on evidence — it
  is never simply *believed* to be better.
- Nothing user-durable may live in any vendor artifact (V1 Principle 5, unchanged). The
  institution survives the consultants; that is what institutions are *for*.

---

## Part IX — The Agent Question, Settled

The brief asks for specialized cognitive agents "without unnecessary complexity." The answer V2
gives: **there are no agents; there are operators and policies.** What V1 called Steward,
Worker, Critic, Archivist are scheduling policies over the operator set — which operator may
fire, on what inputs, under which charter and capability profile. That is all a "role" ever
needed to be. The blackboard's typed-artifact law (V1 D-007) is retained as the operator I/O
discipline. Adding a "new agent" is adding a row of policy, not a new mind — complexity stays
linear in policies, not quadratic in personalities. (V1's D-012 intuited this; V2 states it as
theory: *the society-of-agents metaphor is scaffolding, and operators-over-a-ledger is the
load-bearing structure.*)

---

## Part X — What Changes in the Implementation (Additive Deltas Only)

V1 code survives untouched; the deltas are spec-level and staged:

1. **specs/record.md 0.2** — claim schema → bet schema (falsifiers, scope, horizon, stakes,
   score block); derivation events with context manifests; premortem-bet schema.
2. **specs/blackboard.md 0.2** — operator taxonomy replaces role-first framing (roles remain as
   policy rows); plan artifacts gain premise links.
3. **SCORE and RECONCILE** enter the roadmap as the Alpha-phase deliverables *ahead of*
   consolidation quality — because the Alpha gate's own risk (claim quality) is exactly what
   scoring and truth maintenance exist to police. The gate's metric improves from "% of claims
   the user judges correct" to "calibration curve of settled bets," which is measurable without
   user labeling effort.
4. **Router outcome events** gain (operator, domain, stakes) dimensions to seed per-reasoner
   scorecards.
5. Everything else — kernel, transactions, effect classes, ledger, providers, inbox — is
   already the correct substrate for this theory, which is the strongest evidence V1 was built
   in the right order: *law first, epistemology second* is how durable institutions are founded.

---

## Part XI — The 2035 Answer

*"If this project became the foundation for personal AI systems in 2035, which single
architectural idea would have been responsible, and why?"*

Not the transactional kernel — competitors copied reversibility within a few years, because it
is visible in the product. Not the memory strata or the working-set compiler — those are
excellent engineering, and excellent engineering gets rediscovered.

The idea that could not be rediscovered by imitation, because it only pays after years of
accumulation, is the governing principle: **confidence is never asserted, only earned — one
universal track record governing every belief, memory, skill, mental model, goal, and foundation
model in the system.**

Every first-generation system trusted something on assertion: the model's confident tone, the
retrieval's similarity score, the summary's fidelity, the workflow's continued fitness. Each of
those is a place where rot enters silently and compounds. A system whose every component is
perpetually settling checkable predictions about its own competence is the only architecture in
this design space that gets *more* trustworthy as it ages — and "more trustworthy as it ages"
is, over a decade, the entire ballgame: it is what let users grant year-scale mandates, which
generated the deep records, which made the working-set compiler unbeatable, which made the
platform the default substrate others built on. The moat was never the data. It was that the
data had been *kept honest*.

V1 said: *make delegation safe before making it smart.* V2 completes the sentence: **make the
system keep score on itself, and it becomes both.**

---

## Part XII — Addendum (July 2026): The Identity Question

*Raised after the v0 loop closed: is this an AI assistant with an epistemic layer, or an
epistemic operating system on which an assistant happens to run?*

### The inventory answers before the philosophy does

Take stock of what actually got built. The "assistant" is a thin pipeline: snapshot files, call
a model, stage effects. Everything load-bearing is domain-general epistemics: an evidence log
with provenance and crypto-shredding; positions with falsifiers, stakes, and earned scores;
truth maintenance; derivation manifests; calibration-weighted retrieval; guarded consolidation.
None of that knows or cares that the current client edits files. The same substrate, unchanged,
serves a decision journal, a research corpus, a forecasting practice, a health log, a team's
shared record. The system was named an *institution* in Part II; the honest reading of the code
is that the institution is the product and task execution is its first tenant.

### The dependency asymmetry — and its one crucial exception

The assistant cannot exist without the epistemics: that was this document's founding argument
(unaccountable memory rots; unaccountable action is never trusted). The epistemics can exist
without the assistant — a falsifier-forced decision journal is valuable with zero automation,
the way Tetlock-style forecasting practice is valuable with zero software.

But one dependency runs the other way, and dropping the assistant would sever it:
**epistemics starves without consequences.** A bet is only as good as its resolution stream.
Manual resolution is attention-expensive and sparse; a record whose positions are never settled
is a diary with extra steps, and asserted-confidence rot returns through the back door. Acting
in the world — staging effects, being approved, being rejected with reasons, having outcomes —
is the highest-volume *honest* settlement stream the system can get. The approve/reject/edit
stream already scores the user model for free; task outcomes will settle predictions for free.

### The resolution

**NEXUS is a personal epistemic institution — for knowledge, beliefs, decisions, evidence, and
reasoning. The assistant is Client #1: subordinated in identity, retained as the institution's
primary evidence pump.** The assistant works for the record; the record does not work for the
assistant.

Why this is the stronger 2035 position: it is the direct restatement of Part XI (the universal
track record is the moat, and the track record IS the epistemic OS); it exits the red-ocean
assistant race that every platform vendor must run and enters a lane none of them can enter
cheaply (honest records only pay after years — exactly what quarterly product cycles cannot
copy); and it holds value even through model winters — the institution's worth grows with use,
not with parameter counts.

The named risk: "epistemic operating system" can curdle into a philosophy toy nobody opens
twice. Mitigations are structural, not rhetorical: every epistemic feature must map to one of
three daily jobs (*remember, decide, delegate*); falsifier-drafting stays automated (the human
vetoes, never authors from scratch); and the roadmap gates on daily-use metrics, with an
explicit kill criterion — if pilots use the assistant and ignore the epistemic surfaces, the
identity recenters (ROADMAP v2 risk register).

Consequences: the epistemic schemas are elevated to a fourth sacred interface; the roadmap is
re-cut into kernel/application/platform tracks with the institution's surfaces (decision
artifacts, calibration reports, reconciliation reviews) promoted from "later" to gating
(D-018, ROADMAP.md v2.0). Everything already built survives unchanged — this is a change in
what the project *is*, executed entirely by addition.

---

STATUS: COMPLETE
