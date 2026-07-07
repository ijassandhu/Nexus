# NEXUS — The Minute

**FROZEN · July 2026.** The Minute is a finished artifact: it plays only at `/?demo`,
never as anyone's first screen. The full waking film plays exactly once in daily
life — the first open of a record — and after that the product is quiet; changes in
reasoning arrive as contextual moments (11 Part G). Changes here require a design
review; polish belongs in the surface, not the script.

**Version 1.0 · The sixty-second keynote demo, built before anything else.**
If a stranger watches only these sixty seconds, they understand why NEXUS exists:
*a machine that earns its confidence, refuses what can't lose, changes its mind in
public, and can prove every memory.*

Law of the Minute, inherited from design/11 and not negotiable:

1. **Nothing on screen is scripted prose.** The demo runs on the real backend against a
   real record. The director only *types* and *clicks* — every sentence, weight, quote,
   refusal, and cascade is computed. The record it plays on is seeded through the same
   public API a user would hit, and survives inspection afterward (`/room`, `/classic`,
   `nx` all read the same record).
2. Captions are the one presentational element — five of them, small, and they only
   *frame* what the record is doing. They never claim anything the surface isn't showing.
3. Any keypress or click aborts the director and hands over the live product. The demo
   is not a video; it is the product driving itself.

**Run:** `nexus-desktop --data .nexus-minute --passphrase … ` then open `http://127.0.0.1:4816/?demo`.
First run seeds the record (~2s, behind the black); re-runs replay against the grown
record — each run adds one more real fossil, which is itself the point.

---

## The script — second by second

**0:00 – 0:03 · Black.**
Nothing. A dark screen and a blinking caret. (Behind the black, the record is being
seeded through `/api/bet`, `/api/resolve`, `/api/note` — twelve convictions, earned
folds from repeated `held` resolutions, two falsified, three observations. No
contradiction pair: a tension shown but never fired is a Chekhov violation, 13 §2.
Real writes, real crypto, real ledger. A record carrying demo scar tissue — duplicate
live statements — is refused with instructions to start fresh, never cleaned on
stage.)
*Why it works: three seconds of silence is the most confident thing software can do.*

Throughout: **captions are the only narrator** (`body.keynote-demo` hides voice and
echo; receipts render at stage-legible sizes). The keynote is allowed to be
theatrical because it teaches; daily mode keeps the quiet voice, no captions, no
rotation, no idle motion.

**0:03 – 0:05 · The first line.**
Types itself, character by character: **"I've been thinking."**
*The hook. Not a login, not a dashboard, not a prompt. A claim — which the next seven
seconds must prove.*

**0:05 – 0:12 · The waking.**
The ground line draws itself across the dark. Marks ignite in birth order at their
earned altitude while the caption teaches the one law the surface runs on: *"Everything
it's sure of floats higher. It had to earn that."* Two dashes drop through the line
into the sediment. Then the headline resolves — the heaviest conviction as a sentence,
its two runners-up beneath, each carrying its **scoreboard chip** (*survived ×5 ·
never wrong*). The caption speaks the computed proof: *"I'm holding 12 convictions
right now — 18 times they've turned out right."*
*Proof of the claim: it HAS been thinking, and the evidence arrives with the sentences.*

**0:12 – 0:23 · The question.**
The line types: **"should I ship on friday?"** Echo: *heard as a question — arranging
what bears on it.* One beat of stillness — the machine takes the question. Then the sky
exhales: unrelated convictions recede blurred; the bearing ones glide into a column,
ordered by track record; the amber tension slides with them if it bears. Beneath, dated
monospace quotes rise from the ground — the user's own past words about Friday releases.

> Caption 1: *"It didn't answer. It arranged what you already believe — ranked by how
> often you've been right."*

*The why-NEXUS-exists sentence, shown not said: your judgment, audited, is the answer.*
(Esc — the sky exhales back.)

**0:23 – 0:36 · The standards.**
The line types a belief the record doesn't already hold (per-run candidates, so
re-runs never collide). The sky dims behind the curtain. The sentence hangs mid-air
as a ghost — italic, weightless, *not yet believed*. Beneath it: *"Before I hold
that — what would change my mind?"* The caret waits… and presses Enter on nothing.
The answer comes human-first — **"That can't lose. I won't hold it."** — with the
engine's rule verbatim in small print beneath.

> Caption 2: *"It refuses beliefs that can't lose."*

Then the honest answer is typed — **"churn above 4% within a month"** — and the ghost
solidifies, takes weight, and rises barely off the ground: held, lightly, because it has
never been tested.
*Every AI demo this year agreed with its presenter. This one refused.*

**0:36 – 0:48 · The fall.**
Click the heavy conviction *"The enterprise tier is our growth engine."* Caption,
before the event — attention is aimed in advance: *"Two beliefs rest on this one.
Watch them."* Its receipts unfold at stage size. Then **the cause is visible**: the
reason is typed into the note field on stage — *"Q2 numbers came in — flat two
quarters"* — and the **it turned out wrong** button is visibly pressed. The audience
sees the user report that the world changed; the machine's contribution is what
happens next: filaments flash, both dependents shudder and thin (chips turn shaky),
the sentence goes frail mid-air, tips, falls through the ground line, lands
struck-through with its epitaph. The ground ripples.

> Caption 3: *"Confidence here is never asserted. It is earned by scoring — and lost
> in public."*

*The identity animation. Every frame of it is a fact: the cascade list comes from the
ledger (`resolve → cascaded_ids`), not from the animator.*

**0:46 – 0:52 · The descent.**
The camera sinks below the ground line. The strata: every dead conviction, struck
through, each with its cause of death. Three fossils now — one of them thirty seconds
old.

> Caption 4: *"It keeps score on itself."*

The camera rises back to the living sky.

**0:52 – 0:60 · The close.**
The sky at rest — twelve convictions holding their earned altitude (one of them born
sixty seconds ago, one fallen), one tension
strung, the ground text reading *every memory provable · N kept*. The final caption:

> Caption 5: *"NEXUS — an institution of one."*

The caption fades. The line is focused. The demo is over and the product is simply…
running. Whatever the viewer types next is real.

---

## What the minute proves, beat by beat

| Beat | The claim a viewer walks away with | The mechanism that makes it true |
|---|---|---|
| The waking | "It has been thinking between visits" | append-only record replayed in birth order |
| The question | "It answers with my own audited judgment" | `rank_bets`: similarity proposes, calibration decides |
| The standards | "It refuses what can't lose" | admission rule at the only write path |
| The fall | "It changes its mind, and shows the cost" | falsification cascade over premise links |
| The descent | "It keeps score on itself" | terminal statuses are kept, never deleted |
| The close | "And it can prove all of it" | content-addressed, verifiable ledger |

## Director implementation notes

- Activated by `?demo`; lives in `sky.js` (`runDemo`). The director *types into the real
  line* and calls the same handlers a human would; there is no demo-only render path.
- Seeding is idempotent by statement: live convictions are reused, fossils are never
  re-created, and the fall target + its two dependents are re-created fresh only after a
  previous run has felled them (so every run has a fall, and the sediment accumulates
  honestly).
- One product change shipped with the Minute (not demo-only): pressing Enter on an empty
  falsifier now *submits* to the engine and shows the admission rule verbatim, instead
  of silently doing nothing. The machine states its standards; the UI was hiding them.
- Captions are `#caption`, presentational by declared exception (Law 2). Abort: any user
  keydown/pointerdown cancels all cues, clears the caption, and returns control.
