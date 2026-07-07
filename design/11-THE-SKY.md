# NEXUS Desktop — The Sky

**Version 1.0 · July 2026 · The full rethink demanded by the "from zero" brief.** The Room
(design/10) invented the right *grammar* — one line, computed language, receipts, no
navigation — and then rendered it as a static column of text. Every "reorganization" was an
instant re-render: a jump-cut where the brief demanded choreography. A stranger watching
twenty seconds of The Room sees a well-written page. Nobody records a page.

This document performs the whole process again — three new interaction philosophies, the
keynote, the storyboards, the hostile review, the merge — and ends in the surface that
shipped as `crates/desktop/ui/sky.*`.

Standing law, inherited and not renegotiable:

1. **Nothing may be shown that the backend did not compute.** The invention is in the
   record; the surface may dramatize truth but never simulate it.
2. **The language layer of design/10 survives in full** (conviction / "I'd change my mind
   if —" / "turned out wrong" / "shaky" / "these can't both be true").
3. **Chat is not the front door.** One line. Everything enters as record events.
4. ARCHITECTURE.md wins all conflicts; the backend is frozen.

---

## Part 0 — Hostile review of the incumbent (why The Room failed the brief)

Filed first, because the failure teaches the requirement.

- **F1 — The Room has no space.** A single centered column. But the engine computes a
  *geometry*: every conviction has an earned fold, premises, dependents, tensions,
  relevance-to-a-question. The Room flattened a shape into a list. A list is what databases
  look like; a shape is what thinking looks like.
- **F2 — The Room has no time.** `prose.innerHTML = ""` and re-append. Ask a question and
  the answer *teleports into place*. The single most valuable asset the product owns — the
  moment of rearrangement, the moment a belief dies — was spent in zero frames. The brief's
  words: "the AI is reasoning visually." Zero-frame reasoning is invisible reasoning.
- **F3 — The Room wasted the backend's best sentence.** `resolve` *returns the cascade* —
  the exact list of convictions that became shaky when one died. The Room recomputed an
  approximation client-side and printed a count. The engine hands the UI its keynote moment
  in a JSON field, and the UI said "3 convictions are now shaky."
- **F4 — The Room opens like software.** Load → render → done, in one frame. The mind has
  been *alive between visits* — the record proves exactly what happened — and the surface
  spends that on a bulleted overlay.
- **What The Room got right** (kept, whole): the line and its "heard as —" echo; the
  admission ritual ("What would change my mind?"); the computed voice; receipts under every
  sentence; typographic weight as the earned fold; the refusal to decorate.

The requirement, extracted: **keep The Room's grammar, give it a body — space, time, and
consequence.**

---

## Part A — Three interaction philosophies (invented fresh)

Test applied to each: a stranger watches twenty seconds over your shoulder and stops
walking. Constraint applied to each: every visible behavior maps to an existing route
(`/api/standing · focus · record · bet · resolve · note · consolidate · do · inbox ·
approve · reject · status`).

### Vision 1 — THE SKY: knowledge as a place with physics

There is no page. There is a place: a dark expanse with one horizontal **ground line**.
Convictions live *above* the line as floating sentences; the dead lie *below* it, struck
through, in strata — the record's fossil column. One law generates everything:

> **Altitude is earned. Gravity is falsification. Nothing moves unless the record moved.**

- A conviction's height above the ground *is* its earned fold (07-THE-STANDING's altitude,
  finally made literal). Its typographic mass is the same number (10's weight law). An
  untested belief hovers barely off the ground. A conviction that survived ×6 rides high
  and heavy.
- Premises are **filaments** — hairline threads from a conviction down to what it rests on.
  Usually invisible; they light when they matter.
- A contradiction is an **amber thread** strung taut between two sentences that cannot both
  be true, labeled in plain words.
- When a conviction is falsified it does not disappear. It **falls**. Its filaments flash,
  its dependents shudder and thin, it loses weight mid-air, drops through the ground line,
  and lands in the sediment as a struck-through fossil with its "then: —" note. The ground
  ripples once. That five-second fall is the product's signature.
- A question doesn't navigate anywhere: the sky **rearranges**. Bearing convictions glide
  inward and sharpen into a column — the shape of an answer — everything else recedes into
  the dark. Quotes from your own past rise from the ground beneath them.
- Why it can't be faked: height, weight, the cascade list, the relevance ordering, the
  quotes — every coordinate is a number the backend already computes.
- Twenty-second moment: someone types "should I ship on Friday?" and the whole sky
  reorganizes into two heavy sentences, an amber thread between them, and a line of dated
  quotes rising from the ground.
- Weakness (held for Part D): free 2D scatter risks reading as decoration; collision-packed
  prose risks reading as a tag cloud. The layout must be *derived*, and must degrade
  honestly at 100+ convictions.

### Vision 2 — THE REEL: the mind as a film of itself

The primary axis is time. The surface is the record played as cinema: opening the app
*replays the mind* — observations flicker past, convictions are born, gain weight, die —
at 10,000× until it reaches *now*, where the film simply keeps running at 1×, mostly
still. A scrub bar (the only chrome) lets you drag backward: beliefs un-die, confidence
drains back out of survivors, tensions unstring. Asking a question assembles a *cut*: the
system pulls the relevant moments and plays them as a sequence with the convictions they
produced. "While you were away" is not a summary — it is literally the last three days
played in eight seconds.

- Strength: perfect honesty (the record *is* a timeline; replay is its native reading) and
  the strongest possible proof of "it has been alive."
- Twenty-second moment: watching someone drag time backward and a dead belief rise from
  the ground back into the sky.
- Weakness: a film is watched, not inhabited. The steady state — the 23 hours a day when
  the record barely moves — is dead air. A product cannot be a replay button.

### Vision 3 — THE COUNSEL: thinking as a formal, adversarial ceremony

The surface is a shared table between you and the institution. Nothing is ambient;
everything is a **move**, played in turns, with the gravity of a court. You state a belief
— the counsel does not file it; it *challenges* it ("What would change your mind?"), and
only a survivable answer earns it a place on the table. You ask for a judgment — the
counsel physically lays out its evidence piece by piece, slides contradictions between
your claims like exhibits, then states what it cannot know. Every artifact it places is a
citation you can flip over. When it wants to act, it stops everything and presents the
effects list as a formal motion requiring your signature.

- Strength: it makes the *standards* visceral — the machine with the spine to refuse. The
  proposal/refusal mechanics (already real: admission rule, effect lists, rejection-as-
  premortem) carry drama no dashboard has.
- Twenty-second moment: the machine refusing a belief, verbatim rule on screen.
- Weakness: ceremony fatigues. Forty moves a day of courtroom gravity is exhausting; it
  punishes exactly the casual, half-formed inputs a memory system must welcome.

### The choice

**The Sky is the body. The Reel is its memory of itself. The Counsel is its spine.**
Not a compromise — a composition with strict subordination:

- The Sky is the standing surface: the place you inhabit, the physics of weight and death.
- The Reel survives as exactly two moments: **the waking** (opening replay — the mind
  reconstructs itself from the record, births → weight → deaths → tensions, then hands you
  the present) and **the fall** (a death is always played, never stated).
- The Counsel survives as exactly three ceremonies, each already backend-enforced: the
  admission ritual, the effects-first proposal, and the tension — which is *strung*, never
  auto-resolved.
- The Room's line, voice, echo, and language layer govern all words on screen.

---

## Part B — The keynote (seven minutes, minute by minute)

Stage: dark. Presenter's record is real — three weeks of use, 28 convictions, 9 fossils.
No slides; the product is the deck.

**0:00 — Black.** One line types itself, character by character, centered:
*"I've been thinking."* Beat. The audience quiets on its own.

**0:20 — The waking.** The ground line draws itself across the dark. Then the record
replays: sentences materialize one by one in birth order, drift up as they gain weight,
filaments flick between them; two sentences fall, strike through, and settle below the
line as the audience watches three weeks compress into nine seconds. The sky comes to
rest. The voice says: *"I'm holding 28 convictions right now — 61 of them earned by
turning out right. Two things I believe can't both be true."* First phones come out.
**Why it lands:** nobody has watched software *reconstitute a mind* before showing a UI.

**1:30 — The question.** Presenter types: *"should I raise prices?"* — no menu, no send
button. Echo: *heard as a question — arranging what bears on it.* The sky exhales: twenty
sentences recede into the dark; five glide inward, sharpen, stack into a column ordered —
the presenter says this out loud — *by how often each has turned out right, not by how
confident anything sounds.* An amber thread slides between two of them. Beneath, three
monospace quotes rise from the ground, dated, from the presenter's own past mouth.
**Why it lands:** the machine answered a question by *arranging your own audited beliefs*
— and visibly declined to pretend it knows the answer.

**2:45 — The machine has standards.** Presenter types: *"customers love the new
onboarding."* The sky dims; the sentence hangs mid-air as a ghost — italic, weightless,
*not yet held*. The counter-question appears beneath it: *"Before I hold that — what would
change my mind?"* Presenter smirks, types *"nothing, they just do."* The refusal comes
back verbatim from the engine — a conviction that cannot lose may not be held — and the
ghost disintegrates. Presenter answers properly ("churn above 4% next month"); the ghost
solidifies, takes on weight, and rises — barely — to hover just off the ground: held, but
lightly, because it has never been tested. **Why it lands:** every AI on stage this year
agreed with its presenter. This one refused.

**4:00 — The fall.** Presenter opens the receipts of a high, heavy conviction — *"the
enterprise tier is our growth engine"* — shows what rests on it (three filaments light
downward), and marks it: **it turned out wrong**, typing the reason. Filaments flash. The
two dependent convictions shudder and visibly thin. The sentence loses its weight mid-air
— you watch the font itself go frail — tips, falls through the ground line, and lands
struck-through in the sediment with its epitaph: *then: Q2 numbers came in.* The ground
ripples. The voice: *"I changed my mind. Two convictions that rested on it are now shaky —
I've marked them."* Silence, then the loudest applause of the demo.
**Why it lands:** this is the brief's Moment 4 — a mind changing, played as physics, every
frame of it a fact from the ledger (`resolve` → `cascaded`).

