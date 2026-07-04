# NEXUS Spec — Blackboard Protocol

**Spec version: 0.1 (draft) · July 2026 · One of the three sacred interfaces (ARCHITECTURE.md §2)**

Defines how agents communicate: typed artifacts on a shared blackboard, no inter-agent chat
(DECISIONS.md D-007), all agents stateless with substrate-held state (D-006). Any conforming
agent implementation — any model, any runtime — can occupy any role.

---

## 1. Artifact envelope

Every artifact is an event (`bb.artifact/1`) in the episodic log, materialized into the
blackboard store. Common envelope:

```
artifact: {
  id:       ULID,
  type:     "Intent" | "Plan" | "Finding" | "Diff" | "Critique"
            | "DecisionMemo" | "SkillProposal" | "Incident",
  version:  type schema version,
  task:     root intent id this belongs to,
  author:   { role: "steward"|"worker"|"critic"|"archivist"|..., charter_version, model_id },
  created:  RFC 3339,
  state:    per-type lifecycle state (§3),
  refs:     [artifact/event ids],
  body:     type-specific payload (§2)
}
```

Artifacts are immutable; a revision is a new artifact with `refs` → predecessor. State
transitions are separate `bb.transition/1` events `{artifact, from, to, by, reason}` — so the
full history of who moved what, when, and why is replayable from the log.

## 2. Artifact types and contracts

Contracts are enforced by the runtime at write time (schema validation) — an artifact violating
its contract is rejected, not merely frowned at.

| Type | Body (essentials) | Contract (enforced) |
|---|---|---|
| **Intent** | user's ask (verbatim + normalized), context refs, deadline?, budget? | Only Steward creates from user input; verbatim text is preserved untouched. |
| **Plan** | DAG of steps: `{id, description, needs: [step ids], capability_requests, max_effect (R0–R3), checkpoint: bool}`; assumptions[]; success criteria | Every step declares `max_effect`; every R2/R3 step declares `checkpoint: true`; capability requests must be satisfiable by attenuation from the task grant. |
| **Finding** | question, answer, evidence: [event/source refs], confidence, caveats[] | `evidence` non-empty — a Finding without provenance is rejected (Principle 4). |
| **Diff** | transaction id, **effect_list: ledger seq range**, advisory_summary?, risk notes | `effect_list` is the approval object; `advisory_summary` MUST be marked advisory and may be absent, never the other way around (D-015). |
| **Critique** | verdict: `approve` \| `revise` \| `veto`; issues: `[{target: artifact id + step/line ref, severity, argument}]` | Each issue cites a specific step/line; a `veto`/`revise` with zero issues is rejected. Charter bias: refute, don't approve. |
| **DecisionMemo** | question, options: `[{label, consequences, cost}]`, positions: [role → option + argument], recommendation?, deadline? | ≥2 options; every dissenting role's position included verbatim; one screen (size cap). |
| **SkillProposal** | observed pattern (episode refs ≥3), draft program, capability manifest, proposed ladder stage | Capability manifest reviewed against minimality before `supervised` stage. |
| **Incident** | failure class: `retrieval` \| `planning` \| `execution` \| `specification`; narrative; refs; proposed remedy? | Filed automatically on abort/correction; links the outcome-labeling loop (ARCHITECTURE §12). |

## 3. Lifecycles

```
Intent:   open → planned → executing → review → done | cancelled
Plan:     draft → critiqued → approved → executing → done | aborted
Diff:     proposed → approved | rejected      (approval = user or trust-ledger authority)
Critique: terminal on creation
DecisionMemo: open → decided | expired
SkillProposal: observed → drafted → supervised → autonomous | retired
Incident: open → analyzed → resolved
```

Transition authority is role-scoped (e.g. only Critic moves Plan `draft → critiqued`; only user
or ledger authority moves Diff `proposed → approved`). The authority table ships with the
charter set and is validated at daemon start.

## 4. Charters (roles as data)

A role is a versioned data object, not code (D-012):

```
charter: {
  name, version,
  prompt_ref:          versioned prompt artifact,
  produces:            [artifact types it may create],
  consumes:            [artifact types + states it wakes on],
  capability_profile:  ceiling on what may be minted to this role's tasks,
  model_pref:          router hint (task class, privacy floor),
  attention_budget:    for roles that may address the user (Steward only, v1)
}
```

Charters are prompts-as-code (D-010): edits pass eval gates, canary, rollback. Adding a role =
adding a charter + authority-table rows; splitting Worker later is configuration, not surgery.

## 5. Scheduling semantics

- **Wake condition:** an agent instance wakes when an artifact matching its charter's `consumes`
  enters a matching state. Wakes are queued per task, processed serially per task (no two agents
  mutate one task's blackboard concurrently), parallel across tasks.
- **Leases:** a woken agent takes a lease on the (task, artifact) it processes; leases expire
  (default 10 min) so a crashed agent's work is re-dispatched, never recovered from memory.
- **Checkpoints:** Plan steps marked `checkpoint` persist step results as artifacts before the
  step is considered done — resumption is always from the blackboard.
- **Deadlock rule (normative):** two full Plan→Critique cycles without an `approve` MUST produce
  a DecisionMemo to the user. Disagreement is a feature; its product is the memo, not a stall.

## 6. Conformance

An agent implementation conforms iff: it writes only schema-valid artifacts within its charter's
`produces`; performs all world access through kernel capabilities; holds no state across wakes
beyond the blackboard; honors leases and transition authority. Conformance tests ship with the
spec (Phase 4 publishes them; they exist from Phase 1 as the runtime's own test suite).

## 7. Open questions (tracked for 0.2)

- Body encoding for large step results (inline vs. sidecar blob refs — align with record.md
  large-blob decision).
- Whether `Finding` confidence should be structured (per-evidence weights) or scalar — start
  scalar, revisit with Alpha consolidation data.
- Cross-task artifact references (a Finding reused by another task): allowed read-only via
  `refs`, but does reuse re-trigger ceiling computation? Leaning yes — trust travels with the
  artifact's own origin chain.

---

STATUS: COMPLETE
