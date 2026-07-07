# NEXUS Desktop — The Induction

**Version 1.0 · July 2026 · The first five minutes, choreographed. Supersedes
`05-ONBOARDING.md`'s act structure; its design rules (never fake, user does every action,
each act leaves a durable artifact, exit anytime) still bind.**

---

## 0. The thesis of the choreography

Nobody leaves believing they saw the future because a product *showed* them features.
They leave believing it when the system, in real time and in response to *their* words:

1. **refuses them** (judgment),
2. **remembers with receipts** (evidence),
3. **changes its mind in public** (honesty),
4. **acts with fear and gets over it mechanically** (accountability).

So the induction is not a tour and not a dashboard. It is a single continuous scene on the
Standing — the field starts empty and, in five minutes, fills, rises, refuses, dies,
cascades, fears, and settles, every beat caused by the user and executed by the real
backend. The narrator (a small "director" card docked above the status bar) speaks in the
system's register — about NEXUS in third person, never "I" — and never says anything the
record cannot prove.

**The honesty rule of the choreography:** the director may choose *when* NEXUS takes
initiative (it triggers consolidation the moment the third note lands, the way the
night-shift scheduler will), but every behavior — refusal, extraction, citation, cascade,
caution-heeding, settlement — is the unmodified backend acting on the user's real record.
If the backend does something unexpected (nothing extracted, verdict differs), the director
narrates what actually happened. The scene is directed; the actor is never dubbed.

## 1. The score — five movements, ~5:00

### MOVEMENT 0 · YOURS — *0:00–0:30*
**Stage:** the empty field. One ground line. Status bar already reads `● verified · 0 events`.
**Director:**
> **This record is yours.**
> No account. No server. Your passphrase derived the only key — there is no recovery, and
> that is a promise, not a flaw. Right now NEXUS believes nothing: it is only allowed to
> hold positions that can lose.
> `[ Begin the induction ]` · skip

*Why it plays as the future:* the absence is the demo. Every AI product opens by performing
knowledge it doesn't have; this one opens by proving it will not pretend.

### MOVEMENT I · REFUSAL — *0:30–1:30*
**Beat 1:** director asks: *"Tell it something you believe about your work."* One input.
The director submits it **without falsifiers** — and the backend refuses, on stage,
verbatim:
> `no falsifiers, no bet: a position that cannot say what would kill it may be an episode,
> never a reliance (specs/record.md §11.1)`

**Beat 2:** director:
> It refused you. Every position here must name what would kill it. So — what would prove
> you wrong?

Second input appears. On submit, the card **rises from the input to the 0.50 prior line**,
its DIES IF attached, labeled *unscored — earns altitude by surviving*.

*The intelligence beat is the refusal.* Being told "no, and here is the rule" by a memory
system in the first minute reframes everything that follows.

### MOVEMENT II · WITNESS — *1:30–2:45*
**Beat 1:** director: *"Now give it evidence. Three observations about how you work —
plain notes."* (input placeholder rotates honest hints: `I prefer…`, `we require…`,
`yesterday I noticed…`). Each note lands in the record — the status-bar event count ticks.

**Beat 2 — the initiative moment.** The instant the third note lands, the director does not
wait to be asked:
> NEXUS is appraising what you told it…

Real consolidation runs (mock archivist if no provider is attached; the real one if it is).
Candidate beliefs **rise into the field**, each with evidence chips citing the user's own
notes; and — the crucial line — any candidate that failed admission is shown struck out
with the rule quoted:
> From your notes it drew N positions — each citing the note it came from. One candidate it
> refused **itself**: *[reason, verbatim]*. Same rule, no exceptions — not even for the AI.
> Click any evidence chip: the exact note, decrypted on view.

*Remembering-with-evidence beat:* provenance is one click, and the system is seen holding
itself to the law it applied to the user.