**5:15 — The proof.** Presenter: "Everything you just watched is an append-only,
content-addressed record. Watch what happens if anyone — including me — edits one byte of
its past." A confederate runs the tamper. The sky goes dark mid-sentence; the voice:
*"I can't prove my own memory right now — so I won't show it to you. I would rather go
dark than lie."* Restore; the waking replays in two seconds; the sky returns intact.
**Why it lands:** the emotional register flips from delight to trust.

**6:00 — The descent.** Presenter clicks the quiet line near the ground: *"9 things I was
wrong about —"* and the camera descends below the ground line into the strata: every dead
conviction, struck through, each with its cause of death, oldest deepest. Presenter, over
the fossils: "It keeps score on itself. This is the part no one else will ship."

**6:45 — Close.** Camera rises to the sky at rest. One sentence from the presenter:
*"It remembers me better than I remember myself — and it can prove it."* Product name.
Black.

---

## Part C — Storyboards (every state, every transition)

Timing constants: `--beat: 700ms`, ease `cubic-bezier(.22,0,.06,1)` for glides,
`cubic-bezier(.5,0,.9,.4)` for falls. All choreography respects `prefers-reduced-motion`
(cuts to ≤10ms). Any key or click skips the waking.

**S1 · Cold open (empty record).** Black stage → ground line draws (900ms) → voice types:
*"I don't believe anything yet. Tell me something — but I'll only keep what could turn out
wrong."* The line fades in, focused. No other elements exist.

