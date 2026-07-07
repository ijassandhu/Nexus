# NEXUS — The Teardown

**July 2026 · An adversarial product review of the Minute and the Sky, written as if
I did not build it and my job is to kill it.** Verdict up front: **the concept
survives; the staging does not.** The fall is a genuinely new thing in software. But
the demo currently teaches six visual vocabularies in twelve seconds, narrates
through three competing text channels set in the smallest type on screen, and spends
its most important beats explaining *after* the visual instead of aiming the eye
*before* it. A stranger watching today would be impressed and confused in equal
measure — and confusion wins after the novelty fades.

Nothing here proposes new concepts. Every fix is staging, hierarchy, curation, or cut.

---

## 1 · Where attention drops (timestamped against the Minute as built)

| When | What happens | Why attention drops | Fix |
|---|---|---|---|
| 0:05–0:12 | The waking: dots ignite in waves | **The proof is unreadable.** "I've been thinking" is proven by… anonymous confetti. Nothing to read or grab for ~5s until the headline resolves. The claim's verification (the voice: "12 convictions — 18 times right") is 13px gray at the top of the screen, the least-watched position | Caption the waking (it is the only uncaptioned beat): *"Everything it's sure of floats higher. It had to earn that."* And when the headline resolves, resolve its track-record chip with it (see §5) |
| 0:14–0:17 | Question → 350ms stillness → 1s converge → first sentence at ~3s | Three seconds of moving dots between Enter and the first readable word. The "beat of thought" reads as lag past ~1.5s | Cut the pre-beat to 200ms; start the primary resolve at 60% of the converge glide (motion overlap is fine when it's one continuous camera move toward the same target) |
| 0:23 (between scenes) | After the question exhales, **rest attention re-resolves the old headline** for ~1.2s before the belief beat begins | The eye is yanked to a sentence that has nothing to do with the next scene. Scene transitions must go to neutral, not to a new headline | During the demo, exhale to marks-only between beats; rest attention returns only at the close |
| 0:36–0:38 | Receipts panel opens for 1.9s before the fall | A wall of 12px text — falsifiers, percentages, buttons. Unreadable at distance; worse, it *looks like a database record*, the exact SaaS smell this product exists to kill | In demo staging, the receipts moment shows exactly two lines large: the falsifier and "survived ×2 · never wrong". Full receipts are a product feature, not a stage prop |
| 0:38 | The falsification itself is invisible — the director calls the resolve directly; no one sees the button pressed | **Cause is missing from the causal beat.** The machine appears to change its mind spontaneously — which both oversells (it can't do that) and confuses (why now?) | Visibly press "it turned out wrong": cursor moves, button highlights, 400ms hold, then the cascade. The audience must see that *the user told it the world changed* — the machine's contribution is what happens next |
| 0:46 | Descent to fossils | The fresh fossil landed ~7s ago; its highlight has expired. Down in the strata, nothing marks which death the audience just watched | Keep the newest fossil's text resolved (not dashed) for the duration of the descent |

## 2 · Where users get confused

1. **Six vocabularies, zero teaching.** In the first twelve seconds: dot size, four
   dot shapes (filled / ring / diamond / dashed), altitude, the ground line, dashes
   below it, an amber thread with a label. Films introduce one symbol at a time.
   **Fix:** in the Minute, all dots are filled circles — shape vocabulary (prediction
   rings, worry diamonds, shaky dashes) is a daily-use detail taught by hover, not
   keynote material. The demo needs exactly three symbols: dot, sentence, ground line.
2. **The refusal reads as an error.** Enter is pressed on an *empty* input — nobody
   notices an absence — then red monospace appears citing `specs/record.md §11.1`.
   Spec citations are developer talk; on stage this reads as a crash. **Fix:** the
   surface leads with the human sentence — **"That can't lose. I won't hold it."** —
   with the engine's verbatim rule in small print beneath (honesty intact, order
   reversed). And the demo should stage the *lazy answer being visibly typed and
   visibly rejected as too vague is not possible with the current engine* — so stage
   the emptiness: the caret sits in the falsifier field, a beat passes, the caption
   says *"It's waiting. It won't proceed without a way to be wrong."* — then Enter.
3. **Three narrators.** Voice (top), echo (under it), captions (bottom) all speak
   during the demo — plus the ghost, the sentences, and the ground status line. Six
   text channels; a film has one subtitle track. **Fix:** during the Minute, captions
   are the only narrator: suppress voice rotation and echo entirely; fold their
   content into the caption track. In the product at rest, keep voice + echo, kill
   the 25-second rotation (see §3).
4. **The tension thread is a Chekhov violation.** An amber "can't both be true"
   thread is strung during the waking, never mentioned, never resolved. Unexplained
   props make audiences anxious. **Fix:** cut the contradiction pair from the
   Minute's seed. Tensions earn their own beat in the five-minute version or not at
   all.
5. **The question is never answered — and only the caption says why.** The concept
   (it arranges your own audited judgment instead of generating an answer) is the
   product's soul, but on stage it hinges entirely on caption 1 landing at the right
   second. If a viewer misses it, the product looks broken. **Fix:** the arrangement
   must carry its own evidence — see §5 (track-record chips). When the sentences
   arrive stamped "survived ×3 · never wrong", ranked order becomes self-explanatory.
6. **The record is inside baseball.** "SSO deserves the top of the Q3 roadmap" plays
   to founders only. A Jobs demo would use a *personal* record — sleep, running,
   family, money — because "it remembers me better than I remember myself" is the
   emotional climax and roadmaps have no emotions. **Fix:** ship two seed profiles
   (`founder`, `personal`); default the keynote to personal.

## 3 · Unnecessary animation (cut list)

- **The 1px idle breath.** Was on probation (11 §F); verdict: **cut.** At 1px it is
  subliminal jitter — imperceptible as life, perceptible as blur on low-DPI. The
  product's stillness discipline ("stillness is the proof") is stronger without it.
- **Voice rotation every 25s.** Text changing with no user cause violates the
  product's own law (nothing moves unless the record moved). It also competes with
  whatever the user is reading. Cut; the voice updates on record events only.
- **Ignite pulse on every reload-born node.** Correct for reflection births; noise
  when it fires for routine reloads. Restrict to genuinely new convictions.
- **Tension labels always visible at rest.** A permanent amber badge is a standing
  alarm that dulls. Show the thread at rest; show the label on hover or when either
  endpoint is attended.
- **Keep:** the fall (the product), the ground ripple (its punctuation), the ignite
  waves in the waking (compressed honesty), the sequential arrival (rank as time).

## 4 · Where it's clever instead of understandable

- **Fossil dashes.** Minimal to the point of mute — a dash teaches nothing. The
  *count label* ("7 things I was wrong about —") is doing all the work and that's
  fine; but then the dashes are texture, and texture shouldn't need decoding. Keep
  dashes as texture, make the label the explicit doorway, and let the newest fossil
  stay legible (see §1).
- **Altitude is never taught.** Height = earned confidence is the load-bearing
  metaphor and the interface never says it once. One caption in the waking fixes
  this forever (§1). Additionally, the *anti-fall* — "it held up" → the sentence
  visibly climbs — is the cheapest possible teacher of altitude and appears nowhere
  in any demo. The five-minute version must include one.
- **The refusal's spec citation** (§2.2). Verbatim honesty belongs *on* the surface,
  but hierarchy is a product decision: human sentence first, statute beneath.
- **Demo-record hygiene as theater.** The duplicate-cleanup pass retracts junk — and
  every retraction becomes a *fossil*, so repeated dirty runs fill the sediment with
  "duplicate of an existing conviction" gravestones. The graveyard of honest
  mistakes must not be full of janitorial notes. **Fix:** the Minute refuses to run
  on a polluted record and says so plainly ("this record has demo scar tissue — run
  me on a fresh one: `--data .nexus-minute`"). Curation over cleanup.
- **Hover peeks during a live demo.** A presenter's stray mouse pops random one-line
  texts mid-beat. Suppress pointer reveals while the director is running.

## 5 · The one structural fix that pays for everything

**Attended sentences must carry their scoreboard.** Today the product's central
claim — confidence is earned by scoring — lives one click away (receipts) or in
13px gray (voice). Put a single small chip under every *resolved* sentence:

> survived ×4 · never wrong        (or)        not yet tested — held lightly

This is not a new concept; it is the receipts' first line promoted to the stage. It
makes the question's ranking self-evident, the fall's cost legible (the chip turns
*shaky*), the admission's "held lightly" visible, and the waking's headline
credible — in every beat, the evidence arrives with the sentence.

## 6 · The five-minute keynote (what Jobs would actually show)

One personal record. One narrator. One new symbol per scene. Every beat: say the
sentence, *then* fire the visual.

- **0:00 Cold open (30s).** Black. "I've been thinking." The waking, with one line
  spoken over it: *"This is three months of my life — everything it believes, at
  the height it earned."* Silence. Let them look.
- **0:30 It has standards (75s).** Type a belief live, out loud. The counter-question
  appears. Presenter to audience: "Watch — it won't take it." Beat at the empty
  field. The refusal, human sentence first. Then answer honestly; the ghost
  solidifies and settles low: *"Held — lightly. It hasn't earned height yet."*
- **1:45 It answers with your own record (60s).** "Should I keep running in the
  evenings?" The sky rearranges; sentences arrive stamped with their scoreboards;
  a dated quote from the presenter's own past rises. *"It didn't generate an
  answer. It showed me what I already know — ranked by how often I've been
  right."*
- **2:45 The turn (60s).** Open the heavy conviction. Two lines, large: what would
  change its mind; survived ×4. Visibly press **it turned out wrong**, typing the
  reason out loud. The cascade, the drain, the fall, the ripple. Say nothing for
  three full seconds. Then: *"Name another product that shows you the cost of
  being wrong."* Then the anti-fall: mark another one "held up" and watch it climb.
- **3:45 The proof (45s).** "One more thing about this record — it's append-only and
  content-addressed. If anyone edits one byte of its past—" tamper fires, the sky
  goes dark: *"I would rather go dark than lie."* Restore; two-second waking.
- **4:30 Close (30s).** Descend through the fossils — the newest one still legible.
  Rise to the sky at rest. *"An institution of one."* Black.

What this order does: standards before intelligence (trust before magic), the fall
after the audience knows what height means, the ledger proof after they care, and
the graveyard as the closing image — because the product's most defensible claim is
not that it's smart, but that it's honest about when it wasn't.

## 7 · The uncomfortable truths (no fix inside the UI)

1. **The mock model ceiling.** With no provider attached, reflection produces
   template convictions with generic falsifiers ("a counterexample is observed").
   One audience question — "ask it something hard" — exposes this. The five-minute
   demo must run with a real model attached, or never invoke reflection on stage.
2. **Day-two risk.** The Minute is choreographed; day two is a user, alone, typing
   into the line. The product's daily loop (observe → reflect → resolve) has no
   staged on-ramp after the first hour. That is the next design problem, and no
   amount of keynote polish substitutes for it.
3. **The demo record is a liability until curated.** Founder-flavored, accumulating
   scar tissue across runs, with a tension nobody fires. A keynote-grade seed
   profile is content work, not code work — and it is currently the weakest
   ingredient of the strongest beat.
