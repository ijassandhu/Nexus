# NEXUS Desktop — The Instrument Design System

**Version 1.0 · July 2026 · Dark mode first; light mode equally normative**

Every value here is a semantic token. Components consume tokens, never raw values. Raw values
appear only in the two theme definitions (§2). Token names are the API between design and code.

---

## 1. Foundations

### 1.1 Grid & spacing

4 px base unit. The scale is closed — no off-scale spacing anywhere.

| Token | px | Use |
|---|---|---|
| `space-1` | 2 | glyph-to-glyph, chip internal |
| `space-2` | 4 | icon-to-label |
| `space-3` | 8 | intra-component padding |
| `space-4` | 12 | compact row padding, card internal |
| `space-5` | 16 | card padding, section gaps |
| `space-6` | 24 | panel padding, group separation |
| `space-7` | 32 | screen margins |
| `space-8` | 48 | major section breaks (onboarding, empty states) |

Layout grid: the shell is a fixed three-region layout (rail / primary / inspector, see
`03-…` §2). Inside the primary region, content uses a 12-column fluid grid with `space-6`
gutters; tables and logs ignore the grid and run full-width.

Density modes: **Comfortable** (row height 40 px) and **Dense** (row height 28 px), a global
setting. All list components implement both from the same tokens (`row-h-comfortable`,
`row-h-dense`).

### 1.2 Radius & borders

| Token | Value | Use |
|---|---|---|
| `radius-1` | 4 px | chips, inputs, small controls |
| `radius-2` | 6 px | cards, buttons, list rows |
| `radius-3` | 10 px | panels, dialogs, palette |
| `border-hairline` | 1 px `stroke-subtle` | default separation everywhere |
| `border-strong` | 1 px `stroke-strong` | focused/selected containers |

Borders, not shadows, are the primary separator in dark mode. No border radius above 10 px —
pill shapes are reserved exclusively for ULID reference chips (`radius-full`).

### 1.3 Elevation

Three levels, semantic:

| Token | Meaning | Dark rendering | Light rendering |
|---|---|---|---|
| `elev-0` | resting surface | `bg-1`, hairline border | `bg-1`, hairline border |
| `elev-1` | raised (hover cards, dropdowns, inspector when floating) | `bg-2` + border | `bg-1` + shadow-sm |
| `elev-2` | overlay (palette, dialogs, forget ceremony) | `bg-3` + border-strong + shadow-lg (dark shadows allowed only here) | `bg-0` + shadow-lg |

Scrim under `elev-2`: `overlay-scrim` (black 40% dark / 25% light). No blur.

## 2. Color

### 2.1 Neutrals — "graphite & paper"

Slightly warm neutral ramp (no blue cast — blue-blacks read "tech demo"; warm graphite reads
"instrument"). Values are the theme layer; everything else references the semantic tokens.

| Token | Dark | Light | Role |
|---|---|---|---|
| `bg-0` | `#0E0F11` | `#F7F7F5` | app background |
| `bg-1` | `#141518` | `#FFFFFF` | primary surfaces (cards, panels) |
| `bg-2` | `#1A1C20` | `#F1F1EE` | raised surfaces, hover fill |
| `bg-3` | `#22242A` | `#E9E9E5` | overlays, active fill |
| `stroke-subtle` | `#26282E` | `#E3E3DE` | hairline borders |
| `stroke-strong` | `#3A3D45` | `#C9C9C2` | focus containers, selected |
| `text-primary` | `#E9EAEC` | `#1B1C1E` | statements, values |
| `text-secondary` | `#A6AAB3` | `#5A5D64` | labels, metadata |
| `text-tertiary` | `#6E727D` | `#8A8D94` | timestamps, ids at rest |
| `text-disabled` | `#4A4E58` | `#B4B6BC` | disabled only |

Contrast floor: all text tokens ≥ 4.5:1 against their permitted surfaces (verified per theme);
`text-tertiary` is never used below 12 px.

### 2.2 Accent — one, and it means "interactive"

