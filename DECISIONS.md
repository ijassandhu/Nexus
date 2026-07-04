# NEXUS — Decision Log

ADR-style. One entry per major decision. Status: **Accepted** unless noted. Newest decisions
from the v1.1 architecture review are D-012 … D-016.

Format: Context → Decision → Tradeoffs accepted → Revisit when.

---

## D-001 — Substrate, not operating system
**Context:** "AI OS" framing invites competing with platform vendors on scheduling/UI, which is
unwinnable, and misidentifies the scarce resources (trust, verified context, attention — not compute).
**Decision:** Build a personal intelligence substrate that inhabits existing OSes: owns the
record, mediates action, hosts agents. Keep exactly one OS property: kernel-style total mediation.
**Tradeoffs:** Less grandiose; dependent on host-OS APIs (adapter treadmill, see D-009).
**Revisit when:** V3 "apps as views" experiments succeed and a deeper OS position earns itself.

## D-002 — The record is the center; models are peripherals
**Context:** Model quality commoditizes (~40%/yr price-performance decay); anything durable
stored in a vendor's weights or memory feature is hostage.
**Decision:** All user-durable value (memory, skills, trust, identity) lives in the substrate.
Every reasoning step goes through a neutral router interface; fine-tuning as personalization is
rejected.
**Tradeoffs:** We forgo vendor-specific capabilities (e.g., a vendor's built-in memory or
cross-session features) and pay an abstraction tax on every model call.
**Revisit when:** Never (core principle). The router *implementation* may specialize freely.

## D-003 — Local-first, user-held keys, exportable formats
**Context:** The record is both the moat and the liability; regulation and user trust both favor
demonstrable ownership. A cloud-primary record makes Nexus a data harvester with extra steps.
**Decision:** Record on-device, encrypted with user-held root keys; documented export; any
relay/sync is E2E and self-hostable.
**Tradeoffs:** Harder multi-device story (deferred, D-013); no server-side compute over the raw
record; recovery-from-lost-key is the user's problem (mitigations: escrow options, later).
**Revisit when:** Never for the default. A user-opt-in encrypted cloud tier may exist alongside.

## D-004 — Object capabilities, not scopes/roles
**Context:** OAuth-style install-time scopes are ambient authority: any confused deputy (e.g.,
prompt-injected agent) inherits everything forever. This is the root of most agent-security fear.
**Decision:** Task-scoped, attenuable, time-boxed, revocable capabilities minted per intent by
the kernel. Plugins/adapters receive capability handles, never open access.
**Tradeoffs:** Real engineering cost on every adapter; capability-minting UX must not become a
permission-prompt hell (mitigated by the trust ledger and sensible intent→capability inference).
**Revisit when:** Never for the model itself; the minting heuristics will evolve constantly.

## D-005 — Transactional execution with an effect boundary
**Context:** The verification gap (delegation ⇔ cheap verification + bounded harm) is the
binding constraint on autonomy, per DESIGN.md §0.3–0.4.
**Decision:** Reversible work runs in copy-on-write workspaces reviewed as diffs. Enumerated
irreversible effect classes have no executable path inside a workspace; they queue at the effect
boundary and fire only at commit under earned authority or explicit approval.
**Tradeoffs:** Highest-value delegations (send/pay/publish) still bottleneck on a human until
trust is earned; adapter authors must implement staging semantics; some app state can't be
snapshotted and must degrade to dry-run or gate-everything modes.
**Revisit when:** V2 compensation framework may relax gating for low-stakes sends with
confidence-calibrated auto-commit.

## D-006 — Agents are stateless; all state in the substrate
**Context:** In-memory agent loops lose plans on crash, can't be audited, and weld the system to
one model runtime.
**Decision:** Agents wake → read blackboard → bounded work → write artifacts → exit. Long tasks
checkpoint at plan-step granularity with leases.
**Tradeoffs:** Latency and cost of re-hydrating context each wake (mitigated by working-set
artifacts); checkpoint discipline is a tax on agent authors.
**Revisit when:** Never for the property; the checkpoint granularity will be tuned.

