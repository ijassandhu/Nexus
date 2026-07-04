# NEXUS — A Personal Intelligence Substrate

**Internal architecture proposal · Draft 1 · July 2026**

> **Status: vision/rationale document (non-normative).** The post-review, normative architecture
> is [ARCHITECTURE.md](ARCHITECTURE.md) (v1.1), with decisions in [DECISIONS.md](DECISIONS.md)
> and phases in [ROADMAP.md](ROADMAP.md). Where documents disagree, ARCHITECTURE.md wins —
> notably: 4 agent roles at v1 (not 11), capability *ceilings* rather than the taint-detection
> described in §5.3, and approval bound to raw effect lists rather than semantic diffs.

> One-line thesis: The winning system is not the smartest assistant. It is the one that makes
> **delegation safe before making it smart**, and that makes the **person's accumulated record —
> not the model — the center of computing.**

---

## Part 0 — Interrogating the Premise

You asked to be told if the idea is mediocre. The idea is not mediocre, but the *framing* contains
five errors that, if built into the foundation, will kill the project. We deal with them before
designing anything.

### 0.1 Wrong assumption #1: "AI Operating System" is the right metaphor

An operating system schedules scarce compute and arbitrates access to hardware. Neither of those
is the scarce resource here. Compute is abundant and getting cheaper; model intelligence is
abundant and commoditizing. The scarce resources in personal computing circa 2026–2035 are:

1. **Trust** — how much authority a person is willing to hand over.
2. **Verified context** — what the system provably knows about the person's world, with provenance.
3. **The person's attention** — the only truly non-renewable input.

An "OS" competes with Microsoft and Apple on their home turf and loses. What should be built is a
**substrate**: the layer that owns the person's record and mediates every action taken on their
behalf. The OS metaphor that *does* transfer is the important one: like a kernel, it must mediate
**all** access — no agent touches the world except through it. We keep that and discard the rest.

### 0.2 Wrong assumption #2: memory is the bottleneck

Memory is necessary and the industry knows it; everyone is building memory. But memory alone
produces a *diary*, not a *deputy*. The actual bottleneck is stated in §0.4. A system that
remembers everything but cannot be safely handed authority is a better search engine over your
life — valuable, not transformative.

### 0.3 Wrong assumption #3: more capability → more delegation

Delegation is an economic decision the user makes, mostly unconsciously:

```
delegate iff  E[value of outcome]  >  cost(specifying the task)
                                    + cost(verifying the result)
                                    + E[cost of unrecoverable harm]
```

The entire industry is attacking the left side (make the model smarter, raise E[value]). Almost
nobody is attacking the right side. If verifying an agent's work costs as much as doing the work,
autonomy is worthless *regardless of model quality*. The design center of Nexus is the right side
of that inequality: make specification cheap (memory + context), make verification cheap
(diffs, provenance, explanation), and drive unrecoverable harm toward zero (transactions,
reversibility, graduated authority).

### 0.4 The single biggest bottleneck

**The verification gap.** AI cannot become a true digital companion because people cannot afford
to check its work, and cannot afford not to. Every other limitation — session amnesia, tool
fragility, hallucination — is either downstream of this or fixable with engineering. The one
revolutionary capability (asked for in your brief, answered fully in §3.1) is therefore not a
smarter agent; it is **transactional computing**: every autonomous action runs speculated,
inspectable, and reversible, and irreversible effects pass through explicit trust gates.

### 0.5 Wrong assumption #4: "think in decades" means designing for decades

Linux did not survive three decades because its 1991 code was farsighted. It survived because its
**interfaces** were stable (the syscall boundary is sacred), its **data formats** were open, and
its governance let implementations churn freely underneath. If Nexus is to be "the Linux of"
anything, the durable artifacts are: the **record format**, the **capability/permission model**,
and the **agent–substrate protocol**. Every implementation detail in this document should be
assumed wrong by 2030 and replaceable without the user losing anything.

### 0.6 Wrong assumption #5: people want richer interaction with computers

They want *less* interaction. The success metric of this system is not engagement; it is
**interactions avoided per week** and **decisions surfaced at the right moment**. Any design that
increases time-in-chat is failing. The chat window is the *debugger* of this system, not its IDE.

### 0.7 If every current AI assistant disappeared tomorrow

Redesigning personal computing from first principles, the central defect is obvious and old:
**the application owns the state**. Your life is sharded across app silos — mail in one, code in
another, decisions in Slack, commitments in a calendar, knowledge in twelve wikis — each with an
opaque store and a proprietary API. The person, the only entity all of it is *about*, owns none
of it and can query none of it.

Every previous attempt to fix this (semantic desktop, Plan 9, OpenDoc, the memex lineage) died on
the same rock: **normalizing heterogeneous human context into a coherent record was intractable**.
That is the thing that changed in 2023. A large language model is a *universal adapter* — it can
read anything humans produce and emit structure. First-principles personal computing in the LLM
era inverts the ownership: **state belongs to the person; applications and models become views and
capabilities over the personal record.** That inversion is the deepest idea in this document, and
it is the answer to the final challenge (Part 12).

### 0.8 Architectural mistakes in today's assistants

1. **Chat as the data model.** Conversation is an input modality being abused as a database.
   Context should live in a structured record; chat should be one lens onto it.
2. **The context window as memory.** Context is a cache, not a store. Systems that conflate them
   get amnesia, and their "memory" features are session summaries — lossy, unqueryable, no provenance.