| Token | Dark | Light | Role |
|---|---|---|---|
| `accent` | `#8FA8D8` (muted periwinkle-steel) | `#3D5A9E` | focus rings, links, selected nav, primary buttons |
| `accent-subtle` | `accent` @ 14% | `accent` @ 10% | selected row fill |

The accent is deliberately *not* teal/cyan (AI cliché) and *not* used for any epistemic
meaning. If something is periwinkle, it is clickable or selected — nothing else.

### 2.3 The epistemic palette — color is reserved for outcome

Color in NEXUS carries exactly one message: *what happened to this position*. These six tokens
are the only saturated colors in the application besides `accent`.

| Token | Dark | Light | Bound to (exact backend value) |
|---|---|---|---|
| `ep-live` | `#4CA98A` | `#2E7D5F` | `BetView.status == "live"` · `Outcome::Held` ticks |
| `ep-falsified` | `#D95C5C` | `#B23A3A` | `Outcome::Falsified` · "killed by" |
| `ep-expired` | `#C9A24B` | `#9A7626` | `Outcome::Expired` · horizon warnings |
| `ep-retracted` | `#7E838E` | `#75787F` | `Outcome::Retracted` (evidence forgotten) |
| `ep-unjustified` | `#A98FD8` | `#7A5AB8` | `unjustified: true` flag (RECONCILE) |
| `ep-caution` | `#C97B4B` | `#A85E2E` | premortems in a working set ("cautions") |

Verdicts reuse the palette: `approve → ep-live`, `veto → ep-falsified`, `revise → ep-expired`.
Ledger signature verdicts: `Valid → ep-live`, `Invalid → ep-falsified`, `Unsigned →
text-tertiary`. Effect ops: `Created → ep-live`, `Modified → ep-expired`-family amber,
`Deleted → ep-falsified` — matching universal diff conventions so developers read it instantly.

Rules:
- Epistemic colors appear as **text, glyphs, 2 px left-edge bars, and thin track fills** — never
  as large filled areas. A falsified belief is a card with a red edge and red status line, not
  a red card.
- Charts use only this palette plus neutrals (see `04-…`).
- Each epistemic color has an `-subtle` variant (@12% fill) for backgrounds of chips.

### 2.4 Color-independent redundancy (accessibility)

Every epistemic state is also encoded by a **glyph** so no meaning is color-only:

| State | Glyph | Rationale |
|---|---|---|
| live | `●` filled dot | present, load-bearing |
| falsified | `✕` cross | killed |
| expired | `◔` clock-quarter | time ran out |
| retracted | `⊘` struck circle | evidence withdrawn |
| unjustified | `◌` dashed/hollow dot | still standing, foundation gone |
| held (resolution tick) | `✓` | survived contact |

## 3. Typography

Two families, loaded locally (no network fonts — local-first product, local-first assets):

- **UI: Inter** (variable). Statements, labels, controls. Tracking −1% above 16 px.
- **Data: JetBrains Mono**. ULIDs, hashes, paths, excerpts from the record, effect lists,
  falsifier text in compact contexts. Monospace is the typographic marker for *verbatim
  record content vs. interface chrome* — a reader can always tell what the system said from
  what the record contains.

| Token | Size/leading | Weight | Use |
|---|---|---|---|
| `type-display` | 28/34 | 600 | onboarding moments, empty states |
| `type-title` | 20/26 | 600 | screen titles |
| `type-heading` | 15/22 | 600 | panel & section headings |
| `type-body` | 13/20 | 400 | default UI text, statements in lists |
| `type-body-strong` | 13/20 | 570 | belief statements (the most important text in the app) |
| `type-label` | 12/16 | 500 | field labels, column headers (+2% tracking, no all-caps except column headers) |
| `type-micro` | 11/14 | 450 | timestamps, counts |
| `type-mono` | 12/18 | 450 mono | ids, hashes, paths |
| `type-mono-block` | 12.5/20 mono | 400 | excerpts, effect lists, ledger entries |

Numerals: `font-variant-numeric: tabular-nums` on all counts, scores, and timestamps.
Belief statements render in `type-body-strong` at full `text-primary` — the single most
emphasized text class in the product, by design.

