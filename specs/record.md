# NEXUS Spec — Record Format

**Spec version: 0.2 (draft) · July 2026 · One of the three sacred interfaces (ARCHITECTURE.md §2)**

*0.2 is additive over 0.1: §11 adds Stratum 1 (bets), resolutions, derivation events, and
working-set events per COGNITIVE_ARCHITECTURE.md Parts III–IV. Nothing in §1–§10 changed.*

Change discipline: additive within a major version; unknown-field preservation is mandatory for
all readers; breaking changes require a major version bump and a shipped `nx migrate` tool.
This spec defines what a user's record *is*, independent of any Nexus implementation. A record
written by Nexus v1 must be readable by tools written in 2036 from this document alone.

---

## 1. Concepts

- **Event** — one immutable observation, action, feedback, or system fact. The episodic log is
  an append-only sequence of events. Events are never edited or deleted; corrections are new
  events, and forgetting is key destruction (§7).
- **Claim** — a derived belief about the world ("prefers small PRs"), always carrying provenance
  edges to the events that justify it. Claims live in derived stores but are *also* recorded as
  events (schema `memory.claim/1`) so every derived store is rebuildable from the log alone.
- **Segment** — an append-only file holding framed, encrypted events.
- **Keyring** — the mapping from event id to its wrapped per-item key. Destroying a wrapped key
  is forgetting (crypto-shredding, DECISIONS.md D-011).

## 2. Event schema

Canonical encoding: CBOR (deterministic mode) for the frame; JSON for export. Field names are
normative.

| Field | Type | Meaning |
|---|---|---|
| `id` | ULID (26-char Crockford base32) | Unique, time-ordered identifier. |
| `ts` | RFC 3339 UTC, nanosecond precision | Event time as observed by the daemon. |
| `kind` | enum | `observation` \| `action` \| `feedback` \| `system` |
| `schema` | string | Namespaced payload type + version, e.g. `fs.write/1`, `memory.claim/1`, `ledger.effect/1`. Payload schemas are registered in `schemas/` (additive registry). |
| `origin` | object | Provenance tag, §3. Required on every event. |
| `privacy` | enum | `P0`–`P3`, §4. |
| `refs` | array of ULID | Events this event responds to, derives from, or supersedes. |
| `body_hash` | 32-byte BLAKE3 of plaintext body | Plaintext identity — lets provenance and dedup logic compare content without decrypting. |
| `body` | encryption envelope, §5 | The payload itself. |

Rationale for `body_hash` alongside content addressing: the *storage* address (§6) is over
ciphertext and changes if an item is re-encrypted; `body_hash` is the stable plaintext identity
used by provenance edges and consolidation.

## 3. Origin (trust) tags

```
origin: {
  adapter: string          // e.g. "fs", "git", "mail.imap", "agent.worker"
  actor:   string          // user id, agent role@charter-version, or remote party
  trust:   enum            // "user" | "local" | "derived" | "external"
}
```

`trust` is load-bearing for the capability ceiling (specs/capability.md §5): `user` = the human's
own input; `local` = this machine's own instruments (fs, shell); `derived` = produced by Nexus
reasoning/consolidation; `external` = content that originated outside the user's authority (web
pages, received email). Adapters MUST tag honestly; an adapter that mislabels external content is
a security defect, not a bug.

## 4. Privacy classes

| Class | Meaning | Binding consequence |
|---|---|---|
| `P0` | Effectively public | No routing restriction |
| `P1` | Personal | May be sent to models per user's router policy |
| `P2` | Sensitive | Only to explicitly allow-listed model endpoints |
| `P3` | Never leaves device | Local models only; excluded from sync and export-by-default |

Classification is assigned at ingestion (local model + adapter defaults), user-overridable, and
recorded — a reclassification is a new `system` event referencing the original.

## 5. Encryption envelope and key hierarchy

- **Cipher:** XChaCha20-Poly1305 (AEAD). Chosen for nonce-misuse margin (192-bit nonces) and
  broad, audited library support; envelope carries an algorithm id so the cipher is replaceable.
- **Per-item DEK:** every event body is encrypted under its own random 256-bit data-encryption
  key. **AAD** is the serialized event header (all fields except `body`), cryptographically
  binding header to payload — a header cannot be swapped onto another body.
- **Key wrapping:** each DEK is wrapped (AEAD) by the user's root KEK and stored in the keyring,
  keyed by event `id`. The KEK derives from the user's passphrase/hardware key via Argon2id and
  never touches disk unwrapped.
- **Envelope format:** `{ alg: "xchacha20poly1305/1", nonce, ciphertext }`.

Tradeoff accepted: a keyring entry per event costs storage and one extra unwrap per read.
This is the price of item-granular, provable forgetting (§7) and it is worth it.

## 6. Segments and content addressing

- Segment file: `log/segment-<ulid>.nxl`. Header: magic `NXL1`, format version, created-at.
- Frame: `u32 length ‖ frame bytes ‖ 32-byte BLAKE3 checksum of frame bytes`. Frame bytes are
  the CBOR event (header + envelope). Corruption detection is per-frame; a torn final frame
  (crash mid-append) is truncated on recovery, never repaired in place.
- Segments roll at 64 MiB and are immutable once rolled. `fsync` on every `action`-kind event
  and on ledger events; batched (≤100 ms window) for high-volume observations.
- **Content address** = BLAKE3 of the frame bytes. This is the storage/sync identity: because
  segments are append-only and frames immutable, future multi-device sync is set-reconciliation
  over content addresses (DECISIONS.md D-013 — "multi-device by design").

## 7. Forgetting (normative behavior)