3. **RAG as a bolt-on.** One-shot vector similarity over unowned data is not memory. Retrieval
   must be a *planned, multi-index operation* over a record the system maintains (§6.7).
4. **Ambient authority.** OAuth-scope permissioning gives an agent everything or nothing, forever.
   An agent that can "read email" can read *all* email in *every* task. This is the pre-Multics
   security model and it is how prompt injection becomes catastrophe. Capabilities must be
   task-scoped, delegable, and revocable (§5.5).
5. **The anthropomorphic singleton.** One persona doing everything is a UI mistake (it hides what
   the system is doing) and a security mistake (one confused deputy with all privileges).
6. **Prompts as configuration.** Behavior encoded in unversioned English, changed by vibes, with
   no eval gate and no rollback. Prompts must be treated as code (§9.3).
7. **The model at the center.** Today's stacks are built *around* a vendor's model. The model
   should be a **peripheral** — the best available reasoning coprocessor, swappable per task, per
   year, per jurisdiction — while identity, memory, and trust live in the substrate.
8. **Stateless agent loops.** Agent frameworks are while-loops with tool calls and no durable
   world model. Crash the process, lose the plan. All agent state must live in the substrate so
   agents are stateless, resumable, auditable, and model-swappable (§7.4).

### 0.9 Capabilities people don't know they need

- **A diff for your day.** "What changed in my world since yesterday, and what did anything I
  delegated actually do?" — rendered like a code review, not a feed.
- **Provenance on beliefs.** Ask the system *why* it believes anything about you and get sources.
- **Workflow capture.** The system watches you do a thing three times, writes the program,
  and offers it back (§6.5).
- **Negotiated autonomy.** Authority that is *earned per skill*, expands with a track record, and
  contracts on failure — visible as a ledger, like a credit score you can audit (§5.6).
- **Verifiable forgetting.** Not "we deleted it, trust us" — cryptographic shredding (§6.9).
- **Firing your model vendor.** Switch the intelligence out and lose nothing of *yourself*.

---

## Part 1 — Vision

### The problem

A person's digital life has outgrown the person. The average knowledge worker operates across
dozens of apps holding thousands of threads of state, and carries the integration layer *in their
head*. Every context switch, every "where was I", every re-explanation to a stateless assistant is
the tax paid because **no system owns the whole picture and none can be trusted to act on it.**

### What Nexus is

Nexus is a **personal intelligence substrate**: a local-first, user-owned system that

1. **maintains the record** — a permissioned, provenance-tracked account of the user's digital
   life, continuously normalized into memory;
2. **mediates all action** — every agent operation passes through a trust kernel that makes work
   inspectable, reversible where possible, and gated where not;
3. **hosts a society of agents** — specialized, stateless workers that plan, execute, criticize,
   and learn on the user's behalf;
4. **improves without retraining** — the growing assets are the record, the knowledge graph, the
   skill library, and the trust ledger, all outside any model's weights.

### Why it is inevitable

Three curves cross. (1) Model capability is passing the threshold where multi-step delegation is
*technically* possible, so the binding constraint flips from intelligence to trust. (2) Model
pricing is collapsing toward utility economics, so vendor moats built on model quality erode, and
value migrates to whoever holds the durable context. (3) Regulatory and cultural pressure on
personal data is rising, favoring architectures where the user demonstrably owns the record.
Someone will build the layer that sits at the intersection. The only question is whether it is
built open and user-owned, or arrives as the most invasive product in the history of computing,
run by whoever already owns your OS. That is why this project matters and why "Linux of" is the
right ambition: **the reference implementation of personal AI must be the one the user controls.**

---

## Part 2 — Core Principles

These are immutable. Everything else in this document is negotiable.

1. **The record belongs to the person.** Local-first, encrypted at rest with user-held keys,
   exportable in documented formats, syncable through blind relays. No component of Nexus may
   require surrendering the record to function. *Why:* it is the moat, the liability, and the
   point. Violate this and Nexus is just another data harvester with a manifesto.

2. **The kernel mediates everything.** No agent, plugin, or model touches the world — filesystem,
   network, app, human — except through the trust kernel, with a task-scoped capability, leaving a
   ledger entry. *Why:* auditability and injection containment are impossible to retrofit.

3. **Irreversibility is the enemy.** Every action is classified by reversibility. Reversible
   actions run speculatively and show diffs. Irreversible actions (send, pay, delete-remote,
   publish) queue at trust gates until authority for that action class has been earned. *Why:*
   this is the direct attack on the verification gap (§0.4).

4. **Provenance or it didn't happen.** Every memory, belief, and derived claim carries edges to
   the episodes that produced it, a confidence, and a timestamp. The system can always answer
   "why do you believe this?" *Why:* memory without provenance becomes confidently-wrong
   autobiography, and there is no debugging it.

5. **Models are peripherals.** Any reasoning step must be executable by any sufficiently capable
   model through the router. Nothing user-durable may live in a vendor's fine-tune or a vendor's
   memory feature. *Why:* §0.7 — the inversion is the strategy. Also: models will be swapped
   many times over a decade; the person's system must survive every swap.

6. **Agents are stateless; the substrate is the state.** Plans, working memory, intermediate
   artifacts — all live in the record. Any agent can crash, resume, or be replaced mid-task.
   *Why:* crash-safety, auditability, horizontal scaling, and model-swappability come free.