## 4. Iconography & the provenance glyph system

Icon set: custom geometric, 16 px grid, 1.5 px stroke, squared terminals — instrument-panel
character, no rounded-friendly styling. ~40 icons v1 (nav, verbs, states). No filled icons
except the status glyphs above.

**Trust origin glyphs** (bound to `Trust` enum, ordered by fill = authority):

| Trust | Glyph | Form |
|---|---|---|
| `user` | ◆ | filled diamond — the human's own word |
| `local` | ● | filled circle — this machine's instruments |
| `derived` | ◐ | half-filled circle — NEXUS reasoning |
| `external` | ◇ | hollow diamond — outside authority, never trusted by default |

These glyphs appear on every event row, every evidence citation, and every belief (its
inherited trust). Fill = authority is learnable in seconds and works in monochrome.

**Stakes meter** (bound to `stakes: R0–R3` and `EffectClass`): four ascending bars `▁▃▅▇`,
filled left-to-right. R0 = one bar. Same component renders capability `max_effect` and the
trust ceiling — the backend deliberately shares this taxonomy; the UI shares the glyph.

**Privacy** (`P0–P3`): a padlock glyph with 0–3 tick marks, `text-tertiary`, shown only in
event detail and settings (not in lists — privacy is present but not yet router-enforced,
and the UI must not oversell it).

## 5. Core components (the primitive library)

Each primitive names its backend binding. Components not bound to real data do not exist.

| Component | Binding | Contract |
|---|---|---|
| **StatusGlyph** | `BetView.status` + `unjustified` | glyph + color per §2.3/§2.4; tooltip gives plain-language state ("expired — horizon passed, not re-earned") |
| **TrustGlyph** | `Origin.trust` | §4 glyphs; tooltip: "external — originated outside your authority" |
| **StakesMeter** | `Bet.stakes` / `Capability.max_effect` | 4-bar meter + tooltip taxonomy |
| **RefChip** | any ULID | pill, `type-mono`, first 4 + last 4 chars (`01KW…D4D6`); click navigates, hover previews the referent; copy on ⌥-click |
| **HashChip** | `body_hash`, content address, before/after | 8-hex-char truncation, expand on hover, copy affordance |
| **FoldTrack** | resolutions fold | the confidence primitive: horizontal track, calibration dot at `(1+held)/(2+held+2·falsified)`, prior mark at 0.5, one notch per resolution (✓ green above track, ✕ red below), ×0.25 dimming overlay when unjustified. Never displays a bare percentage without the notch history that produced it |
| **DiesIf** | `falsifiers[]` | the falsifier list, prefixed "Dies if —"; always visible on belief cards (may truncate to first falsifier + "＋2"; never fully hidden) |
| **KilledBy** | terminal resolution `note` + `by` | red-edged citation block on dead beliefs |
| **EvidenceRef** | `refs[]` / provenance | RefChip + TrustGlyph + excerpt on hover (decrypt-on-view) |
| **EffectRow** | `Change {op, path, before, after}` | op glyph (A/M/D colored), mono path, before→after HashChips |
| **VerdictBadge** | Critique `verdict` | approve/revise/veto chip; non-approve always expands its `issues[]` with targets |
| **AdvisoryBlock** | `advisory_summary` | collapsed-by-default block labeled "advisory · model-written" in `text-tertiary` italic; visually subordinate to EffectRows (P9) |
| **CautionCard** | premortem in working set | `ep-caution` edge; shows statement + which falsifier could settle it |
| **HorizonBadge** | `Bet.horizon` | relative time ("re-earn in 12 d"); switches to `ep-expired` amber inside 20% of horizon span |
| **ContradictionPair** | `contradictions()` pairs | two statements side-by-side, shared terms muted, negation terms highlighted; actions: resolve either, or dismiss (keeps both live — surfaced, never auto-resolved) |
| **SigBadge** | ledger `Verdict` | `sig ✓` green / `sig ✗ INVALID` red / `unsigned` gray |
| **EventRow** | `EventHeader` | ts (tabular) · kind icon · schema (mono) · TrustGlyph · body preview or `— forgotten —` |
| **Panel / Inspector / DataTable / Timeline / CommandPalette / Toast** | shell primitives | standard; DataTable is virtualized, keyboard-navigable (j/k/↑↓, Enter opens Inspector), all columns sortable/filterable |