**S2 · The waking (returning user, changes since last seen).** Black → voice types "I've
been thinking." (interruptible) → ground draws → birth replay: convictions materialize in
`created` order, 90ms stagger, each drifting up 24px into place; filaments draw to
premises (280ms stroke-dash) and fade to hairline → deaths: each dead belief appears then
immediately falls below the line (compressed fall, 450ms) → tensions string last (amber
thread draws, both endpoints shiver once, 200ms) → "while you were away" lines fade in
over the settled sky, each opening into *why* → voice speaks the computed presence line.
If nothing changed and the gap is short: settle-in only (staggered materialize, 1.2s, no
typing, no overlay).

**S3 · Sky at rest.** Sentences hang at altitude = effective fold; x-positions are
deterministic (stable per-id jitter around a center-weighted spread, collision-relaxed).
Idle motion is nearly nothing and strictly non-semantic: a sub-pixel breath (±1px, ~9s,
per-sentence phase) — presence, not information; disabled under reduced-motion. The voice
rotates (25s crossfade) through *computed* facts only. Nothing else moves without a record
event. Stillness remains the proof that motion means something.

**S4 · Question.** Enter ⌁ echo "heard as a question —" → `/api/focus` → one beat where
nothing moves (the machine visibly *takes* the question) → non-bearing sentences recede
(600ms: drift 10% outward, blur 1.4px, opacity .16) → bearing sentences glide to the
center column, top-down in relevance order, 80ms stagger, sharpening as they arrive →
filaments *within* the bearing set light; an amber thread interposes if a tension is
internal to the answer → moments rise from the ground under the column (dated monospace,
120ms stagger). Esc or a new input exhales the sky back (everything returns, 700ms).
Empty case: no motion at all; voice: *"nothing I currently believe bears on that — but
here is everything I'm holding."*

