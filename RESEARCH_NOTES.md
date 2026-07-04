# NEXUS — Research Notes

Companion to [COGNITIVE_ARCHITECTURE.md](COGNITIVE_ARCHITECTURE.md). Contents: prior art and
lineage (honesty about what is borrowed), rejected alternatives (and why), open problems we do
not know how to solve, and the experiment queue. A research program is defined as much by what
it refuses as by what it builds.

---

## P — Prior Art and Lineage

Original work should know its ancestors. What V2 borrows, and where it departs:

**P-1 · Truth maintenance systems** (Doyle 1979, JTMS; de Kleer 1986, ATMS). The justification
graph and retraction propagation in Part IV.4 are direct descendants. Departure: classical TMS
assumed a monotonic logical reasoner producing the justifications; ours are produced by a
stochastic rented reasoner, so justifications carry manifests and scores rather than proofs,
and revision triggers re-derivation rather than logical relabeling. TMS died commercially with
expert systems; frozen-weight LLMs are exactly the condition under which it becomes essential
again — nobody else appears to have noticed this.

**P-2 · Cognitive architectures** (ACT-R knowledge compilation, Anderson; SOAR chunking, Laird/
Newell; production systems generally). The compilation hierarchy (deliberate → procedure →
reflex) is their idea, revived. Departure: they were theories *of* the reasoner; we take the
reasoner as an opaque commodity and build the architecture *around* it — the durable structure
is the epistemic state, not the inference engine. Also: their learned productions had no
calibration gates and no authority law; ours cannot act without both.

**P-3 · Forecasting epistemology** (Brier scoring; Tetlock's calibration research; prediction
markets). "Confidence is earned by scoring" is their central lesson, imported as the *admission
rule for machine memory* — which forecasting research never proposed. Falsifiers-as-requirement
is Popper operationalized as a schema constraint.

