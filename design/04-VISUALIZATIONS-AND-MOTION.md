# NEXUS Desktop — Visualizations & Motion

**Version 1.0 · July 2026**

These visualizations exist in no current AI application because no current AI application has
the data. Each one is a *question made visible*, computed entirely from implemented backend
events — no synthetic data, no illustrative approximations. If the fold says 0.5, the dot sits
at 0.5.

---

## 1. The signature visualizations

### 1.1 FoldTrack — confidence as a fold, not a feeling
**Question: how certain, and how was that certainty earned?**
Data: `memory.resolution/1` events per bet; calibration `(1+held)/(2+held+2·falsified)`.

A horizontal track from 0 to 1. A hollow tick marks the 0.5 prior ("unscored"). Each
resolution is a notch placed chronologically left→right along a subtle time axis *under* the
track: ✓ (held, `ep-live`) above, ✕ (falsified, `ep-falsified`) below. The calibration dot
sits at the current fold value; hovering any notch shows note + `by` + date and the dot
ghosts to its value *as of that notch* — scrubbing the history of earned trust. Unjustified:
the entire track dims to 25% opacity (the retrieval discount, ×0.25, made literal) with a
`◌` badge. Micro variant (in table rows): notches + dot only, 64 px wide.

### 1.2 The Mortality Wall (Graveyard)
**Question: what did you believe that died, and what killed it?**
Data: terminal resolutions with notes; the fired falsifier.

Dead beliefs as cards in a chronological wall. Statement struck through; **killed by** in
red with the killing note and its author (`by user`, `by sweeper`, `by reconcile`,
`by settlement.flow` — each attribution glyphed). The falsifier that fired is highlighted
inside the Dies-if list; the ones that never fired stay gray — you *see* which tripwire
caught it. A thin year-axis on the left. This is the product's proudest screen: most AI
products hide their failures; NEXUS frames its.

### 1.3 Cascade view — RECONCILE as a visible law
**Question: what happens to knowledge built on a dead premise?**
Data: `premises` edges; unjustified resolutions authored `by reconcile`.

In the Graph, when a bet falsifies: the node's glyph flips ● → ✕ (400 ms), then
unjustification propagates outward along premise edges breadth-first, one 60 ms step per
edge-depth: each dependent's edge briefly pulses, node dims to the unjustified state ◌.
The animation *is* the mental model: conclusions never silently outlive their premises.
Preview form (press-and-hold, no commitment): dependents pulse violet without state change,
labeled "if this dies: 4 unjustified."

### 1.4 Settlement — a caution retires because mechanics said so
**Question: how does an approval change what the system fears?**
Data: `inbox::approve` settled ids; premortem falsifiers.

On approve, each settled premortem card: its settling falsifier line underlines
(`ep-caution` → `ep-live`), the card stamps "settled — falsifier met mechanically," then
translates down-and-away (400 ms) into the resolutions feed. The symmetric case on reject:
cited cautions each tick +1 held (✓ notch pops onto their FoldTrack, 200 ms). Users watch
fears being *scored*, both directions.

### 1.5 Horizon strip — beliefs age visibly
**Question: what must be re-earned soon?**
Data: `Bet.horizon`; sweep results.

