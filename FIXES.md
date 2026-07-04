# NEXUS — Fixes for SECURITY_AND_FAILURE_REVIEW (S1–S6)

**July 2026.** Implementation-engineer pass over the six findings in
[SECURITY_AND_FAILURE_REVIEW.md](SECURITY_AND_FAILURE_REVIEW.md). Architecture frozen: no new
concepts, no redesign. Every fix is local and reuses machinery the system already has
(derivation manifests, RECONCILE, provenance edges, the signed log, origin trust). All existing
behavior preserved; six regression tests added so no finding can silently reappear.

**Verification headline:** 57/57 unit tests pass (was 51; +6 regression). Demo 19/19, exit 0.
Each of the four *exploitable* findings (S1, S2, S3, S4) was re-run as its original attack and
confirmed closed.

---

## S1 — Automated settlement falsely retired unrelated cautions (HIGH)

**Root cause.** `inbox::approve`/`reject` settled *every in-scope live premortem*, and scope
matching was coarse (`general` matches everything; else path containment). "Similar task" was
never checked, so an unrelated approval met the premortem's falsifier by accident and silenced
safety cautions.

**Fix (causal + owned).** Settlement now touches only premortems that (a) the task **actually
cited** — recorded on the Diff artifact as `cited_premortems`, sourced from the derivation's
working set — and (b) the delegation loop **created** itself (origin adapter `runtime.inbox`).
A user's hand-placed caution is never auto-settled; an unrelated task cites nothing of the
caution and cannot settle it.

**Files changed.** `crates/runtime/src/pipeline.rs` (record `cited_premortems` on the Diff);
`crates/runtime/src/inbox.rs` (new `settleable_premortems` = cited ∩ live ∩ flow-born; `approve`
and `reject` use it; `InboxItem.cited_premortems` added); `crates/substrate/src/bets.rs`
(`BetView.origin_adapter` exposes the placing adapter).

**Tests.** `runtime::loop_tests::s1_unrelated_approval_does_not_settle_manual_caution` (the exact
A5 attack: approving a haiku leaves a general-scoped "deleting prod data is catastrophic"
premortem live). Existing loop tests still assert correct settlement of genuinely-relied-upon
premortems.

---

## S2 — Forgetting evidence did not retract beliefs built on it (HIGH)

**Root cause.** `Substrate::forget` shredded the episode key and tombstoned it but never walked
the provenance edges from surviving bets — the episode→belief propagation specs/record.md §7
promised was unimplemented. Beliefs survived `live` with dangling provenance (epistemic + privacy
leak).

**Fix.** After shredding episode `E`, `forget` retracts every live bet whose provenance contains
`E` and whose evidence is now *entirely* forgotten, via the existing `bets::resolve(Retracted)`
— so RECONCILE cascades to dependents automatically. Beliefs with other surviving evidence are
untouched.