**S5 · Receipts.** Click a sentence → it steps forward (scale 1.02, neighbors dim to .3)
→ an underpanel unfolds beneath it in place (240ms): *I'd change my mind if — · how sure
(survived ×n / never wrong, and the number) · because — you told me (decrypt-on-view
quotes) · this rests on (click = glide focus to premise) · if this turns out wrong, N
others become shaky · [it held up] [it turned out wrong] + "what happened?" note.* Esc
closes; the sky rebrightens.

**S6 · The fall (identity animation, driven by `resolve.cascaded`).**
t+0 filaments from the dying sentence flash and pulse outward → t+300ms each cascaded
dependent shudders (3px, 260ms) and thins to its discounted weight, gaining the *shaky*
mark → t+500ms the sentence's own weight drains (font-weight 700→300, size shrinks,
50ms/step — visible frailty) → t+900ms it tips (−2.5°) and falls (gravity ease, 850ms)
through the ground line → landing: strike-through draws left-to-right, epitaph fades in,
ground ripples once from the impact x → voice states the change and the count, verbatim
truth. Total ≈ 2.4s. "It held up": the sentence pulses once, gains weight (size/mass
animate up), and *rises* to its new altitude — the anti-fall, 900ms.

**S7 · Admission ritual.** Belief detected → sky dims to .4 → ghost sentence center
stage (italic, weight 300, 60% opacity) → counter-question beneath → falsifier entered →
ghost solidifies (300ms: italic→roman, opacity→1), glides to its low altitude near the
ground → voice: *"I'll hold that now — lightly, until it earns its weight."* Refusal →
the engine's rule verbatim beneath the ghost; ghost disintegrates (fade + 12px sink,
500ms). Contradiction on admission → amber thread strings *immediately* from the newborn
to its opponent, both shiver once.

**S8 · Observation & reflection.** "Noted." Nothing else moves (honesty: an observation
changes no belief). After the third: voice: *"I've been thinking about what you told me —"*
→ beat → new convictions rise *from the ground line* (they come from the record), 140ms
stagger, provenance filament flashing down to nothing as each settles → rejected
candidates appear mid-air already struck through and fall immediately; voice: *"one thing
I refused to believe — even from myself: it couldn't say what would make it wrong."*

**S9 · Proposal (instruction).** Echo: "working —" → the sky *holds its breath* (all
motion suspends, 200ms dim) → the proposal descends center-stage: intent, exact effects
(＋/±/−, monospace paths), any carried worry ("I'm wary here — last time: …"), the note
that nothing changes without a yes → **no** requires a reason, which visibly becomes a new
worry near the ground; **yes** applies, and any settled worry *retires*: it doesn't fall —
it dissolves upward, softly (worries that die honorably leave the sky the opposite way).

**S10 · The descent.** Click *"N things I was wrong about —"* → the camera (whole world
container) glides down 60% viewport (900ms): the sky compresses above, the strata fill
the frame — every fossil struck through with its "then:" epitaph, newest shallowest. Esc
rises back. No panel, no page: the graveyard is a *place* under the place.