A compact strip (Overview + Beliefs header): each horizoned belief is a dot on a shared
time axis from now → 90 d. Dots amber-shift and drift left as horizons approach; at
expiry the sweeper ledger event drops the dot into an "expired" tray. Belief cards with
near horizons desaturate their statement text by up to 15% — aging is felt at the exact
place the belief is read, then confirmed by the strip. (Sweeps run at consolidation, and
the UI labels the strip "swept at last consolidation" — no pretending it's real-time.)

### 1.6 Working-set lens — "what the model saw"
**Question: was the judgment wrong, or was its context wrong?**
Data: `memory.derivation/1` + `memory.workingset/1`.

For any machine judgment (plan, critique, appraisal): a two-column view — left, the exact
working set (files, in-scope beliefs, cautions, recalls — each with TrustGlyphs); right,
the output artifact. Header: reasoner, charter@version, manifest hash (HashChip), and the
min-trust → capability-ceiling computation rendered as `min(◆●◐◇) = ◇ external → ceiling R1`
with the StakesMeter capped visually. This is the post-mortem instrument: wrong-belief vs
wrong-working-set becomes a visual diff between two derivations.

### 1.7 Contradiction pair — opposition, typographically
**Question: which of these survives?**
Data: `contradictions()` negation-parity pairs.

Two statement cards face each other across a thin vertical rule. Shared content words render
`text-secondary` (they're the *same*); the negation terms render full-strength with a small
⚡ marker between the cards. Each side shows its FoldTrack and evidence count — the reader
sees not just *that* they conflict but *which one has earned more*. Actions: resolve either
(falsified), or leave both ("surfaced, never auto-resolved" is printed on the component).

### 1.8 Record pulse — integrity as an instrument readout
**Question: is the log intact, and how big is the truth?**
Data: `verify`, segment list, tombstone count.

Integrity screen header: segments as a horizontal band of blocks (width ∝ size), each
flashing brief green as its checksums pass during verify; torn frames notch the band amber;
the final content address prints beneath in large mono like a serial number on an
instrument plate. Forgotten events appear as permanent thin gaps in the band — deletion
leaves a visible scar, honestly.

### 1.9 Reasoner strata — trust in the brain, per duty
**Question: which model has earned which job?**
Data: scorecard rows per (provider, model, operator).

Small-multiple horizontal bars grouped by operator (deliberate / review / appraise): each
model's approval rate as a filled bar over its call count (bar thickness ∝ volume, so a
90% rate over 3 calls looks appropriately thin next to 78% over 200). Latency as a
right-aligned mono column, not a chart. No radar charts, no gauges.

## 2. Visualization rules

1. Every mark is computed from ledgered events; tooltips cite the underlying event ids
   (RefChips) — even charts have provenance.
2. Only the epistemic palette + neutrals. One hue = one meaning, app-wide.
3. No smoothing, no interpolation of epistemic values: folds are step functions and render
   as steps. A curve would assert continuity the data doesn't have.
4. Time axes always run left→right, labeled absolutely on hover (relative labels at rest).
5. Empty visualizations state *why* they're empty and which verb fills them.

## 3. Microinteraction catalog (the complete set)

| Trigger | Motion | Duration | Meaning taught |
|---|---|---|---|
| Resolution recorded | notch pops onto FoldTrack; dot slides to new fold | 400 ms deliberate | confidence is recomputed, not adjusted |
| Falsification | status glyph ●→✕ flip; card left-edge floods red 2 px | 400 ms | death is a state change, not a deletion |
| RECONCILE cascade | §1.3 staggered propagation | 60 ms/depth | consequences travel the graph |
| Settlement | §1.4 underline → stamp → depart | 400 ms | mechanics, not mood, retire fears |
| Reject submitted | reason field → Incident chip → premortem chip slides toward Beliefs rail item | 600 ms total | rejection creates memory |
| Horizon nearing | statement desaturation (continuous, not animated) | — | beliefs age |
| Forget confirmed | row content dissolves to `— forgotten —`; keyring "click" haptic-tick sound optional-off | 400 ms | erasure is real and irreversible |
| Verify pass | segment band sweep §1.8 | ~40 ms/segment | integrity is checked, not assumed |
| Palette open | fade+2 px rise | 120 ms | speed = trust |
| Inspector object change | content cross-fade, anatomy fixed | 200 ms | same questions, every object |
| Inbox arrival | badge increments with 1 px dip; status-bar count updates; no sound, no toast | 120 ms | the system waits; it never interrupts |
| Belief hover (anywhere) | Dies-if line underlines faintly | 120 ms | falsifiers are the handle |

All `motion-deliberate` sequences degrade per reduced-motion (`02-…` §7). Nothing loops,
nothing idles, nothing pulses while the user is reading. The interface is still unless the
epistemic state is changing — stillness is the brand.

---

*Next: `05-ONBOARDING.md`.*