**Files changed.** `crates/substrate/src/lib.rs` (forget→retract loop);
`crates/substrate/src/bets.rs` (`BetView.provenance` exposes the bet event's evidence refs).

**Tests.** `bets::tests::s2_forgetting_evidence_retracts_the_belief` (sole-evidence belief →
`retracted`, dependent → `unjustified`); `bets::tests::s2_belief_with_other_evidence_survives_forget`
(guard: partial forget does not retract).

---

## S3 — External-trust content laundered into a trusted belief (HIGH)

**Root cause.** Consolidation placed every bet with a hardcoded `Trust::Derived` origin,
dropping the `external` taint of untrusted evidence. Downstream, the capability ceiling and
retrieval saw a trusted `derived` belief.

**Fix.** A consolidated bet inherits the **minimum trust of its provenance episodes**, capped at
`Derived` (never elevated above derived, never above its worst source). External evidence →
external belief → the ceiling and retrieval see it correctly.

**Files changed.** `crates/runtime/src/consolidate.rs` (per-episode trust map; `min` over
provenance for the placed bet's origin).

**Tests.** `consolidate::tests::s3_external_evidence_yields_external_belief` (external note →
`External`-tagged belief; user note → `Derived`, capped).

---

## S4 — Transaction authorization was forgeable via plaintext meta (HIGH, local-fs)

**Root cause.** The capability and target authorizing a commit lived in plaintext, unsigned
`txns/<id>/meta.json`. Every commit guard read from that untrusted file, so a consistent forge
(target + scope into a snapshot-matching dir) redirected the commit — signed as legitimate.

**Fix.** The authoritative target and full capability are now written into the **signed
`txn.begin/1` event** (`capability_full`). `commit` re-derives them from the record and refuses
if the on-disk `meta.json` disagrees ("does not match the signed record — tampered"). `meta.json`
is demoted to a non-authoritative cache; forging it changes nothing.

**Files changed.** `crates/kernel/src/txn.rs` (`begin` logs `capability_full`; new
`authoritative()` reads the signed begin event; `commit` cross-checks target + capability);
`crates/kernel/src/capability.rs` (`Capability` derives `PartialEq, Eq` for the check).

**Tests.** `txn::tests::tampered_meta_capability_is_rejected` (forged R3/expiry → refused);
`txn::tests::tampered_meta_target_cannot_redirect_commit` (redirect to victim dir → refused, no
write escapes). Replaces the former `expired_capability_blocks_commit`, whose meta-edit mechanism
*was* this attack; capability expiry remains covered by `capability::tests`.

---

## S5 — No contradiction detection between independent beliefs (MEDIUM)

**Root cause.** Contradiction handling existed only along declared premise links; two
independently-placed, directly-opposed beliefs both stayed live forever, unsurfaced.

**Fix (surface, never resolve).** A read-time, lexical detector `bets::contradictions` pairs live
bets with identical content but opposite negation parity (e.g. "X is Friday" vs "X is never
Friday"). Surfaced as a `⚠` warning at `nx bet place` and a `[CONTRADICTS <id>]` annotation in
`nx bets` — consistent with the stated design (surface, don't auto-resolve). Semantic
contradiction stays an open problem (RESEARCH_NOTES); nothing is stored (derived, rebuildable).

**Files changed.** `crates/substrate/src/bets.rs` (`negation_signature`, `contradictions`);
`crates/nx/src/main.rs` (warn on place; annotate in `bets`).

**Tests.** `bets::tests::s5_contradiction_is_surfaced` (negation pair detected, unrelated bet
not; both remain live).

---

## S6 — Admission rule is syntactic; degenerate falsifiers pass (LOW/MEDIUM)

**Root cause.** Admission checks only that a falsifier is non-empty; a degenerate `"."` or
unfalsifiable-in-practice falsifier satisfies the letter of the rule. This is a defensible
boundary (falsifier quality is not machine-decidable) that the prose overstated.

**Fix (documentation honesty, per the review's own recommendation).** specs/record.md §11.1 now
states plainly that admission guarantees a falsifier is *present*, not *good*, and that the
mechanical-settlement razor (D-021) — not admission — is what forces falsifiers to be real
(specced, not yet shipped). No code change; a regression test locks the boundary so a future
"reject short falsifiers" hack can't silently overclaim.

**Files changed.** `specs/record.md` (§11.1 boundary paragraph).

**Tests.** `bets::tests::s6_degenerate_falsifier_is_accepted_documented_boundary`.

---

## Verification performed

- **`cargo test`** — 57/57 pass (kernel 12, runtime 25, substrate 20). Was 51 before this pass;
  +6 regression tests (S1, S2×2, S3, S5, S6) plus S4×2 replacing one repurposed test.
- **`bash demo/prove-it.sh`** — 19/19 assertions, exit 0; `DEMO.md` regenerated.
- **Attack replay** — the four exploitable findings re-run as their original attacks:
  - S1: unrelated haiku approval → caution stays live, no auto-settle. ✅
  - S2: forget sole evidence → belief in the loss ledger (`retracted`). ✅
  - S3: external note → belief tagged `External/P1`. ✅
  - S4: full consistent meta forge → "does not match the signed record", victim untouched. ✅
- No regressions in the existing belief-lifecycle, transaction, provider, or crypto suites.

---

STATUS: COMPLETE