**S11 · Dark integrity.** Any `/api/status` failure or torn>0 → the sky snaps to 8%
opacity, inert; voice, in the wrong-red: *"I can't prove my own memory right now — so I
won't show it to you. I would rather go dark than lie."* Ground text carries the raw
error in monospace. Recovery re-runs the settle-in.

---

## Part D — The hostile review of my own work

- **"The Sky is a screensaver."** The lethal one. Defense is discipline: every pixel of
  motion is caused by a record event and every coordinate is a readable number (altitude
  *is* the fold; open any receipt and see it). The one exception — the 1px idle breath —
  is deliberately sub-semantic and dies under reduced-motion. If a tester ever asks "what
  does that motion *mean*?" and the answer is "nothing," the motion ships removed. The
  breath is on probation, explicitly.
- **"Free 2D placement is false information."** x-position genuinely encodes nothing.
  Mitigation: x is *stable* (seeded per-id, collision-relaxed), so position reads as
  identity ("my pricing belief lives up-left"), not as data. Altitude, size, and mass are
  the only spatial encodings, and all three encode the same audited number — redundancy,
  not decoration. This is the same license typography took in The Room.
- **"It will not scale past ~40 convictions."** Correct, and accepted *as product truth*:
  a sky of 200 sentences is unreadable in exactly the way a mind of 200 simultaneous
  convictions is unmanageable. At >36 live convictions the sky renders the heaviest 36 and
  the ground line says *"and N more, held lightly —"* (click: they surface in focus mode).
  Honest triage, stated on-surface, never silent.
- **"The waking will get old."** It plays fully only when the record changed since last
  seen; otherwise a 1.2s settle-in. Any key skips. The keynote version is the *long* cut;
  daily use gets the short one. If telemetry-of-one shows skipping every time, the waking
  demotes to first-run + returns-after-absence only.
- **"The fall dramatizes what a click already said."** No — the click said "this one is
  wrong." The *cascade* (which dependents thinned, by how much) is information most users
  would never have queried, delivered at the exact moment it matters, from `cascaded`,
  not simulated. The animation is the difference between being told consequences exist
  and watching them arrive.
- **"Two surfaces died; this is the third. Why believe it?"** The Instrument failed
  because it was a dashboard. The Room failed because it was a page. Both failures share
  one cause: they rendered *state* and discarded *transitions*. The Sky is the first
  surface whose primary artifact is the transition. That is not a coat of paint on The
  Room; it is the missing half of it.
- **Removed in this review:** the Reel's scrub bar (chrome; replay covers the honest
  need), the Counsel's turn-taking everywhere (kept only where the backend already
  enforces ceremony), free-floating physics simulation (jitter without meaning), and any
  particle effects on death (the fall is a fact; sparks would be a lie).

---

## Part E — The merge, as shipped

`crates/desktop/ui/sky.html · sky.css · sky.js`, served at `/`. The Room remains at
`/room` (the grammar reference), the Instrument at `/classic` (operator tooling:
providers, ledger, keynote HUD). No new backend semantics; one route table edit only.

| Element | Source of truth | Choreography |
|---|---|---|
| Altitude, mass, size | `fold` (× unjustified discount) from `/api/standing` | glide on change |
| The waking | `created` order + statuses from `/api/standing` | S2 |
| Question rearrangement | `/api/focus` relevance + moments | S4 |
| The fall + cascade | `/api/resolve` → `cascaded` | S6 |
| Admission / refusal | `/api/bet` (+ verbatim engine error) | S7 |
| Reflection | `/api/consolidate` placed/rejected | S8 |
| Proposal / worry / settlement | `/api/do` + `/api/inbox` + approve/reject settled list | S9 |
| Fossils & epitaphs | terminal statuses + `terminal_note` | S10 |
| Dark integrity | `/api/status` verify | S11 |
| All words on screen | design/10 language layer | — |

The twenty-second test, restated one last time: a stranger watches a sentence lose its
weight mid-air, fall through a line, and land struck-through among the other things this
machine has admitted it was wrong about — while two surviving sentences visibly grow
frail because they rested on it. There is no software that looks like that.

---

## Part F — The attention law (v1.1: the film cut)