Forbidden components: progress spinners longer than 400 ms (use skeleton rows), modal
confirmation for reversible acts (only `forget` and `reject` get ceremonies — both are
genuinely consequential), percentage rings for confidence (implies asserted certainty).

## 6. Interaction standards

- **Keyboard map (global):** ⌘K palette · ⌘1–7 navigation rail · ⌘\ toggle inspector ·
  ⌘J attach "Ask the record" drawer to current object · `[` `]` prev/next in list ·
  Enter open · Esc up one level · `?` shortcut overlay. List screens use j/k, `f` filter,
  `s` sort. Inbox: `a` approve (opens effect-list confirm), `r` reject (focuses reason
  field — reason is mandatory, the backend refuses without it).
- **Focus:** 1.5 px `accent` ring, 2 px offset, visible on every focusable element, never
  suppressed. Roving tabindex in tables; skip-links in the shell.
- **Selection vs. focus** are distinct: selection = `accent-subtle` fill; focus = ring.
- **Hover previews** (RefChip, EvidenceRef) open after 350 ms, `elev-1`, and are themselves
  focusable via long-press of the key path (⌘-hover equivalent: `p` on focused chip).
- **Empty states** teach philosophy: the empty Beliefs screen says *"Nothing is believed yet.
  NEXUS only stores positions that can lose — place one, or add notes and consolidate."* with
  both verbs as buttons.
- **Errors are quoted, not paraphrased:** backend refusals (e.g., the admission rule) render
  verbatim in a mono block — the rule text is the product speaking.

## 7. Motion

| Token | Value | Use |
|---|---|---|
| `motion-fast` | 120 ms, ease-out | hover fills, chips |
| `motion-standard` | 200 ms, cubic-bezier(0.2, 0, 0, 1) | panel slide, selection, palette |
| `motion-deliberate` | 400 ms, same curve | epistemic state changes: fold recompute, settlement retirement, status transitions |
| `motion-cascade-step` | 60 ms stagger | RECONCILE propagation across dependents |

`prefers-reduced-motion`: all `motion-deliberate` sequences replace with a 120 ms cross-fade
plus a persistent change-marker (colored dot on the changed element for 5 s) so meaning is
not lost with motion. Canonical epistemic animations are specified in `04-…` §3.

## 8. Accessibility (WCAG 2.2 AA floor)

- Contrast verified per theme for every (text-token, surface-token) pair in use; epistemic
  colors have text-safe variants where used as foreground.
- All meaning triple-encoded: color + glyph + text (§2.4).
- Full keyboard operability including graph view (tab-order = reading order of nodes;
  arrow-key traversal along edges — an edge traversal *is* the provenance question).
- Live regions: inbox arrivals, resolution outcomes, and verify results announced politely.
- Hit targets ≥ 24×24 px even in dense mode (glyphs get invisible padding).
- Both themes are first-class: tokens forbid any component from referencing a raw hex value,
  so light mode cannot rot.

## 9. Voice & microcopy

Register: laboratory notebook, not assistant. Rules:
- The system is "the record" / "NEXUS," never "I." Beliefs are "held/live," never "I'm sure."
- Canonical vocabulary is the backend's: *bet, belief, prediction, premortem, falsifier
  ("Dies if"), resolution, held, falsified, expired, retracted, unjustified, RECONCILE,
  settle, caution, effect list, advisory, reasoner, charter, working set, forget (shred).*
- Numbers over adjectives: "held ×4 · falsified 0 · since 12 Mar" not "high confidence."
- Refusals quote their rule: *"no falsifiers, no bet."*

---

*Next: `03-ARCHITECTURE-AND-SCREENS.md` — navigation, every screen, every journey.*
