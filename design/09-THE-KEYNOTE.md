# NEXUS Desktop — The Keynote

**Version 1.0 · July 2026 · A seven-minute score. Companion to 07 (the screen) and 08 (the
first five minutes); this document sequences the product for a stage, and specifies the
product mechanics built to make those sequences land in everyday use.**

---

## 0. The rule of the show

Nothing on stage is theater that isn't true off stage. Every audible moment below is an
unmodified backend behavior; the only things "keynote mode" adds are stage-scale rendering,
a presenter cue HUD, and a sanctioned attack tool (§4). If a beat can't survive a skeptic
walking on stage and pressing the keys themselves, it's cut.

The emotional architecture: **anticipation → refusal → initiative → death → the question →
fear → betrayal (attempted) → the reveal.** Each beat plants the next. The audience should
react not to visuals but to *conduct* — the system saying no, taking initiative, changing
its mind, surviving an attack, and finally being unmasked as having no model at all.

## 1. The score — 7:00

**Staging:** dark theme, stage scale on (F9), the Standing full-screen. Presenter speaks
little; the interface's own words carry (they were written for this). Camera holds on the
field; screen transitions are the choreography — no slide deck after 0:20.

### 0:00 — COLD OPEN · the empty field
One line from the presenter over a field holding nothing but the ground line:
> "This is NEXUS. It knows nothing about me — and, more importantly, it is not *allowed*
> to pretend it does."
Status bar visible: `● verified · 0 events`. *Anticipation: an AI demo that opens by
proving an absence.*

### 0:20 — BEAT 1 · THE REFUSAL — *first laugh*
> "Let's teach it something every AI would love to believe about me."
Types: **"my code never breaks production"** — submits. The backend refuses, verbatim,
huge on stage:
> `no falsifiers, no bet: a position that cannot say what would kill it may be an episode,
> never a reliance`
*(laugh, then applause — the machine said no.)* Presenter adds the falsifier — "I ship a
sev-1" — and the belief rises to the 0.50 prior. "It believes me now. Exactly this much."