v1.0 failed its own test in one specific way: it rendered *every* conviction as a full
sentence, simultaneously. Ten overlapping paragraphs is a wall of text, not a mind — the
viewer had to read the thinking instead of watching it. The correction is a single law,
borrowed from cinematography and enforced at every moment, including rest:

> **One primary. Two supports. Everything else is a mark.**

- A conviction's standing form is a **mark**: a small point of light at its earned
  altitude, whose size is the same fold number (belief = filled point, prediction =
  ring, worry = diamond, shaky = hollow dashed). Marks are the ensemble; they are never
  read, only *seen* — position and size carry the whole truth, and any mark resolves
  into its sentence the instant attention lands on it (hover = a one-line peek; click =
  promotion to primary). Nothing is hidden; everything is *deferred*.
- **Attention is the camera.** At rest, the mind's heaviest conviction is the headline —
  it alone is a full sentence, its two runners-up as smaller supporting lines. A
  question re-aims the camera: bearing marks converge, then resolve *sequentially* —
  first, second, third, in calibration order, ~300ms apart — so rank is experienced as
  *the order thoughts arrive*, not read off a list. The fall re-aims it again: the dying
  conviction is primary, its two most important dependents are the supports, and the
  rest of the sky recedes to dim points. The ritual and the proposal were already
  single-focus and are unchanged.
- Fossils obey the same law: at rest the sediment is a column of struck dashes (you see
  *that* you were wrong, and how often); the sentences resolve only during the descent —
  or for the 2.2 seconds after a fresh landing, long enough to read the epitaph once.
- Honesty is preserved, not traded: glow was rejected in design/10 because it *replaced*
  the number; the mark *defers* to it — same altitude law, same size law, and one
  gesture from the verbatim sentence and its receipts. Elision is not alteration.

Why this is "watching the AI think": with at most three legible sentences on stage, any
change in what is legible IS the reasoning. The camera movement — what resolves, in what
order, what recedes — is the visible train of thought.

---

## Part G — The daily open (v1.2: the demo becomes a moment) · *superseded by Part H: the home never plays the film; the Sky keeps this behavior at `/sky`*

The keynote (design/12) is frozen and demoted to `/?demo`. The governing split:
**the product feels magical exactly once; after that it feels trustworthy.**

- **First open of a record** (no last-seen timestamp, record non-empty): the full
  waking film plays — typing, ignition, the headline. Once. This is the product's
  one theatrical moment in daily life.
- **Every other open**: no overlay, no typing — the sky settles in about a second
  (wave ignition, ≤16 waves at any record size) and the line is focused. Fast,
  quiet, trustworthy.
- **The contextual return**: if reasoning changed while away, a short note names
  exactly what — computed from the record's own `memory.resolution/1` events since
  last seen, never inferred from all-time state (the old note re-reported every
  fossil forever; that is now impossible). Each line carries its own doorway:
  *turned out wrong* → the descent; *now shaky* → attend the shaken conviction;
  *came to believe* → attend the newest. If nothing changed, nothing plays.
- **Live choreography is already contextual** and unchanged: the fall plays when
  the user falsifies, reflection births play when consolidation places, the
  admission ritual plays when a belief is offered. Reasoning changes are the only
  license for motion — the boot is no longer an exception to that law.

---

## Part H — The Home (v1.3: the Room, developed into the product)

The keynote succeeded and is done teaching. The product is the Room's readable
prose, rebuilt around one law: **the first screen answers exactly one question —
"what changed while I was away?"** Ships as `home.*` at `/`; the Sky moves to
`/sky`, keeping the frozen Minute (`/?demo` redirects there). Nothing is hidden
behind animation; nothing waits.

1. **The changes lead.** The top of the page is *"since yesterday —"* followed by
   the actual changed sentences (not counts, not an overlay): what turned out
   wrong and what it took down with it, what is now shaky, what was newly
   believed, what survived testing. Each line is a doorway — the fallen open the
   sediment, the shaky and the newborn scroll to themselves. If nothing changed:
   *"Nothing changed. Everything I hold stands as you left it."* Answering with
   nothing is also answering.
