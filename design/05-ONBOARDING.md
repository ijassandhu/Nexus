# NEXUS Desktop — Onboarding: The First Five Minutes

**Version 1.0 · July 2026**

Goal: the user *experiences* belief → evidence → confidence → contradiction → settlement →
updated belief, on their real record, before they read anything. Nothing is simulated: every
step calls the real backend, using the **mock provider** (`providers.rs::mock` — deterministic,
offline, real code paths) until the user configures a real one. The onboarding writes real
events into the user's actual record — their first beliefs are not throwaway tutorial data;
they are the founding entries of a permanent ledger, and the copy says so.

The tone is a guided lab induction, not a product tour. No tooltips-on-everything, no
character mascot, no skippable video. Six acts, each ending with something the user *did*.

---

## Act 0 — The key ceremony (~40 s)

`nx init` as UI. One screen, `type-display`:

> **This record will be yours.**
> Everything NEXUS remembers is encrypted per-item on this machine. The passphrase derives
> the only key. There is no recovery — that is a feature, and it is the last time we will
> say "trust us."

Passphrase (with strength meter), confirm, create. The status bar appears for the first time
with `● verified · 1 event` — the record exists, and its integrity readout is already on.
Provider setup is *deferred*: "NEXUS runs on a deterministic stub until you attach a model.
Knowledge belongs to the record, not the model — you can attach or swap one anytime."
(This ordering is deliberate: the record precedes the AI, structurally and now
experientially.)

## Act 1 — A belief that cannot lose is refused (~45 s)

The Beliefs screen, empty state, with one framed exercise:

> Try to make NEXUS believe something unfalsifiable. Type any statement. Leave "Dies if"
> empty. Submit.

The backend refusal renders verbatim, mono, unstyled-error red:

> `no falsifiers, no bet: a position that cannot say what would kill it may be an episode,
> never a reliance`

Caption: **"This is the admission rule. It is enforced at the only write path. Everything
else you'll see follows from it."** Then the user adds a falsifier and the bet places —
their first belief, live, with its Dies-if line and a FoldTrack sitting at the 0.5 prior
labeled "unscored — confidence must be earned."

## Act 2 — Evidence becomes belief (~60 s)

Prompt: "Tell the record three things about how you work — plain notes, no rules." User
appends three notes (real `dev.note/1` observations; they appear in the Record screen as
they type). Then one button: **Consolidate**.

The mock archivist runs (real consolidation pipeline). The Candidates review appears:
extracted candidate beliefs, each citing the user's actual note events (EvidenceRefs with
◆ user trust glyphs) — and, crucially, the mock's deliberately unfalsifiable candidate is
shown **rejected, with the rule quoted**. The user watches the admission rule filter a
machine the same way it filtered them. Caption: "Same rule, no exceptions — not even for
the AI."

## Act 3 — Confidence is earned, and death cascades (~60 s)

Two pre-staged exercise beliefs are placed *by the user* from a template ("A", and "B —
premised on A"). The user resolves A as **held**: the ✓ notch pops, the dot slides right.
"Earned, never asserted."

Then: "Now something A predicted fails. Record it." User resolves A **falsified** with a
note. The cascade animation runs: A's glyph flips ✕, and B — untouched by the user — dims
to ◌ unjustified before their eyes. Caption: **"RECONCILE: a conclusion never silently
outlives its premises."** The Graveyard tab badge appears: 1. Opening it shows A with
**killed by:** the user's own note.

## Act 4 — Delegation: preview, reject, and the memory it mints (~90 s)

"Now let it work." The user runs a real task (mock worker) against a scratch folder the
onboarding created inside the record's data dir. The Tasks trail forms; the Inbox badges.

In the Inbox: the effect list (one file, hashes), the collapsed advisory, critic verdict.
The exercise instructs the user to **reject** it, with a reason. On submit, the consequence
chain animates: txn aborted → Incident filed → **a premortem is minted from your words**
(the new caution chip slides into Beliefs). Caption: "Rejection is training signal. Your
'no' is now memory with a falsifier."

Immediately: "Run a similar task." In the new task's plan, the caution appears —
*heeded* (the mock worker surfaces cautions verbatim; real code path). The user approves
this one. **Settlement** fires: the premortem's falsifier ("a similar task is later
approved without edits") underlines, stamps *settled — falsifier met mechanically*, and
retires. The full loop — fear minted from rejection, fear retired by approval — completed
by the user's own two verbs.

## Act 5 — The instrument panel (~30 s)

The Overview assembles itself from what the user just did — live beliefs, one grave, one
settled caution, resolutions feed. Final card:

> **You now know the whole system.** Everything else is scale.
> · Beliefs — what the record relies on, and what would kill each one
> · Record — every event, encrypted, yours; forgettable with proof
> · Inbox — nothing changes without your yes
> · Graveyard — ask any AI: "show me a belief you lost." This one answers.
>
> [ Attach a real model ] · [ Start using NEXUS ]

"Attach a real model" opens Reasoners → provider setup (real onboarding.rs flow: provider,
model with suggested default, key sealed with a visible "sealed ✓ — never stored in
plaintext"). Skipping is fine; the stub keeps working.

---

## Design rules for onboarding

- **Never fake:** every animation is triggered by a real backend event the user caused.
  If consolidation places nothing, the UI says so and offers better example notes — no
  canned success.
- **The user does every action.** The system never demonstrates to a spectator.
- **Each act ends in a durable artifact** the user can revisit (their beliefs, their grave,
  their incident) — the tutorial *is* their record's first chapter.
- **Exit anytime** (Esc): acts are re-runnable from Settings → "Induction," and the
  exercise beliefs are ordinary bets the user may resolve or forget like any other.
- Total target: ≤ 5 minutes, ≤ 800 words of copy on screen across all acts.
