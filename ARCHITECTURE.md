# NEXUS Architecture (Normative)

**Version 1.1 · July 2026 · Status: pre-implementation, post-review**

This is the *normative* architecture document. [DESIGN.md](DESIGN.md) is the vision/rationale
manifesto; [COGNITIVE_ARCHITECTURE.md](COGNITIVE_ARCHITECTURE.md) is the V2 theory layer that
sits above this file (its deltas are additive — see its Part X; where it assigns meaning and
this file assigns mechanism, both hold). Where documents disagree on mechanism, this file wins. Every major choice here has a numbered entry in
[DECISIONS.md](DECISIONS.md). Version 1.1 incorporates the Principal-Architect review that
simplified v1.0 (see D-012 through D-016).

---

## 1. What Nexus is, in one paragraph

A local-first **personal epistemic institution** (identity per D-018): a daemon that (a)
maintains a provenance-tracked record of the user's knowledge, beliefs, decisions, evidence,
and reasoning — positions admitted only with falsifiers, confidence earned only by scoring;
(b) mediates every action through a trust kernel that makes work inspectable, reversible where
possible, and gated where not; and (c) runs stateless operators that appraise, deliberate,
criticize, and consolidate over that record. The assistant is Client #1 of the institution and
its primary evidence pump. Models are replaceable peripherals; the record, the capability
model, the agent protocol, and the epistemic layer are the durable product.

## 2. The three sacred interfaces

Nexus is designed to evolve for 10+ years by keeping three interfaces stable and everything else
disposable. These are versioned specs with a syscall-boundary change discipline (additive by
default; breaking changes require a major version and a migration tool):

| Interface | Spec file (future) | Contents |
|---|---|---|
| **Record format** | `specs/record.md` | Episodic event schema, claim/provenance schema, content-addressing, encryption envelope, export format |
| **Capability model** | `specs/capability.md` | Capability structure, attenuation, effect-class taxonomy, ledger entry format |
| **Blackboard protocol** | `specs/blackboard.md` | Typed artifact schemas (Intent, Plan, Finding, Diff, Critique, DecisionMemo, SkillProposal, Incident), lifecycle states, agent charter format |
| **Epistemic layer** | `specs/record.md` §11 (elevated per D-018) | Bet schema + admission rule, resolution fold semantics (SCORE/RECONCILE), derivation events with context manifests, working-set events |

Everything not in a spec — storage engines, models, agent implementations, UI — is assumed
wrong-by-2030 and replaceable without user-visible loss. **Rule: no feature may create user
value that survives only in a non-spec component.**

## 3. System overview

```
 Surfaces:   nx CLI ─ Delegation Inbox ─ IDE ext ─ (later: intent bar, briefing, browser)
                │  intents in / reviews out
 Agents:     Steward ─ Worker ─ Critic ─ Archivist        (stateless; roles are charter DATA)
                │  read/write typed artifacts
 Substrate:  episodic log (append-only, content-addressed, per-item encrypted)
             + derived stores in ONE SQLite db (graph, claims, skills, plans,
               blackboard, trust ledger, indexes)  — all rebuildable from the log
                │  every world access
 Kernel:     capability manager ─ transaction manager ─ effect boundary ─ policy ─ ledger
                │  mediated I/O only
 Adapters:   WASM modules (fs, shell, git, later mail/browser), MCP-compatible boundary
 Spine:      event bus + scheduler (embedded, in-process), model router (static policy v1)
```

Single Rust daemon, single machine (v1). No services, no cluster, no message broker. Personal
scale is millions of events, not billions; the complexity budget goes to semantics, not
infrastructure (D-008).

## 4. Substrate

- **Episodic log** — ground truth. Append-only segments, content-addressed, each item encrypted
  with a per-item key wrapped by the user's root key (enables crypto-shredding, D-011). Events:
  observations, agent actions, user feedback. Never summarized at write time.
- **Derived stores** — one SQLite database: semantic graph (entities + claims with provenance
  edges, confidence, freshness, status), skill library, plan store, blackboard artifacts, trust
  ledger, and indexes (FTS5 lexical, sqlite-vec vector, temporal). Everything derived is
  rebuildable from the log; `nx rebuild` is a supported, tested operation from day one — this is
  the escape hatch that makes every derived-store schema decision reversible (D-013).
- **Retrieval** — a planner compiles memory queries into multi-index plans (temporal + graph +
  lexical + vector + freshness). v1 planner is heuristic; model-assisted planning later.
- **Consolidation ("night shift")** — idle-time pipeline: episodes → candidate claims →
  reconcile (corroborate/contradict) → decay → pattern detection → hypotheses. Gated feature:
  claim extraction quality is the project's top research risk and is measured before it is
  trusted (see ROADMAP Alpha gate).

## 5. Trust kernel

The only path to the world. Runs in the daemon; adapters get capabilities, never ambient access.

- **Capabilities** — object-capability model. Task-scoped, attenuable, time-boxed, revocable.
  Minted per task from the intent (e.g., "reply to Anna re: invoice" → read(thread), draft(outbox)),
  not from install-time grants (D-004).
- **Capability ceiling** — any task whose assembled context includes content below a trust
  threshold runs with a capped capability set for its lifetime. The kernel does **not** attempt
  to detect whether untrusted content influenced the model (undecidable); it confines what a
  possibly-influenced task can do (D-014). Origin/trust tags travel with every context item to
  make the ceiling computable.
- **Transactions** — reversible work (files, repo, local state) executes in copy-on-write
  workspaces. Irreversible effect classes (send, pay, publish, delete-remote) have **no code
  path** inside a workspace; they queue at the effect boundary and fire only at commit under
  ledger authority or explicit approval (D-005).