2. **The standing beneath.** Convictions as prose, ordered by earned weight,
   typographic mass = the fold, every sentence carrying its scoreboard chip.
   Receipts unfold in place. Tensions sit above the prose as paired sentences.
   The sediment folds at the bottom. It scrolls; it is a record, and records
   scroll.
3. **Reasoning animates only on reasoning:** a **question** reorders the prose in
   one continuous FLIP glide — bearing convictions rise to the top in calibration
   order, the rest compress and dim, the user's own past quotes slide in beneath
   (Esc exhales). A **falsification** plays the column-native fall: dependents
   shake and thin from `cascaded_ids`, the sentence strikes through, collapses,
   and leaves the prose for the sediment. A **held-up** climbs the prose as its
   weight rises. A **birth** materializes in place. Everything else is still.
4. **The line, the voice, the ritual, the proposal** carry over unchanged in
   grammar; the ritual is inline (no curtain — daily mode has no theater), the
   refusal is human-first with the statute beneath.
5. The Sky remains the keynote and explanation surface (`/sky`, `/sky?demo`); the
   original Room stays at `/room` as the grammar reference; `/classic` serves
   operators.

---

## Part I — The first thirty seconds (v1.4: the interview)

A stranger with a real decision, no philosophy, thirty seconds. The interaction
must teach memory, evidence, changing-your-mind, and confidence — without ever
using the internal vocabulary (belief, fold, epistemic, settlement, premortem,
ledger, provenance) until each has already been *felt*. Nothing explains; the
flow is the explanation.

**The flow.** They type the decision — *"should I take the Berlin offer?"* — and
the empty mind does the one honest thing available:

1. *"I don't know you yet — so I won't guess."* → **memory, discovered by its
   absence.** The ask: *"tell me three things you've actually seen about this —
   not guesses, one at a time."*
2. Each thing they type is kept, visibly — quoted verbatim under *"what you've
   seen —"*, stamped *just now* → **evidence, watched accumulating.**
3. After three: *"give me a moment —"* and their own sentences return as held
   positions, each chip reading *not yet tested — I hold this only lightly* →
   **confidence experienced as something earned, currently zero** — and each one
   opens into *"because — you told me"* with their words quoted back.
4. The original question re-asks itself: the prose arranges around it, their own
   quotes beneath — and the closing line plants the last concept without naming
   it: *"when one turns out right or wrong, tell me: that's how I learn which of
   these to trust."* → **changing-your-mind, promised;** delivered the first time
   they press *it turned out wrong* and watch the strike-through and the shaking
   dependents.
5. If something they said couldn't be kept: *"One thing you said I couldn't
   keep — nothing could prove it wrong."* → the standards, tasted early.

**Mechanics.** The interview triggers when a question finds nothing bearing and
the record holds fewer than three convictions; Esc cancels; a mid-interview
question re-targets it. With no model attached, the mock appraiser now keeps
recent observations *verbatim* as candidates (previously it pattern-matched
"prefer/require" — natural speech produced nothing and the happy path
dead-ended); the admission rule and dedupe still filter everything downstream,
and the runtime tests encode the new contract. With a real model attached, the
same flow runs through real appraisal.

**The trigger word audit** stands as a test on the surface: no user-visible
string contains the internal nouns; the verbs of ordinary speech ("I believe",
"you said") are not vocabulary, they are English.

**The counsel (v1.5: ANSWER is reasoning, not a restatement).** The focused
convictions are inputs, not the answer. After FOCUS, a counsel block renders
inside the answer bar, computed entirely from the record's numbers: *the
evidence* (how many bear; tested / untested / shaky; the strongest and its
record), *what's missing* (untested testimony named as such, the falsifier that
would settle the strongest, the absence of past record, the user's undeclared
deciding condition), *my footing* (mean earned fold as a percentage with a
plain-language grade), and *where I stand* — a recommendation only when
evidence has earned it (stand on what has survived testing; discount shaky;
refuse both sides of a live tension), and an explicit, reasoned refusal when it
hasn't ("everything here is untested — test the strongest, or give me the one
condition that would decide it"). No fabricated pro/con valence: direction of
evidence requires interpretation the record alone cannot honestly supply, so
the counsel never pretends to it. Gated by seven checks in `home.e2e.js`.