7. **Interfaces are sacred; implementations are disposable.** The record schema, the capability
   model, and the agent–substrate protocol are versioned, documented, and evolved with the
   discipline of a syscall boundary. *Why:* §0.5 — this is the only known way to last decades.

8. **Attention is the budget.** Every proactive surface (notification, briefing item, question)
   spends from an explicit, measured attention budget. The system optimizes for interactions
   avoided. *Why:* §0.6 — a companion that costs more attention than it saves is a parasite.

9. **Trust is earned, granular, and legible.** Autonomy is granted per (skill, context,
   consequence-class), expanded by track record, contracted by failure, and always visible as a
   ledger the user can read and edit. *Why:* flat permission grants are both unsafe and — worse —
   *unconvincing*, so users never delegate.

10. **Prompts, skills, and policies are code.** Versioned, eval-gated, canaried, rollbackable.
    *Why:* a self-improving system without regression discipline self-degrades.

---

## Part 3 — The Ten Revolutionary Ideas

### 3.1 Transactional computing (the ONE capability)

If Nexus shipped a single capability, this is it. Every autonomous task executes inside a
**workspace transaction**: a copy-on-write view of the affected state (files via snapshotting,
app state via adapter-level staging, communications via outbox-holding). The agent works at full
speed inside the transaction; the user reviews a **semantic diff** — not "347 file changes" but
"renamed the API, updated 12 call sites, drafted the migration email (held)". Commit applies;
abort vanishes; partial commit is first-class. Irreversible effects never execute inside a
transaction — they accumulate at the **effect boundary** and fire only on commit, by which point
they have either earned-autonomy clearance or explicit approval.

*Why better:* every current agent either asks permission per keystroke (exhausting) or runs open
loop (terrifying). Transactions give full-speed autonomy with bounded harm — the same trick that
let databases and version control scale human trust. It converts "watch the agent" into "review
the diff," which is 10–100× cheaper, which — per §0.3 — is what actually unlocks delegation.

### 3.2 The ownership inversion

The person's record is the center; models and apps are peripherals (§0.7). *Why better:* every
competitor's moat is the model or the app; both commoditize. A record that compounds for years is
the only asset in this industry that gets *more* defensible over time — and because the user owns
it, the "lock-in" is loyalty rather than captivity, which survives regulation and platform wars.

### 3.3 The trust ledger (earned, granular autonomy)

Authority is a first-class data structure: per skill, per context, per consequence class, with a
visible track record. New skills start supervised; success expands scope; failure contracts it,
automatically and legibly. *Why better:* today autonomy is a static setting chosen in fear.
A ledger makes autonomy a *trajectory* — and gives the user the experience of watching their
system earn its keep, which is the emotional core of adopting a deputy.

### 3.4 Provenance-complete memory

Every belief is a claim node with edges to evidence episodes, confidence, and freshness; nothing
is remembered without a *why*. *Why better:* solves memory poisoning (injected content is tagged
by origin and quarantined from authority-bearing contexts), enables true forgetting (delete the
evidence, and dependent claims decay), and makes the system's self-model debuggable.

### 3.5 Procedural memory as compiled skills

Repeated workflows are *compiled into typed programs* (with preconditions, effects, eval
histories), not re-derived by a model each time. The model writes the skill once; the substrate
runs it deterministically forever, escalating to a model only on precondition failure.
*Why better:* current agents re-reason every execution — slow, expensive, and nondeterministic.
Compilation makes routine automation fast, cheap, auditable, and *improvable* (skills are code;
code can be reviewed, versioned, and eval-gated). This is how the system gets faster and cheaper
every month while everyone else's agent bills grow.

### 3.6 The blackboard society (typed artifacts, no agent chatter)

Agents do not chat with each other. They read and write **typed artifacts** (plans, findings,
diffs, critiques, decision memos) on a shared blackboard in the substrate, under contracts.
*Why better:* natural-language agent-to-agent chatter is lossy, unauditable, and compounds
hallucination. Typed artifacts give checkable interfaces between agents, deterministic replay,
and the ability to swap the model behind any role without renegotiating a "team culture."

### 3.7 Retrieval as query planning

"Remembering" is a planned operation: a retrieval planner compiles a question into a multi-index
plan (temporal scan + graph walk + lexical + vector + freshness policy), executes, and *cites*.
*Why better:* one-shot vector similarity is the memory equivalent of full-table scan. Planned
retrieval is the difference between a search box and a memory.

### 3.8 The delegation inbox and the daily diff

The primary UI object is not a chat thread; it is a **review queue**: pending transactions with
semantic diffs, held communications, decisions needing a human, each with provenance and a
one-glance risk summary — plus a morning **briefing that is a diff of your world**, not a feed.
*Why better:* it matches the actual job (supervising a deputy) instead of the borrowed metaphor
(texting a friend), and it is the surface on which trust is *visibly* earned.

### 3.9 Consolidation as sleep