## D-007 — Typed blackboard artifacts; no inter-agent chat
**Context:** NL agent-to-agent chatter is lossy, unauditable, compounds hallucination, and
creates hidden coupling between role prompts.
**Decision:** Agents communicate only via schema'd artifacts (Intent, Plan, Finding, Diff,
Critique, DecisionMemo, SkillProposal, Incident) with contracts (e.g., a Critique must cite
disputed lines).
**Tradeoffs:** Schema evolution overhead; some genuinely conversational coordination becomes
stilted; contracts must be enforced or they rot.
**Revisit when:** Artifact schemas will evolve (spec-versioned); the no-chat rule stands.

## D-008 — Single daemon, single machine, boring storage
**Context:** Personal scale is ~10⁶–10⁷ events. Distributed infrastructure would consume the
complexity budget needed for the actually-novel parts (kernel, memory, protocol).
**Decision:** One Rust daemon; append-only log segments + one SQLite db (FTS5, sqlite-vec,
property-graph tables); in-process scheduler and event table, no broker; Tauri surfaces.
**Tradeoffs:** SQLite single-writer serializes derived-store writes (fine at personal scale);
a future team/org edition will need a real re-architecture (accepted: that's a different product).
**Revisit when:** Any store hits measured limits — the log-rebuild escape hatch (D-013) makes
derived-store swaps cheap.

## D-009 — WASM plugins with MCP-compatible boundary
**Context:** Adapter maintenance is a permanent treadmill (Part 12.4); ecosystems are the only
durable answer; but third-party code with ambient authority is unacceptable given D-004.
**Decision:** Plugins are wasmtime modules with zero default imports — capabilities passed in
explicitly. MCP tool servers are supported at the boundary but wrapped in kernel mediation.
**Tradeoffs:** WASM constrains plugin authors (language, syscall surface); wrapping MCP costs a
mediation layer and some tool-latency; some native integrations will need blessed first-party
adapters instead.
**Revisit when:** If WASI maturity stalls, reconsider process-sandboxed plugins for heavy adapters.

## D-010 — Prompts, charters, skills, and policies are code
**Context:** A self-improving system without regression discipline self-degrades; English config
changed by vibes is how assistants rot.
**Decision:** All behavioral artifacts are versioned; changes pass per-user eval gates
(assembled from the user's own labeled history), canary, and are rollbackable. One improvement
loop only (ARCHITECTURE §12).
**Tradeoffs:** Slower iteration; personal eval sets are thin for subjective tasks (open research,
accepted); eval infrastructure is real up-front cost.
**Revisit when:** Never for the discipline; gate strictness will be tuned per artifact class.

## D-011 — Crypto-shredding as the forgetting mechanism
**Context:** Forgetting is what makes total memory acceptable — socially, legally, politically.
"We deleted it, trust us" is not a property.
**Decision:** Per-item encryption keys wrapped by the root key; forget = destroy keys +
tombstone propagation through derived claims (dependents lose evidence and decay/die). The
system can enumerate and prove what it no longer knows.
**Tradeoffs:** Key-management complexity on every write path; content-addressing must work over
ciphertext envelopes; backups of shredded items must also be provably dead (backup design must
inherit per-item keys).
**Revisit when:** Never for the property; the key hierarchy will be engineered iteratively.

## D-012 — Four agent roles in v1, not eleven *(v1.1 review)*
**Context:** DESIGN.md Part 7 specifies an 11-role society. Review verdict: an org chart, not an
architecture — each role adds prompt surface, dispatch logic, and failure modes with no evidence
of need. Part 12.6 flagged it; v1.0 didn't act on it.
**Decision:** v1 ships Steward, Worker (absorbs Planner/Researcher/Engineer/Envoy), Critic,
Archivist (absorbs Optimizer). Roles are charter *data*, so future splits are configuration.
Deferred roles' duties live as checklist items in surviving charters.
**Tradeoffs:** The Worker charter is broad, risking muddier prompts and weaker specialization;
Critic's security-review duty (Warden's job) may get shallower coverage.
**Revisit when:** Post-mortems attribute failures to charter overload — split exactly the role
the evidence indicts, nothing more.

## D-013 — Defer: multi-device sync, ambient sensors, learned consolidation trust *(v1.1 review)*
**Context:** Review found three premature-complexity sites: CRDT sync (no users, no second
device requirement yet), ambient watchers (privacy machinery not built; adapter treadmill), and
trusting consolidation output before measuring it.
**Decision:** v1 is single-device (log format keeps merge trivial later — "single-device by
decision, multi-device by design"); sensors are explicit adapters only (fs/git/shell/IDE);
consolidation runs but its claims are quarantined from agent context until Alpha-gate quality
metrics pass. `nx rebuild` (derived stores from log) is a supported operation from day one, making
every derived-store choice reversible.
**Tradeoffs:** Weaker demo appeal early; browser/comms value delayed; memory feels thinner in MVP.
**Revisit when:** Each deferral has an explicit gate in ROADMAP.md.

## D-014 — Capability ceilings, not taint detection *(v1.1 review)*
**Context:** DESIGN.md §5.3 claimed the kernel checks whether untrusted content "was the source
of an instruction." Review verdict: unsound — instruction-provenance through a model is
undecidable; a paraphrase launders any taint. Security claims must be enforceable, not aspirational.
**Decision:** Every context item carries an origin/trust tag; the kernel computes a **capability
ceiling** for the task — if any assembled context is below a trust threshold, the task's
capability set is capped (notably: no irreversible-class queuing, restricted write scope) for its
lifetime. Confinement, not detection.
**Tradeoffs:** Coarse: a task reading one untrusted web page loses authority even if the page was
irrelevant (mitigations: context partitioning — fetch-and-summarize in a low-privilege subtask,
pass the summary with the *subtask* as origin). Some legitimate workflows need two-phase designs.
**Revisit when:** Research provides sound information-flow control through models; until then, never.

## D-015 — Approval binds to the raw effect list, not the semantic summary *(v1.1 review)*
**Context:** v1.0 had users approving model-generated semantic diffs. Review verdict: soundness
hole — approving a summary written by the same system being supervised means approving a
possible hallucination.
**Decision:** Commit approval is over the kernel's ledger-derived effect list (files, queued
messages, capabilities exercised). Semantic summaries are advisory presentation, visibly labeled,
with one-keystroke drill-down to raw effects.
**Tradeoffs:** Raw effect lists are noisier; inbox UX must work hard to keep review cheap
(grouping, risk-ranking) without re-smuggling model judgment into the approval object itself.
**Revisit when:** Never for the binding; presentation will iterate forever.

## D-016 — Static router policy with outcome logging; learned routing deferred *(v1.1 review)*
**Context:** v1.0 specified outcome-learned routing. Review verdict: learning with zero outcome
data is a fantasy; ship the flywheel's intake first.
**Decision:** v1 router is a static, user-editable policy table (task class, privacy class,
budget → model) that logs every call's outcome. Learned routing turns on when data exists
(Beta gate).
**Tradeoffs:** Suboptimal early routing costs money and quality; acceptable versus building
learning infrastructure on imagined data.
**Revisit when:** Beta, with real per-user outcome distributions.

## D-017 — Provider-agnostic router; keys sealed, never hardcoded *(Phase 1)*
**Context:** The first router implementation hardcoded the Anthropic API call and read
`ANTHROPIC_API_KEY` directly — a Principle 5 violation caught in use: switching vendors would
have meant editing application logic, and key handling bypassed the record's encryption.
**Decision:** All providers implement one `Provider` trait behind a registry
(`anthropic`, `openai`, `gemini`, `openrouter`, `ollama`, `lmstudio`, `openai-compat` + `mock`).
The OpenAI chat-completions protocol is implemented once and parameterized — it is the de facto
standard spoken by OpenRouter/Ollama/LM Studio and most future providers, so `openai-compat`
with a custom `base_url` covers vendors that don't exist yet. Switching is a `router.json` edit
or `nx configure`; application logic never changes. API keys resolve environment-variable
override first (operators/CI), then the **sealed secrets store** (XChaCha20-Poly1305 under the
user's KEK — same root of trust as the record). The default route table is deliberately
`unconfigured` and refuses with guidance: a fresh record must never silently route to a vendor
the user didn't choose. Onboarding (`nx init` interactive / `nx configure`) prompts for
provider, model, and key (masked input; `--api-key-stdin` for scripts).
**Tradeoffs:** Lowest-common-denominator request shape (system + user + max_tokens) — no
vendor-specific features (tool use, caching) until the trait grows capability flags; one more
indirection layer; suggested default models will go stale and need periodic refresh.
**Revisit when:** A task class needs a vendor-specific capability — extend the trait with
feature discovery then, not before.

## D-018 — Identity: a personal epistemic institution; the assistant is Client #1 *(V2)*
**Context:** After the v0 loop closed, inventory showed the assistant is a thin pipeline while
everything load-bearing (evidence log, bets, truth maintenance, derivations, calibration,
consolidation) is domain-general epistemics. The dependency is asymmetric: the assistant needs
the epistemics; the epistemics needs only a *consequence stream* — which the assistant happens
to be the best source of (COGNITIVE_ARCHITECTURE Part XII).
**Decision:** The project's identity is a **personal institution for managing knowledge,
beliefs, decisions, evidence, and reasoning**. The assistant continues as Client #1 and the
primary evidence pump — subordinated, not removed (pure knowledge-management starves the
scoring loop). Mechanically: (1) the epistemic schemas (specs/record.md §11) are elevated to a
fourth sacred interface with the same change discipline; (2) ROADMAP v2.0 re-cuts phases into
kernel/application/platform tracks and promotes the institution's human surfaces (decision
artifacts, calibration reports, reconciliation reviews) into phase gates; (3) full backward
compatibility — every existing command, schema, spec, and passed gate remains valid; all
changes additive.
**Tradeoffs accepted:** Positioning is harder to explain than "assistant" (mitigated: market it
by its three jobs — remember, decide, delegate); assistant feature depth is deferred relative
to competitors during Phases 2–3; epistemic UX costs user attention (mitigated: system drafts,
human vetoes; automated settlement prioritized).
**Revisit when:** Pilot usage data exists. Kill criterion (also in ROADMAP v2 risk register):
if pilots actively use delegation but ignore the epistemic surfaces for two consecutive review
periods, recenter the identity on the assistant and demote epistemics to internal machinery.

## D-019 — The four-verb identity; claims deprecated; personas retired *(V2)*
**Context:** The four-audience explanation test (engineer / PM / non-technical / 15-year-old)
exposed three complexity leaks: a dual belief representation (`memory.claim/1` has no falsifier
requirement, contradicting "it only believes losable things" — and no code ever emitted one);
agent personas appearing in user-facing vocabulary though no explanation needed them; and an
identity described by a dozen nouns when every explanation reduced to the same four verbs.
**Decision:**
1. **The canonical identity is four verbs — REMEMBER, BELIEVE, ACT, SCORE.** Every
   identity-level feature must implement exactly one verb (REMEMBER: log, crypto, retrieval;
   BELIEVE: bets, consolidation, reconcile; ACT: kernel, capabilities, transactions, inbox,
   mandates; SCORE: resolutions, calibration, scorecards, trust). A feature that maps to none
   or several is rejected or split. Infrastructure (blackboard artifacts, router, charters,
   manifests) must never be *required* to explain the system — it serves the verbs invisibly.
2. **Claims are deprecated** (specs/record.md §8): bets are the sole reliance representation.
3. **Personas are retired from user-facing vocabulary**: roles remain as internal scheduling
   policy (charters), but surfaces report what happened ("plan drafted", "effects reviewed"),
   never who "did" it. No persona ships in UI, docs, or CLI output going forward.
**Tradeoffs:** Losing persona vocabulary costs some approachable anthropomorphism (accepted:
personas were the V1 scaffolding Part IX already demoted); the four-verb test may occasionally
force awkward splits of genuinely hybrid features (accepted: that friction is the point).
**Revisit when:** A load-bearing feature genuinely cannot be expressed under one verb —
that would be evidence the decomposition is wrong, not the feature.

## D-020 — Adversarial review verdict: the reduced core, the razor, and the cuts *(V2)*
**Context:** A hostile review from a competitor's chair ("prove NEXUS unnecessary"). Conceded
without defense: transactions are commoditizing (necessary, not differentiating); local-first
is a commercial handicap accepted as the price of independence; the agent runtime is commodity
scaffolding with zero unique value; decision journaling as a standalone product is a graveyard;
model-interpreted settlement risks laundering vibes through bookkeeping.
**Decision:**
1. **The load-bearing core is four elements** — owned/shreddable record, admission rule +
   settlement loop, capability-gated action as the evidence pump, provider-neutral routing
   (the scorekeeper cannot be owned by the measured). Nothing else may be described as
   invention; it is plumbing.
2. **Mechanical-settlement razor** (the one redesign, additive): every falsifier must name its
   settlement source; bets settleable only by model interpretation are flagged `interpretive`
   and scored in a separate calibration bucket. The honest core (mechanically settled) is
   never polluted by the soft periphery.
3. **Cuts:** Phase 5 skills marketplace deleted; Client #2 decision-journal surface deleted
   (`nx decide` demotes to optional capture; decisions are harvested from approve/reject flows
   the user already performs); blackboard demoted from sacred interface to internal versioned
   protocol (published compatibility surface = record + capability + epistemics only);
   charter/prompt optimization demoted from learning channel to hygiene.
4. **The identity litmus** (the one idea, for all future positioning): *NEXUS's memory can
   lose.* "Show me a belief you lost last month and what killed it" must always have an answer.
**Tradeoffs:** Fewer publishable interfaces (three, not four); less demo surface in Phase 2;
the razor adds one field and one bucket to specs 0.3.
**Revisit when:** The pre-registered Phase 2 experiment (ROADMAP v2.1 gate) resolves — it can
falsify the invention itself.

## D-021 — Pre-publication technical review: five objections, five accepted fixes *(V2)*
**Context:** Brutal design review simulating principal engineers (DB/OS/security/ML/KR) ahead
of open-sourcing. Three objections fully valid, two partially; all five fixes accepted as spec
0.3 / Phase 1–2 work items. Full text in PROGRESS session 17 and the review response.
**Accepted findings and fixes (smallest change each):**
1. *Scoring lacks probabilistic semantics* (VALID): add optional `credence` at placement,
   proper scoring at settlement; rename count-derived rank factor `calibration` → `reliability`;
   resolution events record their trigger (flow/scheduled/opportunistic) for bias stratification.
2. *"Truth maintenance" overstated* (PARTIALLY VALID): spec language becomes
   "dependency-directed retraction over **declared** premises"; derivation inputs normatively
   defined as context-not-premises; horizons documented as the staleness backstop where the
   graph is silent.
3. *No atomicity/isolation; commit-crash leaves an unrecoverable state* (VALID — traced in
   code: crash mid-apply reads as drift, refusing commit forever while abort's "target never
   touched" invariant is already violated): record lock file (single writer); `committing`
   intent state with idempotent roll-forward recovery from the workspace; `op_id` correlation
   + terminal markers on multi-event groups (commit ledgers, RECONCILE cascades).
4. *Privacy overclaims* (VALID in part): frame v2 — headers move inside the encrypted payload
   (only event id + envelope plaintext; metadata graph no longer readable at rest); forgetting
   claim reworded to "cryptographic erasure relative to ciphertext copies, contingent on
   keyring lifecycle"; KEK epochs (re-wrap on forget) added to Phase 2.
5. *O(N) decrypt-scan per operation contradicts the privacy rule* (VALID): rule restated as
   "no plaintext at rest — derived caches must be KEK-sealed"; sealed incremental materialized
   views for bets/blackboard/retrieval; 10⁶-event benchmark added to the Phase 1 gate.
**Also admitted on the record:** no mechanism moat exists (defensibility = accumulated records
+ vendor-neutrality positioning); S2/S3 strata, mandates, and operator taxonomy are manifesto,
not architecture, until implemented; grandiose register confined to DESIGN.md going forward.
**Revisit when:** each fix lands (they are individually falsifiable by the existing test
discipline); the moat admission is permanent.

---

STATUS: COMPLETE
