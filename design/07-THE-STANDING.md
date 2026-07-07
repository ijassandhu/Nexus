# NEXUS Desktop — The Standing

**Version 1.0 · July 2026 · The keynote screen, and the application built around it.
Supersedes the navigation-first home of `03-ARCHITECTURE-AND-SCREENS.md`; everything else in
`design/01–06` (tokens, voice, component semantics, quality bars) still binds.**

---

## 1. The one screen people remember

If NEXUS gets ninety seconds on a keynote stage, the audience sees a single composition:

```
 1.0 ┄
                    ┌─────────────────┐
 0.75┄              │ ● belief   0.80 │        ┌─────────────────┐
                    │ we require code │        │ ● belief   0.67 │
                    │ review on main  │        │ small PRs, same │
 0.50┄┄ prior ┄┄┄┄  │ DIES IF — a …   │  ┄┄┄┄  │ day             │ ┄┄┄
        ┌───────────│ ✓✓✓✓            │        │ DIES IF — …     │
 0.25┄  │ ◌ 0.13 UNJUSTIFIED          │        │ ✓✓              │
        │ friday cleanup jobs fine    │
━━━━━━━━━━━━━━ THE GROUND ━━━ what fell lies below, with what killed it ━━━━━━━━
   ┌────────────────────────────┐   ┌────────────────────────────┐
   │ ~~staging db safe to wipe~~│   │ ~~tasks like "add config"  │
   │ KILLED BY — July wipe      │   │ fail without schema…~~     │
   │ destroyed analytics data   │   │ KILLED BY — falsifier met  │
   │ ✕ falsified · by user      │   │ mechanically · settlement  │
   └────────────────────────────┘   └────────────────────────────┘
```

**One line of physics explains the whole product: altitude is earned confidence, and the
ground is death.**

- Every live belief floats at the altitude of its **fold** — the exact backend value
  `(1+held)/(2+held+2·falsified)`. Nothing is asserted; a new belief enters at the 0.50
  prior line labeled *"unscored — earns altitude by surviving."*
- Every card carries its **DIES IF —** line. Falsifiers are never collapsed away; a belief
  is visually inseparable from its killing condition.
- **Unjustified** beliefs (RECONCILE flag) sink to `fold × 0.25` — the literal retrieval
  discount — and dim violet. Shaky foundations are *low*, not annotated.
- Below the ground line: the **graves**. Struck statements, **KILLED BY —** with the killing
  note and who resolved it (`by user`, `by sweeper`, `by settlement.flow`, `by reconcile`).
  Most AI products hide their failures; this one frames them at the center of the home screen.

The keynote beat: the presenter records one piece of evidence, resolves a belief falsified —
the card **falls through the ground** (600 ms, `motion-deliberate`) and every belief standing
on it pulses violet and sinks, untouched by any hand. "Memory that can lose," demonstrated,
zero paragraphs.

### Why this composition (and not a table, graph, or feed)

- A table says *database*. A chat says *assistant*. A field with a ground line says *these
  things are alive, positioned by merit, and mortal* — which is the invention.
- The y-axis is not a metaphor; it is the calibration function plotted. The screen is honest
  enough to put gridlines and numbers on it (1.0 / 0.75 / 0.50 prior / 0.25 discounted).
- Spatial memory (P7): cards move only when their fold moves, and movement is meaning —
  rising = held, sinking = unjustified, falling through = death.

## 2. Everything else orbits the Standing

The application is four surfaces; three exist to feed the field.

| Surface | Role relative to the Standing |
|---|---|
| **Standing** (home) | the state of what is believed; place · resolve · inspect · witness bar |
| **Inbox** | verdicts that move the field: approve → premortems fall (settlement); reject → a new caution rises, minted from your reason |
| **Record** | the ground truth under the field: episodes, recall, forget ceremony (which can retract beliefs above) |
| **Reasoners** | the replaceable brain, scored; attach/swap providers — the field survives the swap |

Elements on the Standing itself:
- **Witness bar** — one input: *"Tell the record what you saw."* Notes become episodes;
  **consolidate** appraises them into candidate beliefs through the admission rule (rejected
  candidates surface with the rule quoted, verbatim).
- **Inspector** (right panel, any card) — the six questions: why (evidence chips), since when,
  how certain (fold shown *as the formula*), dies if, structure (premises/dependents +
  "if this dies: N become unjustified"), and resolve verbs.
- **Place a belief** — the admission rule as a form: submit disabled until a falsifier exists;
  the backend refusal renders verbatim on any attempt to cheat it.
- **Status bar** — permanent integrity strip: `● verified · N events · addr …` from `verify`,
  plus the active route and inbox count.
- **⌘K palette** — verbs (place/delegate/consolidate) + every belief statement as a noun.

## 3. Implementation shipped (v0.3 surface)

`crates/desktop` — `nexus-desktop`, a deliberately thin, std-only localhost bridge
(zero new dependencies; builds offline) that holds the open record and maps HTTP routes 1:1
onto the same substrate/runtime calls the CLI makes. The UI is three static files served by
the same binary. Non-spec and replaceable by design; the planned Tauri shell (06 §1) wraps
these same assets unchanged.

```
cargo run -p nexus-desktop -- --data .nexus --passphrase <pass>   # opens the browser
```

Verified end-to-end on a real record (all through the UI's own API):
unfalsifiable belief refused with the rule verbatim · placement → card rises to the prior ·
held → altitude rises · falsified → the fall + RECONCILE pulse (cascade count from the
backend) · consolidation places cited beliefs and rejects the unfalsifiable candidate ·
contradiction surfaced, never resolved · reject mints a scoped premortem · the next task
heeds it · approval settles it mechanically (`settled:[id]`) and it drops into the graves ·
forget ceremony discloses which live beliefs cite the episode before shredding.

Honesty constraints carried from 01–06: no fake data anywhere (every pixel is an API value);
the advisory summary is collapsed and labeled under the kernel's effect list; color = outcome,
glyph = provenance, monospace = verbatim record; folds render as the formula, never a bare
percentage.