- **Approval binds to effects, not summaries** — the user approves the kernel's raw effect list
  (files changed, messages queued, capabilities exercised). Model-written semantic summaries are
  advisory presentation over that list, clearly marked as such (D-015). This closes the
  "approve the hallucination" hole found in review.
- **Trust ledger** — autonomy per (skill, context, consequence-class), expanded by track record,
  contracted by incident, always user-readable and user-editable. Merges conservatively
  (lowest autonomy wins) if multi-device ever makes it concurrent.
- **Effects ledger** — every mediated call, signed, queryable, replayable. The flight recorder.

## 6. Agents

**v1 ships four roles** (D-012). Roles are *charters* — versioned data (system prompt, artifact
contract, capability profile, model preference) — so adding or splitting roles later is
configuration, not architecture:

| Role | Charter |
|---|---|
| **Steward** | Intent triage, dispatch, attention budget; only role that speaks to the user unprompted. |
| **Worker** | Plans and executes (absorbs Planner/Researcher/Engineer/Envoy from DESIGN.md until post-mortem evidence demands a split). Always inside transactions. |
| **Critic** | Adversarial review with veto: plans before execution, effect lists before commit-proposal. Charter says refute, not approve. |
| **Archivist** | Consolidation, graph hygiene, retrieval quality, forgetting (absorbs Optimizer duties in v1). |

Warden (security review), Tutor, Scout, and the rest of DESIGN.md's society are **deferred
charters** — their duties exist as checklist items inside Steward/Critic/Archivist until scale
justifies them.

**Execution model:** agents are stateless processes — wake, read blackboard, do bounded work,
write artifacts, exit. Long tasks checkpoint at plan-step granularity with leases, so a crashed
or superseded agent is resumed by re-dispatch, never by in-memory recovery (D-006). No
inter-agent chat; only typed artifacts (D-007).

**Deadlock rule:** two Plan–Critique cycles without convergence → escalate to the user as a
one-screen DecisionMemo.

## 7. Model router

v1: static, user-editable policy (`router.json`) mapping task class → provider/model, with
outcome logging from day one. Learned routing is deferred until there is outcome data to learn
from (D-016). **Provider-agnostic by construction (D-017):** every vendor implements one
`Provider` trait behind a registry (Anthropic, OpenAI, Gemini, OpenRouter, Ollama, LM Studio,
any OpenAI-compatible endpoint via `openai-compat` + `base_url`); switching vendors is a config
edit, never an application-logic change. API keys resolve env-var override → sealed secrets
store (encrypted under the user's KEK); the default route table is `unconfigured` and refuses
with guidance — onboarding (`nx init` / `nx configure`) is where the user chooses. Privacy
tiers are enforced here: some record classes never leave the device (local models — Ollama/
LM Studio today, llama.cpp embedding for PII classification and retrieval planning later).

## 8. Adapters & plugins

WASM modules (wasmtime), zero default authority — every import is a passed capability handle.
MCP-compatible at the boundary so third-party tool servers plug in, wrapped in kernel mediation
(an MCP server never gains ambient authority by being installed). Adapters declare effect
classes for policy binding. v1 adapters: filesystem, shell, git. Sensors are explicit adapters
too — v1 ingests from fs/git/shell/IDE only; ambient watchers (browser, comms) arrive with
their consent and quarantine machinery, not before (D-013).

## 9. Event bus & scheduler

In-process durable pub/sub over the substrate (a table, not a broker). Sensors publish;
consolidation and Scout-checklist subscriptions consume; scheduler owns cron, deadlines,
follow-ups, and the attention budget (every proactive surface spends from a measured allowance;
non-interruptions are logged so silence is auditable too).

## 10. Sync & devices

**v1 is single-device by decision, multi-device by design** (D-013): the log is append-only and
content-addressed, hence trivially mergeable later; derived stores are rebuildable, hence never
need to sync at all. When multi-device ships (Beta+), it is E2E-encrypted blind relay +
last-writer-wins on the few genuinely mutable objects, CRDTs only if evidence demands them.

## 11. Surfaces

v1: `nx` CLI (do/ask/log/diff/trust/rebuild), the Delegation Inbox (Tauri), IDE extension.
Later: global intent bar, morning briefing, browser overlay, voice capture. Chat exists as the
drill-down debugger attached to artifacts, never as the front door.

## 12. Self-improvement loop (the only one)

outcome-labeled episodes → post-mortem classification (retrieval / planning / execution /
specification failure) → versioned change to a charter, skill, or policy → per-user eval gate →
canary → adopt or roll back. Prompts, charters, skills, and routing policy are code: versioned,
gated, rollbackable (D-010). Skills climb observed → drafted → supervised → autonomous; the
trust ledger is the visible integral of all learning.

## 13. Security posture (summary)

Threats ranked: (1) prompt injection → capability ceilings + effect boundary; (2) record theft →
per-item encryption, user-held root key, crypto-shredding; (3) coercion/duress → V1-blocker
work item: duress modes, partitioning, retention defaults (carried from DESIGN.md Part 12.3,
not yet designed — tracked in PROGRESS.md); (4) malicious plugins → WASM + zero default
capabilities; (5) model exfiltration → privacy tiers in the router. The TCB is the kernel and
ledger, kept small enough to audit; models are outside it by construction.

## 14. Known weaknesses accepted at v1.1

Carried forward knowingly (full critique in DESIGN.md Part 12): irreversible actions get gating,
not undo — compensation framework is V2; consolidation quality is unproven — Alpha gate;
personal evals for subjective tasks are thin — open research; adapter maintenance is a permanent
~30% tax — mitigated by MCP, not eliminated; the moat is thin until months of record accumulate —
mitigated by niche focus (developers) and early protocol publication.

---

STATUS: COMPLETE