A scheduled background process ("night shift") turns episodes into semantic claims, reconciles
contradictions, strengthens the useful, decays the stale, generates hypotheses ("user seems to
prefer X — unconfirmed"), and queues reflection questions. *Why better:* every other system
either stores raw logs (unusable) or summarizes at write time (lossy, wrong granularity).
Biological memory separates experience from consolidation for good reasons; so should we.

### 3.10 Verifiable forgetting

Per-item encryption keys; "forget" destroys keys (crypto-shredding), tombstones propagate through
derived claims, and the system can *prove* what it no longer knows. *Why better:* forgetting is
the feature that makes remembering acceptable. No current assistant can demonstrate deletion;
Nexus can make it a mathematical property. This single property changes the political viability
of the entire category.

---

## Part 4 — Complete Architecture

```
┌─────────────────────────── SURFACES ────────────────────────────┐
│  Intent Bar · Delegation Inbox · Briefing · CLI · IDE plugin    │
│  Browser overlay · Voice capture · Notifications (budgeted)     │
└───────────────▲─────────────────────────────▲───────────────────┘
                │ intents / reviews           │ renders / asks
┌───────────────┴─────────────────────────────┴───────────────────┐
│                        AGENT SOCIETY                            │
│  Steward · Planner · Researcher · Engineer · Critic · Archivist │
│  Warden · Tutor · Optimizer · Scout · Envoy   (stateless roles) │
│         — communicate via typed artifacts on the Blackboard —   │
└───────▲──────────────────▲──────────────────────▲───────────────┘
        │ read/write        │ reason via           │ act via
┌───────┴────────┐  ┌───────┴────────┐  ┌──────────┴──────────────┐
│   SUBSTRATE    │  │  MODEL ROUTER  │  │      TRUST KERNEL       │
│  (the Record)  │  │  local + API   │  │ capabilities · txns ·   │
│ episodic log   │  │ capability     │  │ effect boundary ·       │
│ semantic graph │  │ profiles ·     │  │ policy engine · ledger  │
│ skills · plans │  │ outcome-tuned  │  └──────────┬──────────────┘
│ blackboard     │  │ routing        │             │ mediated I/O
│ trust ledger   │  └────────────────┘  ┌──────────▼──────────────┐
└───────▲────────┘                      │   ADAPTERS (WASM)       │
        │ ingest (quarantined)          │ fs · shell · browser ·  │
┌───────┴─────────────────────────────  │ mail · calendar · apps  │
│  SENSORS: file watcher · browser ·    │ (MCP-compatible)        │
│  comms · IDE · shell · voice          └─────────────────────────┘
│         EVENT BUS + SCHEDULER (cron, triggers, attention budget)│
│         SYNC: E2E-encrypted, CRDT, blind relay                  │
└─────────────────────────────────────────────────────────────────┘
```

### 4.1 The Substrate (state)

- **Episodic log**: append-only, content-addressed, per-item-encrypted event stream. Every
  observation and every agent action lands here first. Immutable; the ground truth.
- **Semantic graph**: entities (people, projects, tools, commitments, preferences) and claims,
  each with provenance edges, confidence, and freshness. Derived — always rebuildable from the log.
- **Skill library**: compiled procedures (§6.5) with versions, preconditions, eval records.
- **Plan store**: plans as data — typed DAGs with step status, so execution is resumable.
- **Blackboard**: the agents' shared working memory; typed artifacts with schemas.
- **Trust ledger**: capability grants, track records, autonomy levels per skill/context.

### 4.2 The Trust Kernel (control)

The only path to the world. Components: **capability manager** (mints task-scoped, attenuated,
revocable capabilities — object-capability model, no ambient authority), **transaction manager**
(workspace snapshots, semantic diff generation, commit/abort, the effect boundary for
irreversibles), **policy engine** (declarative rules binding consequence classes to required
autonomy levels; Warden proposes, user ratifies), and the **effects ledger** (every mediated call,
signed and queryable — the flight recorder).

### 4.3 Ingestion (senses)

Sensors observe what the user permits: filesystem watcher, browser extension, mail/calendar
adapters, IDE and shell integration, voice capture. Everything ingested passes through
**quarantine**: content is tagged with origin and trust class, PII-classified (by a *local*
model), and — critically — external content is marked non-authoritative so it can never silently
become an instruction to an agent (prompt-injection containment at the data layer, §11).

### 4.4 Event bus & scheduler (time)

A durable pub/sub spine: sensors publish, consolidation subscribes, agents subscribe to triggers
("PR merged", "email from X", "every morning at 7"). The scheduler owns cron, deadlines,
follow-ups ("if no reply in 3 days"), and the **attention budget** — proactive surfaces draw from
a measured allowance, and the Optimizer tunes what was worth interrupting for.

### 4.5 Model Router (reasoning)

Maintains **capability profiles** per available model (cost, latency, context, strengths measured
by *outcomes on this user's tasks*, not benchmarks). Routes each reasoning step by task class,
privacy class (some content never leaves the device → local model), and budget. Router policy is
itself learned (§9). Vendor-independence is enforced here: every agent request is expressed in a
neutral reasoning interface.

### 4.6 Plugin framework (extension)

Adapters and third-party skills run as **WASM modules** with no default capabilities — every
syscall-equivalent is a capability handle passed in explicitly. MCP-compatible at the boundary so
the existing tool ecosystem plugs in, but MCP servers are wrapped in kernel mediation (an MCP tool
never gets ambient authority just because it was installed). Plugins declare effect classes for
policy binding, and are the community's surface — this is where "Linux of" gets its ecosystem.

### 4.7 Sync & state management

Local-first. The episodic log is append-only and content-addressed, hence trivially mergeable
across devices. Mutable derived stores (graph, ledger) sync via CRDTs. Transport is an
E2E-encrypted blind relay (self-hostable). Conflict semantics: the log never conflicts; derived
stores are rebuildable; the only genuinely concurrent mutable object is the trust ledger, which
merges conservatively (lowest autonomy wins).

### 4.8 Desktop integration

Nexus ships as a background daemon (Rust) plus thin surfaces: a global intent bar, a tray/menu
presence, an inbox window, a CLI (`nx`), an IDE extension, a browser extension. It does not try
to replace the OS shell in v1; it *inhabits* the existing desktop and earns its place. (The
long-game inversion — apps as views over the record — is Version 3+, §10.)

---

## Part 5 — Security Architecture

Security is not a subsystem here; it is the product (Principle 2). Specifics:

1. **Object capabilities, not roles.** A task to "reply to Anna about the invoice" mints
   capabilities for: read thread(Anna, invoice), draft in outbox. Not "read email." Capabilities
   are attenuable, delegable to sub-agents in weakened form, time-boxed, and revoked at task end.
2. **The effect boundary.** Irreversible effect classes (external send, payment, remote delete,
   publish, contract) are enumerated, and *no code path exists* for executing them inside a
   speculative workspace. They queue; the kernel fires them at commit under ledger authority.
3. **Injection containment.** The lethal trifecta (private data + untrusted content + external
   comms) is broken structurally: content carries origin tags through every context assembly;
   untrusted-origin content cannot be the *source of an instruction* that exercises a capability
   above its trust class — the kernel checks the taint chain, not the model's judgment.
4. **Local-by-default privacy tiers.** Every record item has a privacy class; classes bind to
   routing policy (what may be sent to which model vendor) and to sync policy.
5. **The trust ledger** (§3.3) as the human-facing security model: legible, editable, earned.
6. **Crypto-shredding** for forgetting (§6.9); user-held root keys; the vault is useless stolen.
7. **The audit stance:** every mediated action is replayable from the effects ledger. "What did
   you do while I was away, and why?" has a complete, signed answer. Aspirationally, kernel and
   ledger are small enough to be formally audited — the TCB is the kernel, not the models.

---

## Part 6 — Memory

The memory system is the substrate's beating heart. Design goal: **the system is measurably more
useful every month, with zero foundation-model retraining**, because what grows is outside the
weights: record, graph, skills, routing statistics, trust.

### 6.1 Episodic memory

Raw experience: events with timestamp, origin, actor, content pointer, privacy class, trust tag.
Append-only and immutable (edits are new events). Cheap to write, never summarized at write time —
write-time summarization bakes in today's (wrong) guess about tomorrow's questions.

### 6.2 Semantic memory

The knowledge graph: entities and **claims** ("prefers PRs under 400 lines", "project Atlas ships
Sept 12", "Anna is the decision-maker on X"). Every claim: provenance edges → episodes,
confidence, created/verified/expires timestamps, and status (observed / inferred / hypothesized /
user-stated / contradicted). User-stated outranks inferred; contradictions are surfaced, not
silently resolved.

### 6.3 Procedural memory

Skills (§3.5): typed, versioned programs with declared preconditions, capability requirements,
effect classes, and eval history. Lifecycle: **observed** (pattern detected in episodes) →
**drafted** (Engineer writes the skill, Critic reviews) → **supervised** (runs only inside
reviewed transactions) → **autonomous** (per trust ledger). Skills call models as subroutines for
judgment steps but are deterministic in structure.

### 6.4 Project memory

Per-project working sets: the live plan, open loops, decision journal (auto-maintained ADRs:
decision, alternatives considered, rationale, provenance, outcome-when-known), key artifacts, and
the project's people. This is what makes "resume where we left off, it's been three weeks"
a five-second operation instead of a thirty-minute one.

### 6.5 Relationship graph

People: interaction cadence, commitments in both directions, preferences, tone models
("terse with Marcus, warm with Priya"), and history pointers. Commitments are first-class —
"you told Anna Thursday" is a scheduled, tracked object, and the single most quietly
life-improving feature in this document.

### 6.6 Decision history

Choices with alternatives and rationale, linked to outcomes as they arrive. Enables the
Optimizer's counterfactual reviews ("we chose X over Y three times; X underperformed twice")
and the user's own "why did we do it this way?" — answered with receipts.

### 6.7 Retrieval

A **retrieval planner** (small, fast model + heuristics) compiles each memory need into a plan
over five indexes: temporal (time-range scans), graph (entity walks), lexical (exact/BM25),
vector (semantic), and freshness/confidence filters. Results return *with provenance*, are
assembled into context under the taint rules (§5.3), and the plan itself is logged — so retrieval
failures are debuggable and the Optimizer can tune the planner on outcome data.

### 6.8 Consolidation — the night shift

Scheduled (idle-time) pipeline over new episodes: extract candidate claims → reconcile against
the graph (corroborate ↑confidence, contradict → flag) → link entities → compress episode spans
into narrative summaries (originals retained) → decay untouched claims → detect procedural
patterns → emit **hypotheses** (stored as claims with `hypothesized` status, confirmed or killed
by future evidence, surfaced to the user only when confidence and relevance warrant the
attention spend).

### 6.9 Forgetting

Three modes. **Decay**: confidence and retrieval priority fall without reinforcement; decayed
claims stop influencing behavior long before deletion. **Deprecation**: superseded claims keep
history but lose authority. **Destruction**: user-decreed; per-item keys are shredded, tombstones
propagate (dependent claims lose evidence and decay or die), and the system can enumerate what
was forgotten and prove it can't be recovered. Forgetting is what makes total memory livable.

### 6.10 Reflection

Weekly, the Archivist and Optimizer run structured reflection: contradictions worth resolving
with the user (batched into one low-attention review), hypotheses worth testing, skills worth
proposing, memory areas gone stale, and a self-assessment appended to the record — which makes
the system's own competence trajectory a queryable object.

### Why this compounds

Month 1: the record is thin; Nexus is a good assistant with receipts. Month 6: the graph knows
your projects and people; specification cost has collapsed ("handle the Atlas standup prep" is a
complete instruction). Month 18: fifty skills run autonomously at near-zero marginal cost; the
router knows which model is good at which of *your* tasks; the trust ledger has earned real
authority. None of that lives in a model. Swap every model in 2029; Nexus doesn't blink.

---

## Part 7 — The Agent Society

### 7.1 Roles

| Agent | Charter |
|---|---|
| **Steward** | Chief of staff. Owns intent triage, dispatch, and the user's attention budget. The only agent that speaks to the user unprompted. |
| **Planner** | Turns intents into typed plan DAGs with declared effect classes and checkpoints. |
| **Researcher** | Gathers and grounds: record retrieval, web, docs. Produces *findings* with provenance, never conclusions without evidence. |
| **Engineer** | Executes: code, files, app operations — always inside transactions. Drafts skills from observed patterns. |
| **Critic** | Adversarial reviewer with **veto**. Reviews plans before execution and diffs before commit-proposal. Prompted to refute, not to approve. |
| **Archivist** | Memory keeper: owns consolidation, graph hygiene, retrieval quality, forgetting. |
| **Warden** | Security auditor: owns policy proposals, taint review, capability minimization, injection drills. Reviews every new skill's capability manifest. |
| **Tutor** | Explains the system to the user; surfaces what Nexus learned; teaches the user what it *could* do next (adoption is a curriculum). |
| **Optimizer** | Meta-agent: failure post-mortems, prompt/routing/skill tuning behind eval gates (§9). |
| **Scout** | Ambient monitoring: watches subscribed streams for events that matter; feeds the briefing. |
| **Envoy** | External communications specialist: drafts in the user's voice(s); everything it writes is held at the effect boundary by definition. |

### 7.2 Communication

No inter-agent chat. Agents read/write **typed artifacts** on the blackboard: `Intent`, `Plan`,
`Finding`, `Diff`, `Critique`, `DecisionMemo`, `SkillProposal`, `Incident`. Each type has a
schema and a contract (a `Critique` must cite the artifact lines it disputes; a `Finding` must
carry provenance). Handoffs are blackboard state transitions, so the entire "conversation" is
replayable, and any role's model can be swapped without social renegotiation.

### 7.3 Cooperation and conflict

Standard flow: Steward accepts an Intent → Planner emits a Plan → Critic reviews (veto returns it
with a Critique) → Engineer/Researcher execute steps inside a transaction → Critic reviews the
Diff → Steward routes to the Delegation Inbox or auto-commits per trust ledger. Deadlock rule:
after two Plan–Critique cycles, the disagreement is escalated to the user as a one-screen
DecisionMemo (options, tradeoffs, both agents' positions). Disagreement is a feature; the memo is
its product.

### 7.4 Statelessness

Every agent is a process that wakes, reads the blackboard, does bounded work, writes artifacts,
and exits. All continuity is substrate. Consequences: crash-safe, resumable, parallelizable,
auditable, and — because the "agent" is really a *charter plus a model call plus capabilities* —
each role can run on a different model, chosen by the router, changed at any time.

---

## Part 8 — Interaction

Design north star: **calm**. The system is ambient by default, summonable in one keystroke,
and spends attention like money.

- **The Intent Bar.** One global hotkey anywhere in the OS. Three verbs: *ask* (query the
  record/world), *do* (delegate a task), *find* (retrieve anything you've seen). Context flows in
  automatically (active app, selection, project) so most intents are one short sentence.
- **The Delegation Inbox.** The core new UI object (§3.8): pending transactions as semantic
  diffs, held communications, decision memos, autonomy proposals. Keyboard-driven; a good user
  clears it in minutes. This is where trust is visibly earned — the inbox shrinks over months as
  the ledger grows.
- **The Briefing.** A morning diff of your world: what changed, what Nexus did, what needs you,
  what's at risk (commitments, deadlines, silence-from-people). Never a feed; always a diff.
- **CLI (`nx`).** Full parity for power users; `nx do`, `nx ask`, `nx log`, `nx diff`, `nx trust`.
  Scriptable — Nexus itself is automatable, which is how power users become plugin authors.
- **IDE & browser.** In-situ annotation, not context-switch: the record's knowledge surfaces
  where you're looking (this PR touches the thing you decided against in March — memo attached).
- **Voice.** Capture-first, command-second: thinking out loud into episodic memory is a
  first-class act ("note: Anna prefers the phased rollout") — command dialogs are secondary.
- **Chat.** Exists — as the drill-down/debugger surface attached to any artifact ("why did you
  hold this email?"), not as the front door.
- **Background execution** is the default mode of all work; the surfaces exist to *supervise*,
  which is the point of the whole design.

---

## Part 9 — Self-Improvement Without Retraining

The foundation models are frozen; the *system* learns. Every improvement mechanism follows one
loop: **outcome-labeled episodes → analysis → versioned change → eval gate → canary → adopt or
roll back.** Nothing self-modifies outside this loop (Principle 10).

1. **Outcome labeling.** Every task ends with a cheap signal: committed / aborted / edited-then-
   committed (edit distance is gold), user correction, explicit rating (rare), downstream success.
2. **Failure post-mortems.** Aborts and corrections trigger the Optimizer: was it retrieval
   (wrong context), planning (wrong decomposition), execution (wrong action), or specification
   (wrong understanding)? The classification lands in the record; patterns become fixes.
3. **Prompt/charter optimization.** Agent charters and skill prompts are versioned artifacts;
   proposed edits must beat the incumbent on a per-user eval set (assembled from the user's own
   labeled history — real personal evals, not benchmarks) before canary deployment.
4. **Routing policy learning.** The router's model-capability profiles update from outcomes:
   which model, at which cost, succeeded at which task class *for this user*.
5. **Skill discovery and refinement.** The observed→drafted→supervised→autonomous ladder (§6.3)
   is the system's compiler from experience to cheap deterministic competence; skill eval
   histories gate promotion, incidents trigger demotion.
6. **Retrieval tuning.** Retrieval plans are logged with outcomes; the planner learns which index
   mixes answer which question shapes.
7. **Knowledge organization.** Consolidation quality is itself measured (how often did retrieved
   claims get used and confirmed?) and tuned.
8. **The trust ledger as the master learning curve.** Autonomy expansion is the visible integral
   of all of the above — the user experiences self-improvement as *the inbox getting shorter*.

---

## Part 10 — Technology Choices

| Layer | Choice | Why | Rejected |
|---|---|---|---|
| Core daemon | **Rust** | Always-on, memory-safe, low-footprint, WASM host, decade-viable | Node/Electron core (footprint, safety); Python core (deployability) |
| Record store | **SQLite** + append-only content-addressed log segments | Boring, embedded, ubiquitous, 30-year format stability; the log is just files — user-ownable | Postgres (server dependency); cloud DBs (violate Principle 1) |
| Vector index | **sqlite-vec / embedded HNSW** | Personal scale is millions, not billions; stays in-process | Pinecone/Weaviate etc. (cloud-primary, absurd for personal scale) |
| Lexical | **Tantivy** | Embedded BM25, Rust-native | Elasticsearch (a JVM cluster for one human) |
| Graph | Property graph **on SQLite**; revisit Kuzu at scale | Personal graphs are small; avoid a second store until proven | Neo4j (server, license) |
| Sync | **CRDTs (Loro/Automerge) + E2E blind relay**, self-hostable | Local-first with multi-device; log merges trivially | Cloud-primary sync (Principle 1) |
| Plugins | **WASM (wasmtime)**, capability-passed imports; **MCP-compatible boundary** | True sandbox + ecosystem compatibility with kernel mediation | Native plugins (no sandbox); processes-only (too coarse) |
| Local models | **whisper.cpp** (voice), small local LLM via **llama.cpp/candle** (routing, PII classification, retrieval planning) | Privacy tiers require never-leaves-device inference | — |
| Frontier models | Via **router**, neutral interface, all major vendors | Principle 5 | Single-vendor SDK coupling; fine-tuning as personalization (locks identity into weights) |
| Surfaces | **Tauri** shell UI; native extensions for IDE/browser | Light, Rust-integrated | Electron (footprint for an always-on layer) |
| Agent runtime | In-house thin runtime on the blackboard protocol | The runtime *is* the product's core IP; frameworks impose the chat-loop model we reject | LangChain-style frameworks (wrong abstraction, churn) |
| Orchestration | Single daemon, embedded scheduler | It's a personal system; distributed-systems cosplay kills it | Kubernetes, microservices (scale problem we don't have) |

General rule applied throughout: **boring storage, radical semantics.** Innovation budget is
spent on the trust kernel, memory model, and agent protocol — never on exotic infrastructure.

---

## Part 11 — Roadmap

**Phase 0 — Spike (2–3 mo).** The trust kernel walking skeleton: capability minting, one adapter
(filesystem+shell), workspace transactions with semantic diff, effects ledger, `nx` CLI. *Risk
retired: is transactional agent work actually buildable and pleasant?*

**MVP (mo 3–8) — "the deputy for developers."** Target user: developers (most instrumentable
environment, highest injection-risk tolerance, plugin-author pipeline). Episodic log + basic
retrieval; Steward/Planner/Engineer/Critic only; Delegation Inbox; IDE + CLI surfaces; router
with two vendors + one local model. *Success metric: ≥5 real tasks/week delegated per active
user with commit-rate >80%.*

**Alpha (mo 8–14).** Consolidation night shift; semantic graph with provenance; skill ladder
(observed→supervised); briefing; browser sensor; Archivist + Warden. *Risk: consolidation
quality — this phase exists to find out (§12.2).*

**Beta (mo 14–22).** Full society; comms adapters (mail/calendar) with effect boundary; Envoy +
held-outbox; trust ledger v1 with earned autonomy; sync across devices; plugin SDK + WASM
sandbox opened to third parties.

**V1 (mo 22–30).** Hardening: crypto-shredding, audit tooling, attention-budget tuning,
record export/import, self-hosted relay. Public protocol specs (record format, capability model,
agent–substrate protocol) — the "Linux of" move is publishing these.

**V2.** Informed by Part 12's critique: compensation framework for irreversibles, org/team
edition (shared substrates with inter-personal capability grants), skill marketplace with
Warden-style review, mobile capture surfaces.

**V3.** The inversion matures: apps-as-views experiments — the record becomes the primary copy
for select domains (notes, tasks, contacts) with apps rendering it. This is the decade bet.

**Top risks:** (1) consolidation quality (research risk, gated at Alpha); (2) adapter treadmill
(mitigated by plugin ecosystem + MCP compatibility, but see §12.4); (3) prompt injection arms
race (structural containment helps; drills continuous); (4) the trust cold-start — if early
errors salt the earth, the ladder never gets climbed (mitigated by starting in low-stakes,
high-reversibility domains).

---

## Part 12 — Brutal Critique

The proposal, attacked honestly.

1. **Reversibility has a hole, and it's the important one.** The most valuable delegations —
   communication, purchases, publishing — are exactly the irreversible ones. "Held at the effect
   boundary" means the highest-value actions still queue for a human, and *compensation* (send a
   correction, request a refund) is a weak substitute for undo. The transaction story is genuinely
   revolutionary for files/code/state and merely *good hygiene* for the rest. V2 must invest in a
   real compensation framework and in confidence-calibrated auto-commit for low-stakes sends, or
   the inbox never shrinks past a floor.
2. **Consolidation may write confidently-wrong autobiography.** Claim extraction and
   contradiction resolution at personal scale, unsupervised, is a research problem wearing an
   engineering costume. Provenance makes errors *debuggable*, not absent — and a user who
   catches their system believing something false about them loses trust disproportionately.
   Mitigation (surface hypotheses, user-stated outranks inferred) costs attention, which fights
   Principle 8. This is the project's deepest technical risk and Alpha's gate.
3. **The record is the world's best honeypot.** Local-first and crypto-shredding are strong, but
   the threat model includes device compromise, coercion, subpoena, and abusive partners. A
   perfect memory of a person is a weapon against them. Needs first-class: duress modes,
   plausible partitioning, jurisdiction-aware retention defaults, and the humility to *not
   record* certain classes at all. This document under-specifies that; treat it as a V1 blocker.
4. **The adapter treadmill can kill a small team.** Every OS update and app redesign breaks
   sensors and adapters. The plugin ecosystem is the only durable answer, and ecosystems need
   scale that a young project lacks — a chicken-and-egg the "Linux of" framing romanticizes.
   MCP compatibility buys real leverage here (others maintain tool servers); still, expect 30%
   of engineering forever spent on the boundary.
5. **Platform vendors will sherlock the 80%.** Apple/Microsoft/Google will ship memory + on-device
   agents with distribution Nexus can't match. The defensible 20% is exactly the parts that
   conflict with their business models: user-owned record, vendor-neutral routing, verifiable
   forgetting, open protocol. Strategy must lean into what they *structurally cannot copy*,
   not compete on assistant quality — that race is unwinnable.
6. **The agent society may be premature.** Eleven agents is an org chart, and org charts are
   overhead. Honest reading: MVP needs four (Steward, Planner/Engineer merged, Critic, Archivist)
   and the rest are charters waiting for evidence. The blackboard protocol matters more than the
   cast; resist the theater of specialization until post-mortems demand a split.
7. **Personal evals are unsolved.** §9 leans on per-user eval sets harvested from history; for
   subjective tasks (tone, judgment) these are thin and noisy. Self-improvement could plateau —
   or worse, overfit to stale preferences (the system that keeps writing like 2026-you).
   Freshness-weighting helps; honestly, this is open research.
8. **Attention budgeting can hide failure.** A system optimizing for "fewer interruptions" learns
   to stay quiet — including about things it should have escalated. The briefing's diff format
   mitigates (silence is visible as an empty section), but incident review must audit
   *non*-interruptions too.
9. **Cost.** An ambient society consulting frontier models could cost real money per user per
   day. The skill-compilation economy (§3.5) and local routing are the answer, but the MVP —
   before skills accumulate — will be expensive precisely when it must prove value. Budget
   caps per task class are needed from day one.
10. **How a competitor beats this:** ship the delegation inbox + transactional workspace as a
    *feature of an existing IDE or OS* with zero-install distribution, skip the memory substrate,
    and iterate faster on the 20% of this document that users feel immediately. The moat only
    exists after 12+ months of record accumulation per user; the window before that is naked.
    Counter: get to compounding fast in one niche (developers) and publish the protocol so the
    ecosystem — not the product — is the thing competitors must beat.

**How V2 should differ, having learned from real users:** fewer agents, more skills; a
compensation framework for irreversibles; duress and partitioning as first-class; auto-commit
tiers for low-stakes sends; and, almost certainly, the discovery that the briefing and the
commitment tracker — the humble features — drive retention, while the autonomous execution that
motivated the project follows trust rather than leading it.

---

## Final Challenge

*"If you were the founder of a company worth $100B in 2035 because of this technology, what is
the one idea in this document that made that outcome possible?"*

Not the memory system — everyone built memory. Not the agent society — architectures converged.
Not even the ownership inversion, though that is where the *valuation* lives, because a
user-owned compounding record was the only asset in the industry that models couldn't
commoditize and platforms couldn't confiscate.

The idea that made it possible is smaller and earlier: **we made delegation safe before we made
it smart.** The transactional kernel — diffs instead of surveillance, earned authority instead of
settings, reversibility instead of apology — is what got the system *let in*. Everything
compounding (the record, the skills, the trust ledger, the ecosystem) was downstream of users
granting access they had rationally refused every other system. The moat was the record; the
record existed because of trust; trust was an architecture, not a promise.

The inequality in §0.3 was the whole company: while the industry raised E[value] and fought over
model quality that depreciated 40% a year, we spent a decade driving cost(verification) and
E[unrecoverable harm] toward zero — and those investments never depreciated at all.

---

STATUS: COMPLETE