`forget(id)` MUST: (1) destroy the wrapped DEK for `id` from the keyring and all keyring
replicas/backups; (2) append a `system` tombstone event `memory.forget/1` referencing `id`
(by id and `body_hash` only — never content); (3) on next consolidation, strip provenance edges
pointing at `id`, whereupon dependent claims lose evidence and decay or die per their status.
Backups store ciphertext and keyring **separately**, so shredding the keyring entry kills all
copies. The system MUST be able to enumerate tombstones — "what have I forgotten" is answerable;
"what was it" is not.

## 8. Claims (derived-belief schema) — **DEPRECATED**

> **Deprecated in 0.2 (D-019):** superseded by bets (§11), the sole reliance representation —
> a belief without falsifiers is inadmissible, and claims have no falsifier field. No
> implementation has ever emitted `memory.claim/1`; the schema is retained here only for
> registry completeness. New code MUST NOT write claims.

Recorded as `memory.claim/1` events; materialized into the graph store.

```
claim: {
  id, entity, predicate, object,           // subject–predicate–object over registered entities
  status:  "observed" | "inferred" | "hypothesized" | "user_stated" | "contradicted",
  confidence: 0.0–1.0,
  provenance: [event ids],                 // REQUIRED, non-empty (Principle 4: no claim without evidence)
  created, last_verified, expires?,        // freshness
  supersedes?: claim id                    // deprecation chain
}
```

Normative rules: `user_stated` outranks `inferred` on conflict; conflicts set both claims to
`contradicted` and queue a reflection item — resolution is never silent; a claim whose last
provenance edge is tombstoned MUST decay to deletion.

## 9. Export / import

`nx export` emits a directory: `manifest.json` (spec version, counts, hash tree), `events.jsonl`
(decrypted events, one JSON object per line, `P3` items excluded unless `--include-p3`),
`claims.jsonl`, `schemas/` (copies of all payload schemas used). `nx import` MUST round-trip
losslessly (tombstoned items excluded — they no longer exist). The export format is the
user-ownership guarantee (Principle 1) and is versioned with this spec.

## 11. Version 0.2 additions — bets, resolutions, derivations (S1 of the memory strata)

### 11.1 Bets (`memory.bet/1`)

A bet is a position the system may *rely on*. **Admission rule: no falsifiers, no bet** — a
statement without at least one observable killing condition may exist as an episode but can
never be relied upon (COGNITIVE_ARCHITECTURE Part IV). Enforced at append.

```
bet: {
  statement,                       // the position
  scope,                           // where it applies (context predicate; free text in v0)
  kind:   "belief" | "prediction" | "premortem",
  stakes: "R0" | "R1" | "R2" | "R3",   // consequence class if acted on and wrong (reuses capability taxonomy)
  falsifiers: [string, ...],       // REQUIRED, non-empty: observable conditions that kill it
  horizon?: timestamp,             // when it must be re-earned or expires
  premises: [bet event ids],       // justification links (for RECONCILE)
}
```

Provenance = the event's `refs` (episode ids): no reliance without evidence. The bet's id is
its event id. Bets are the **sole** reliance representation — claims (§8) are deprecated per
D-019; consolidation emits bets directly, through the admission rule.

**Scope of the admission rule (honest boundary, per SECURITY_AND_FAILURE_REVIEW S6).** Admission
guarantees a falsifier is *present*, not that it is *good*: a degenerate falsifier (`"."`) or an
unfalsifiable-in-practice one (`"the heat death of the universe occurs"`) satisfies the letter of
the rule. Falsifier *quality* is not machine-decidable and is deliberately not claimed here. What
forces falsifiers to be real is the **mechanical-settlement razor** (D-021): a falsifier with no
mechanical settlement source is flagged `interpretive` and scored in a separate calibration
bucket. Until the razor ships, treat presence — not quality — as the enforced guarantee.

### 11.2 Resolutions (`memory.resolution/1`) — SCORE and RECONCILE

Confidence is never asserted; it is the fold of resolutions:

```
resolution: { bet: id, outcome: "held" | "falsified" | "expired" | "retracted" | "unjustified",
              note, by }
```

Fold semantics (normative): `held` increments the score and leaves the bet live; `falsified` /
`expired` / `retracted` are terminal; `unjustified` flags the bet (retrievable only with an
explicit warning label). **RECONCILE rule:** when a bet resolves `falsified` or `retracted`,
every transitive dependent (via `premises`) MUST receive an `unjustified` resolution — a
conclusion never silently outlives its premises. Crypto-shredding a bet's evidence (§7)
triggers the same propagation.

### 11.3 Derivation events (`memory.derivation/1`) — the atom of machine reasoning

Every model judgment records what produced it:

```
derivation: { operator, inputs: [record ids], output: record id,
              reasoner: model id, charter: "name@version",
              manifest: BLAKE3 hex of the exact working set }
```

The **context manifest** makes every judgment attributable and replayable: post-mortems can
distinguish wrong-belief from wrong-working-set; memories earn credit/blame for outcomes they
were in context for; model swaps are regression-tested by replaying stored working sets.

### 11.4 Working-set events (`memory.workingset/1`)

The manifest's preimage: the full compiled context, stored encrypted like any body, referenced
by the derivation event. This is what makes replay (RESEARCH_NOTES E-5) possible. Privacy
class inherits the *highest* class of any included item.

## 12. Open questions (tracked for 0.3)

- Keyring storage structure and its own backup/rotation story (KEK rotation = rewrap, not
  re-encrypt — confirm at implementation).
- Large-blob handling (video, big files): sidecar content-addressed blob store vs. in-frame;
  leaning sidecar with the event holding the address.
- Deterministic CBOR profile pin (RFC 8949 §4.2 core deterministic encoding) — confirm library
  support in Rust before freezing.

---

STATUS: COMPLETE