### MOVEMENT III · THE LOSS — *2:45–3:30*
**Director:** *"Beliefs are bets. One of yours is now wrong — click the belief that
deserves to die, tell it what happened, and resolve ✕ falsified."* (field spotlit; the
inspector's resolve verbs do the work).

On the kill: the card **falls through the ground**; if anything stood on it, the cascade
pulses violet and sinks — and the director names it:
> It changed its mind in public. {N dependent positions dropped with it — nothing silently
> outlives its premises.} Below the ground: ask any AI *"show me a belief you lost."*
> This one answers — with what killed it, in your words.

### MOVEMENT IV · FEAR & SETTLEMENT — *3:30–5:00*
**Beat 1 — work.** Director: *"Now let it work. Nothing real happens without your yes."*
One button delegates a small task (`add a config file for logging`) into a transaction.
The Inbox badges; director points there: *"Review the effect list — hashes, not prose.
Then reject it, and say why. Your no is not a dead end here."*

**Beat 2 — fear (on rejection).** The moment the rejection lands:
> From your reason NEXUS minted a fear — a premortem, standing in the field with its own
> falsifier. It will carry that fear into similar work until mechanics retire it.
> `[ Run a similar task ]`

**Beat 3 — settlement (on approval).** The second task's plan visibly **heeds the caution**;
the inbox shows *settles if: a similar task is later approved without edits*. On approve,
settlement fires: the caution's falsifier was met **mechanically** — it falls into the
graves stamped `settlement.flow`, retired by evidence, not mood.

### CLOSE — *5:00*
> **Five minutes.** It refused you once. It believed you {N} times — each with evidence.
> It died once, in public{, and {M} things built on the dead belief were flagged}.
> It feared once — from your words — and retired that fear by mechanics.
> Everything it did is in the Record: signed, enumerable, yours.
> This is not a smarter chat. It is an accountable memory.
> `[ Open the record ]` `[ Finish ]`

Every number in the close is computed from the record at that moment — never hardcoded.

## 2. Direction rules

- **The user performs every beat.** The director speaks ≤ 40 words at a time and then waits.
  Total on-screen copy across the score: under 500 words.
- **Advance on evidence, not on clicks:** movements advance when the corresponding *record
  event* succeeds (bet placed, third note appended, terminal resolution, rejection,
  approval) — the choreography listens to the same API results the app renders.
- **Spotlight, don't dim:** the current target (witness input, Inbox rail item, the field)
  gets a calm accent ring. No scrims, no arrows, no confetti, no sounds.
- **Interruptible and replayable:** skip at any beat (Esc leaves the app fully usable);
  replay anytime from the ⌘K palette ("Replay the induction"). Induction artifacts are
  ordinary record entries — the user's first chapter, not tutorial debris.
- **Degrade honestly:** if consolidation extracts nothing, the director says so and asks
  for a sharper note; if the user approves instead of rejecting in IV-1, the director
  follows the actual path ("approved first try — the fear beat needs a no; reject the next
  one if it deserves it") rather than pretending.
- **Trigger:** offered automatically only when the record is empty (0 events — a genuinely
  first minute); never re-imposed after skip/finish (local preference), always available
  from the palette.

## 3. What each minute proves (the skeptic's ledger)

| Minute | The user saw | Backed by |
|---|---|---|
| 0–1 | it refused a belief that couldn't lose | admission rule at the only write path |
| 1–2 | it turned their words into cited positions, and refused its own bad candidate | consolidation + provenance validation + admission |
| 2–3 | evidence, verbatim, one click from any belief | refs → decrypt-on-view |
| 3–4 | it changed its mind in public, and consequences propagated | resolutions + RECONCILE |
| 4–5 | it feared from a rejection, heeded the fear, and retired it by mechanics | premortem minting → working-set cautions → automated settlement |

---

*Implemented in `crates/desktop` as `ui/induction.js` — a director engine over the live
application; see §2 rules for its behavioral contract.*
