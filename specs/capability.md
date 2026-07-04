# NEXUS Spec — Capability Model

**Spec version: 0.1 (draft) · July 2026 · One of the three sacred interfaces (ARCHITECTURE.md §2)**

Defines how authority is represented, granted, attenuated, capped, exercised, and recorded.
Everything here follows from two decisions: object capabilities with no ambient authority
(DECISIONS.md D-004) and confinement-not-detection for untrusted context (D-014).

---

## 1. Effect-class taxonomy (load-bearing)

Every operation an adapter exposes MUST declare exactly one effect class. The taxonomy is part
of the spec because the transaction manager, the policy engine, and the trust ledger all key on
it (D-005).

| Class | Name | Definition | Examples | Kernel behavior |
|---|---|---|---|---|
| **R0** | Observe | No world mutation. Privacy-relevant but not safety-relevant. | read file, list threads, fetch URL | Executes immediately under capability scope; logged. |
| **R1** | Reversible | Mutation the *kernel itself* can fully restore from snapshot, without adapter cooperation. | workspace file writes, local db changes, git worktree ops | Executes speculatively inside the transaction; abort = restore. |
| **R2** | Compensable | External mutation with a **registered compensator** — an inverse operation the adapter implements and the kernel can invoke. Compensation is not undo: the effect may have been observed while live. | create/delete calendar event, move cloud file, add label | Executes at commit (not inside speculation); compensator recorded in ledger; abort-after-commit = run compensator + notify user. |
| **R3** | Irreversible | No reliable compensation exists. | send email/message, payment, publish, delete-without-trash, sign/contract, identity or permission changes | **No executable path inside a task.** Queues at the effect boundary; fired by the kernel only at commit, under trust-ledger authority or explicit approval. |

Normative rules:
- An adapter operation without a declared class is rejected at plugin load.
- An operation claiming R2 without a registered, tested compensator is treated as R3.
- Classification is per-operation, not per-adapter (mail: read=R0, label=R2, send=R3).
- The taxonomy is closed; proposals for new classes are breaking changes (major bump). Rationale:
  policy simplicity is a security property — four classes fit in a human's head.

## 2. Capability structure

```
capability: {
  id:            ULID,
  task:          task/intent id it was minted for,
  adapter:       target adapter id,
  ops:           [operation names],
  scope:         resource constraint (path globs, thread ids, entity refs, URL patterns),
  max_effect:    R0 | R1 | R2 | R3,        // ceiling for this capability
  constraints:   { rate?, bytes?, count?, time_box? },
  expiry:        timestamp (mandatory — no immortal capabilities),
  parent:        capability id | null,      // attenuation chain
  revoked:       bool
}
```

Capabilities are unforgeable references held by the kernel; adapters and agents receive opaque
handles. There is no install-time grant — installation confers *zero* authority (D-004, D-009).

## 3. Minting

The kernel mints capabilities from an approved Plan's declared step requirements: default-deny,
narrowest scope that satisfies the step, mandatory expiry, all mints logged. The Delegation
Inbox renders minted scope with the plan so the user approves *authority*, not just intent.
Minting heuristics (intent → scope inference) are implementation, not spec; the structure and
the default-deny rule are spec.

## 4. Attenuation and delegation

A holder may derive a child capability that is ⊆ parent on **every** axis (ops, scope,
max_effect, constraints, expiry). Delegation to a subtask happens *only* by attenuation — a
subtask can never hold broader authority than its parent. Revoking a capability revokes its
entire descendant chain. This is how low-privilege subtasks are built (see §5, partitioning).

## 5. Capability ceilings (context-trust confinement)

Per D-014, the kernel does not attempt to detect whether untrusted content influenced a model.
Instead, every context item assembled for a task carries the `origin.trust` tag from the record
(specs/record.md §3), and the task's effective authority is capped by the *minimum* trust
present in its context:

| Lowest trust in assembled context | Ceiling applied to the task |
|---|---|
| `user` / `local` | Task's minted capabilities as-is (up to R3-queueing) |
| `derived` | Max R2; R3 queueing permitted only for effect classes the trust ledger marks autonomous |
| `external` | **Max R1**; no R3 queueing; write scope restricted to the task workspace; no capability may target comms adapters |

Rules:
- The ceiling is computed at every context assembly and is monotonic downward for the task's
  lifetime (reading untrusted content mid-task lowers the ceiling; nothing raises it).
- The ceiling at exercise time is recorded in every ledger entry (§6) — auditable after the fact.
- **Partitioning pattern** (the sanctioned workaround): a parent task spawns a low-privilege
  subtask (attenuated to R0/R1) to read external content and return a summary; the summary
  enters the parent's context with `origin.trust = derived` (origin: the subtask), so the parent
  retains R2/queueing authority. This trades fidelity for authority explicitly and visibly —
  never silently.

## 6. Effects ledger entry

Every mediated operation, without exception, appends:

```
ledger.effect/1: {
  seq:            monotonic u64,
  ts:             RFC 3339,
  task:           task id,
  capability:     capability id,
  op:             operation name,
  args_hash:      BLAKE3 of canonical args,
  effect_class:   R0–R3,
  ceiling:        ceiling in force at exercise time,
  result_hash:    BLAKE3 of result | null,
  txn:            transaction id | null,
  sig:            Ed25519 signature by the daemon ledger key
}
```

Ledger entries are events in the episodic log (append-only, per-item encrypted like everything
else). The signature chain makes the flight recorder tamper-evident. **Approval binds here:**
what a user approves at commit is the ledger-derived effect list, never a model summary (D-015).

## 7. Revocation

Revocation is immediate and synchronous: the kernel checks `revoked` and expiry on every
exercise, not at mint time. Task completion auto-revokes all capabilities minted for it.
A user-initiated "stop everything" revokes all live capabilities and aborts open transactions —
this MUST be a single, always-available operation (`nx stop --all`).

## 8. Open questions (tracked for 0.2)

- Compensator testing discipline: how does an adapter *prove* its R2 compensator works
  (conformance harness in the plugin SDK)?
- Scope grammar: one unified resource-pattern language vs. per-adapter scope types — leaning
  per-adapter types validated against a registered schema, since a unified grammar tends to
  become stringly-typed globs for everything.
- Whether R3 queue entries should themselves expire (a held email drafted Tuesday may be wrong
  by Friday) — leaning yes, with expiry surfaced in the inbox.

---

STATUS: COMPLETE