### 1:10 — BEAT 2 · INITIATIVE — *the first "ooh"*
Three fast notes into the witness bar ("I prefer trunk-based development", "we require a
rollback plan for every deploy", one more). The instant the third lands — unprompted —
the presence line changes to *"appraising what you told it…"* and beliefs **rise into the
field citing his exact words**. And one candidate is struck through, self-refused, rule
quoted. *(ooh — it held itself to the law it applied to him.)*
> "I never asked it to do that. It also just refused **itself**."

### 2:10 — BEAT 3 · THE DEATH — *the gasp*
> "Twenty minutes before this keynote, I shipped a hotfix. You can guess what happened."
Clicks "my code never breaks production" → note: *"sev-1, eleven minutes into the demo
freeze"* → **✕ falsified**. The card falls through the ground; the belief standing on it
pulses violet and sinks, untouched. *(gasp — consequences propagated, visibly, without
being asked.)*
> "Nothing built on a dead premise gets to pretend otherwise."

### 2:50 — BEAT 4 · THE QUESTION — *applause line*
Presenter turns to the audience:
> "Tonight, ask your favorite assistant one question: **show me a belief you lost — and
> what killed it.**"
Types exactly that into ⌘K. The app answers by diving below the ground: the grave, struck
through, **KILLED BY —** his own words. *(applause — the litmus test, answered from a
ledger.)*

### 3:30 — BEAT 5 · FEAR & SETTLEMENT — *the quiet one*
Delegate "add a config file for logging" → Inbox → the effect list (hashes, not prose) →
**reject**: "config files need a schema comment." Cut to the field: a new card stands —
*a fear, minted from his sentence, with its own falsifier.* Run a similar task: the plan
heeds it. Approve → the caution's falsifier is met **mechanically**; it falls into the
graves stamped `settlement.flow`. *(low "ooh" — the machine got over its fear the only way
allowed: evidence.)*
> "I never manage its memory. My yes and my no *are* the memory."

### 4:50 — BEAT 6 · THE ATTACK — *the big gasp*
> "You think this is pretty state in a web app. Let me attack my own record."
Presenter flips bytes in the log — live (cue HUD button; a hex editor works identically).
The next heartbeat, the **entire application refuses to run**: status bar hard-red,
`record tampered or corrupt`, every screen declining to render rather than guess.
*(gasp)* Restore the bytes — green within a heartbeat.
> "It will not show you a memory it cannot prove. It would rather die than lie."

### 5:40 — BEAT 7 · THE REVEAL — *the one they'll quote*
> "One question I haven't answered: which frontier model has been doing all this?"
Opens Reasoners. The scorecard reads, in full view: **provider `mock` — a deterministic
offline stub.**
> "None. Everything you just saw — the refusal, the appraisal, the cascade, the fear, the
> settlement — ran with **no AI model at all**. The intelligence you felt is the
> *institution*: the rules of the record. The model is a peripheral."
Attaches a frontier model live (key sealed on stage), reruns consolidation on the same
notes — richer candidates rise, same admission rule filtering them. *(the reaction)*
> "Swap the brain anytime. The record — the beliefs, the graves, the scores — survives it."

### 6:30 — CLOSE · YOURS
Forget one note on stage — the ceremony discloses consequences, the body becomes
`— forgotten —`, the tombstone stays enumerable. Wide shot of the Standing: living
positions at earned altitudes over their graves. Final line:
> "Every AI you've seen today asks for your trust. **This one keeps receipts.**"

## 2. The gasp map (engineering targets)

| Time | Beat | Audible target | Mechanism (all real) |
|---|---|---|---|
| 0:35 | refusal | laugh → clap | admission rule at the only write path |
| 1:50 | self-refusal | "ooh" | consolidation guardrails + admission |
| 2:35 | cascade | gasp | RECONCILE over premises |
| 3:10 | litmus answer | applause | terminal resolutions + killing notes |
| 4:40 | settlement | low "ooh" | flow-scoped automated settlement |
| 5:10 | tamper refusal | gasp | per-frame BLAKE3, hard-fail semantics |
| 6:00 | "no model at all" | the quote | deterministic mock provider, scorecard as proof |
| 6:50 | receipts | applause | crypto-shred + tombstone enumeration |

## 3. Fallbacks (live-demo discipline)

- Every beat is idempotent on a rehearsal record; the full sequence is scripted against the
  API (rehearsed in CI like `prove-it.sh`).
- Beat 2 extracts nothing → presenter narrates the honest miss, sharpens one note ("the
  system holds a bar; watch me meet it") — the recovery *is* the message.
- Beat 6 restore fails → switch to backup record; the tamper tool refuses to run if the
  segment changed since backup (no accidental data loss even in rehearsal).
- Beat 7 network fails → the reveal already landed with the mock; attach-live becomes
  "and here's the config that swaps it" (10 s, no loss).
- Hard rule: presenter never types into a tampered record (appends during Beat 6 are
  blocked by the same refusal that makes the beat work).

## 4. What this turn adds to the product (not just the stage)

The keynote is a lens; these mechanics ship for every user:

1. **The presence line** — one quiet sentence at the top of the Standing, always computed
   from record state, never generated prose: *"2 positions near their horizon; one
   contradiction stands unresolved."* / *"3 effect lists await your verdict — nothing
   changes without it."* / *"The record is quiet: 12 positions standing, 47 survivals
   earned."* This is the collaboration feel in daily use: the system speaks first, briefly,
   and only with receipts.
2. **The litmus command** — ⌘K understands the product's own catechism: typing *"show me a
   belief you lost"* dives below the ground and spotlights the freshest grave.
3. **Keynote mode** (`--keynote`): stage scale (F9), presenter cue HUD with the eight beats
   (F8), and the sanctioned attack tool — `POST /api/dev/tamper` / `/api/dev/restore`,
   available *only* under the flag, with a size-guard so restore can never lose data.
   Tamper detection itself is the everyday product; the flag only adds the attacker.
4. **Integrity as conduct** — on any verify failure the whole surface visibly refuses
   (status bar hard-red, error verbatim) rather than rendering unverifiable state. Already
   the backend's semantics; now the UI's posture matches it.

## 5. Collaboration principles (the generalization)

What makes it feel like intelligence rather than software, extracted from the beats:

- **It refuses.** Judgment shown by saying no with a rule, to the user and to itself alike.
- **It moves first, small.** Initiative is one quiet sentence or one staged appraisal —
  never an unrequested action on the world (the kernel forbids that anyway).
- **It answers with receipts.** Any claim it makes about itself resolves to record events
  in one interaction.
- **It changes its mind in public,** and consequences travel without being asked.
- **It would rather die than lie.** Unverifiable state renders as refusal, not best-effort.