**P-4 · Object capabilities** (Dennis & Van Horn; Mark Miller's E). V1's law. V2's extension —
one capability algebra spanning action, attention, and purpose (mandates) — appears novel.

**P-5 · Memory-hierarchy and event-sourcing traditions.** The working-set framing is Denning's
working sets applied to cognition; the record is event sourcing applied to a life. Both are
standard systems ideas; their composition with calibration (manifest-based credit assignment to
individual memories) is not standard anywhere we know of.

**P-6 · Engelbart (H-LAM/T) and Hutchins (distributed cognition).** The institution-of-one is
closer to Engelbart's augmentation program than to the assistant tradition: the human plus
their record plus their procedures form the cognitive system; the model is one replaceable
stage in it.

**P-7 · CYC (negative lesson).** Decades of hand-authored ontology produced brittle knowledge
that could not defend itself. The bet schema is the anti-CYC: structure is *earned from use and
scoring*, never authored as schema-first ontology. Where CYC asked "what is true?", we ask
"what has paid?".

---

## R — Rejected Alternatives

**R-1 · Fine-tuning / adapters as personalization.** Rejected (again, and finally). Identity in
weights is: unauditable (no provenance), unforgettable (no crypto-shredding), non-portable
(vendor hostage), and unscoreable at item granularity. Every property the record needs, weights
lack. Local LoRAs may someday serve as *caches* of S3 policies — never as the source of truth.

**R-2 · Vector database as memory.** Rejected as memory; retained as candidate generator.
Similarity is a statement about text, not about reliability. A memory system whose authority
signal is cosine distance will confidently retrieve eloquent falsehoods forever — it has no
mechanism by which being *wrong* makes an item less retrievable. Calibration-weighted rerank
fixes exactly this.

**R-3 · Long-context maximalism ("just put the whole record in").** Rejected on four
independent grounds, any one sufficient: (1) cost scales with context, and the institution runs
thousands of judgments per day; (2) attention dilution is real — burying the load-bearing bet
in a million tokens of diary is how it gets ignored; (3) verification burden: a human auditing
a judgment needs the *actual* working set to be small enough to inspect (manifests must be
reviewable); (4) privacy tiers and capability ceilings *require* selective assembly — "send
everything" is unlawful by V1's own kernel. Long context changes W's budget constant, not its
existence.

**R-4 · Memory summaries (write-time compression).** Rejected in V1; the V2 argument is
sharper: a summary is an *unfalsifiable bet* — it asserts "this is what mattered" with no
falsifiers, no horizon, no scoring rule. It is therefore inadmissible to S1 by the admission
rule. Summaries survive only as disposable rendering.

**R-5 · Reinforcement learning over user interactions.** Rejected for the core loop. Personal
n is tiny; rewards are sparse, delayed, and non-stationary; and reward-hacking a *person* is
the failure mode (the system learning to propose diffs the user rubber-stamps rather than diffs
that are right). SCORE uses proper scoring rules against *resolved outcomes*, not user
approval as reward — approval is one evidence stream for the user model, never the objective.

**R-6 · Agent personas / society-of-minds.** Rejected as architecture (kept as UI garnish where
it helps users). Personalities multiply prompt surface, hide the actual computation, and add
quadratic coordination. Operators + policy rows carry all the load (Part IX). Minsky's Society
of Mind inspired the framing and is respectfully demoted to metaphor.

**R-7 · Ontology-first knowledge graph.** Rejected (see P-7). Entities and relations survive as
one index over bets, grown lazily from what bets actually mention — never as a schema the world
must be forced into.

**R-8 · Bayesian belief networks as the S1 formalism.** Seriously considered; deferred.
Exact structure learning at personal scale with LLM-extracted variables is research-grade
fragile; miscalibrated structure would launder wrongness through mathematics, which is worse
than honest score-keeping. Track-record scoring is the robust 80%; graphical structure can be
earned later where dependencies prove stable. This is a *deferral with a trigger* (see X-6),
not a rejection.

**R-9 · Blockchain/DLT for the ledger.** Rejected without much agony. Single-owner system;
Ed25519 signatures + append-only segments give tamper-evidence; consensus machinery adds cost
and no trust the owner doesn't already have.

---

## T — Load-Bearing Trade-offs (accepted with eyes open)

**T-1 · Falsifier admission rule vs. expressiveness.** Much of what people believe resists
crisp falsifiers ("I want to be a good father"). Resolution: such statements live as mandates
(purpose) or episodes (evidence), not S1 bets — the system may *hold* them without *relying* on
them as predictions. The cost: some genuine knowledge is under-weighted. Accepted: the
alternative (unfalsifiable beliefs steering action) is the rot we exist to prevent.

**T-2 · Scoring overhead vs. judgment throughput.** Every reliance event writes score
bookkeeping. At personal scale this is thousands of small writes a day — fine for the log, but
resolution (settling bets against later episodes) is a real background compute budget.
Accepted; CONDENSE exists precisely to amortize repeated judgments into cheap reflexes.

**T-3 · Recommitment friction vs. drift safety.** Mandate review cadences interrupt the user;
too frequent and goals get rubber-stamped (which poisons the user model's scoring stream), too
rare and drift returns. The cadence itself must be tuned per mandate from attention-budget
data. Accepted as an open tuning problem, not a design flaw.

**T-4 · Conservatism of track-record gating.** Earned-confidence systems are structurally slow
to trust new things — new skills, new models, new beliefs all start quarantined. This is the
correct default for an institution holding real authority, but it means the system will
sometimes be annoyingly slow to adopt a genuinely better model or habit. Mitigation: cheap
sandboxed evaluation (replay against manifests) accelerates earning without granting authority.

---

## X — Open Problems (unsolved, stated plainly)

**X-1 · Scoring rules for non-forecast judgments.** Brier-style scoring needs resolvable
predictions. Much of the Worker's output is *generative* (a draft, a refactor) where "was it
good?" has no natural resolution event. Current best idea: score the *downstream proxies*
(edit distance on approval, incident rate, reversal rate) and accept that generative quality
scoring is weaker than forecast scoring. Genuinely open.

**X-2 · Dependency explosion in the justification graph.** If every judgment links every
manifest item as a premise, retraction cascades touch everything and the graph is noise.
Need a theory of *material* premises (which working-set items were actually load-bearing for
the judgment) — possibly via counterfactual replay (re-run the manifest minus one item, diff
the judgment), which is exact but costly. Open; the manifest makes it *possible*, not cheap.

**X-3 · Cold start / small-n calibration.** A personal system may see five instances of a
decision class per year. Hierarchical priors (population → domain → user) shrink honestly, but
population priors require *someone's* aggregate data, which collides with Principle 1.
Federated calibration-sharing (share scores, never content) is a research direction, not a plan.

**X-4 · Self-referential gaming.** The system routes, retrieves, and compiles based on scores
it also produces. Degenerate equilibria exist (e.g., W learns to include only items whose bets
resolve easily, inflating apparent calibration while dodging hard predictions). Defenses:
scoring rules must be proper; resolution must come from episodes (world-anchored), not from
model self-judgment; periodic adversarial audits (Critic charter) on the scoring pipeline
itself. Partially mitigated, not solved.

**X-5 · The user model's ethics.** A well-calibrated model of a person is a lever on that
person. It lives at privacy tier P3 (never leaves device), is user-inspectable and shreddable —
but the deeper question (should the system *use* its model of your weaknesses, e.g. to time
suggestions when you're persuadable?) is a policy question the architecture must expose to the
user rather than answer silently. Flagged as a V1-blocker-class design item alongside duress
modes.

**X-6 · When does structure get earned?** R-8 deferred graphical models. Trigger condition to
revisit: when settled-bet volume in a domain shows stable conditional dependencies that naive
independent scoring visibly mishandles (miscalibration concentrated in correlated clusters).

**X-7 · Institutional capture by the reasoner.** All bets are extracted, and all falsifier
proposals drafted, by the rented model — a subtle channel by which one vendor's biases could
shape the "vendor-neutral" record for years. Mitigations: multi-model APPRAISE sampling for
high-stakes bets; falsifier quality audits; regression replay across vendors (Part VIII).
Residual risk acknowledged.

---

## E — Experiment Queue (each falsifiable, each cheap enough to actually run)

**E-1 · Calibration-weighted retrieval vs. similarity retrieval.** Personal QA set from a
pilot user's record; measure answer accuracy and (critically) *confidence honesty* of cited
support. Success: calibration-weighting beats cosine top-k on honesty at equal accuracy.
This is the first experiment because it tests the central principle at minimum cost.

**E-2 · Manifest-based failure attribution.** Seed known-stale bets into working sets; verify
post-mortems localize failures to the poisoned items via manifests. Success: >80% correct
attribution. Tests the derivation-event design end to end.

**E-3 · Compilation cascade on recurring tasks.** Take one user's 20 most-repeated task shapes;
measure cost/latency/incident-rate as behaviors migrate deliberate→procedure→reflex over 8
weeks. Success: order-of-magnitude cost drop with flat-or-better incident rate.

**E-4 · Premise-linked plan invalidation.** Long-horizon plans with deliberately expiring
premises; measure stale-execution rate with and without RECONCILE. Success: zero stale
executions with, nonzero without (this one should be a rout; it exists to prove the machinery).

**E-5 · Model-swap regression replay.** Replay 200 stored manifests through a second provider;
diff judgments against settled outcomes. Deliverable: the first evidence-based model-swap
report — the Part VIII stance made operational.

**E-6 · Recommitment cadence tuning.** Vary mandate review cadences; measure rubber-stamp rate
(sub-second approvals) vs. drift incidents. Output: cadence policy, not a paper.

---

## The honest one-line self-assessment

V2's ideas are individually traceable to strong lineages (that is a feature — orphan ideas are
usually wrong). The claim to originality is the *synthesis under one principle*: admission by
falsifiability, influence by earned score, revision by justification propagation, learning by
gated compilation, and one authority algebra over action, attention, and purpose — around a
reasoner treated as a rented, measured commodity. If a future frontier model can regenerate
that synthesis tomorrow, it will be because documents like this one taught it to.

---

STATUS: COMPLETE
