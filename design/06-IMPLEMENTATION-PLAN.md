# NEXUS Desktop — Implementation Plan

**Version 1.0 · July 2026 · Status: approved-pending-review; no code until this is signed off**

The backend is frozen; the desktop app is a *surface* over the existing crates. Per
ARCHITECTURE.md §2, UI is a non-spec component — assumed replaceable — so no user value may
live only here. Every feature below is a projection of substrate/kernel/runtime state.

---

## 1. Platform: Tauri 2

ARCHITECTURE.md §11 already names Tauri for the Delegation Inbox; we extend that decision to
the full app. Rationale: the backend is Rust — the `substrate`, `kernel`, and `runtime`
crates link **directly into the Tauri process** as a library (no HTTP daemon, no serialization
boundary to invent, no plaintext crossing a socket). Passphrase/KEK stay in one process.
Native menus, tray, and file dialogs; ~10 MB binaries; Windows/macOS/Linux from one codebase.

- New crate: `crates/nexus-desktop` (Tauri shell) depending on `runtime` exactly as `nx` does.
  The CLI remains the reference surface; **command parity is a CI-enforced invariant** (every
  Tauri command maps 1:1 to an existing `runtime`/`substrate` API also reachable via `nx`).
- Session model: app launch = `Substrate::open` with passphrase (Act 0 / unlock screen);
  KEK held in-process, zeroized on lock (existing `zeroize`-style handling in substrate).
- Long operations (verify, rebuild, consolidate, pipeline runs) execute on a Tokio worker
  pool; progress streamed to the UI via Tauri events (`verify:segment`, `pipeline:stage`…).

## 2. Frontend stack

| Layer | Choice | Why |
|---|---|---|
| Language | TypeScript, strict | table stakes |
| UI | React 19 | ecosystem, concurrent rendering for virtualized logs |
| State/server | TanStack Query over `invoke()` | backend is the single source of truth; queries invalidate on Tauri events — no client-side duplicate of epistemic state, ever |
| UI-only state | Zustand (panel sizes, density, selection) | tiny, persistable |
| Styling | CSS custom properties (tokens) + vanilla-extract | tokens are the design-system API (`02-…`); themes are two `:root` blocks; zero runtime CSS-in-JS cost |
| Tables/logs | TanStack Virtual | millions of events (personal scale) |
| Graph | Custom SVG + d3-force (layout only, run once, positions persisted) | spatial memory requirement (P7) rules out auto-layout libs |
| Charts | Hand-rolled SVG primitives (FoldTrack, strips, bars) | our visualizations don't exist in chart libraries; steps-not-curves rule |
| Icons | In-house 16 px set (SVG sprites) | Instrument character |
| Fonts | Inter + JetBrains Mono, bundled | local-first, no network fetch |
| Tests | Vitest + Testing Library; Playwright (tauri-driver) for journeys | §5 |

Explicitly rejected: Electron (footprint, second runtime), Next.js (no server), component
kits (Radix acceptable for a11y primitives — menu/dialog/focus — but visual components are
bespoke; a kit's look would defeat the design language).

## 3. Architecture

```
src/
  tokens/          themes.css, tokens.ts          (generated from 02-DESIGN-SYSTEM tables)
  primitives/      StatusGlyph, TrustGlyph, RefChip, FoldTrack, DiesIf, EffectRow, …
  shell/           Rail, StatusBar, Inspector, CommandPalette, AskDrawer
  screens/         overview/ beliefs/ inbox/ tasks/ record/ graph/ reasoners/ integrity/ settings/
  data/            commands.ts (typed invoke bindings), queries.ts, events.ts
  motion/          cascade.ts, settlement.ts, fold.ts   (the canonical sequences, one impl each)
  a11y/            focus, announcer, shortcuts registry
```

Principles:
- **Primitives own semantics.** Only `FoldTrack` may render confidence; only `DiesIf` may
  render falsifiers. Screens compose primitives; grep-able guarantee that an epistemic value
  is never displayed two ways.
- **Typed command layer.** Rust structs (`BetView`, `InboxItem`, `Change`, `ScoreRow`,
  `Report`, `RunResult`…) are exported to TS via `specta`/`tauri-specta` — the UI cannot
  drift from backend field names.
- **Read model = derived index + decrypt-on-view.** Lists read headers from SQLite
  (`DerivedStore::query`); bodies decrypt only for the Inspector/detail (matches the
  backend's own privacy posture). No decrypted content is cached beyond the view.
- **Event-driven invalidation.** Every append publishes a Tauri event with the schema
  string; queries subscribe by schema prefix (`memory.bet` invalidates Beliefs, `txn.`
  invalidates Inbox/Integrity…). No polling.
- Graph node positions: persisted in UI-state store keyed by record identity — *not* in the
  record (layout is not knowledge).

## 4. Performance budgets (CI-measured)

| Metric | Budget |
|---|---|
| Cold start → interactive (unlock screen) | < 1.5 s |
| Unlock → Overview rendered | < 800 ms (10k-event record) |
| Palette open | < 50 ms |
| List scroll | 60 fps, < 8 ms scripting/frame |
| Recall query (10k events) | < 300 ms perceived (progressive) |
| Memory | < 250 MB with 100k-event record open |

## 5. Quality gates

- **No-placeholder rule (from the brief), mechanized:** every screen renders exclusively
  from `invoke()` data; a CI lint forbids literals in `screens/` matching id-like or
  statement-like fixtures. Demo content comes from running the real mock provider against
  a scratch record (`demo/prove-it.sh`'s approach, reused as the UI dev fixture).
- **Journey tests:** the six canonical journeys (`03-…` §4) as Playwright specs against a
  real record with the mock provider — the UI equivalent of `prove-it.sh`.
- **A11y:** axe-core clean on every screen, both themes; keyboard-only journey test;
  reduced-motion snapshot test.
- **Parity check:** script asserts every `nx` subcommand has a registered palette verb.

## 6. Build order (four milestones, each shippable)

1. **M1 — Shell + Record + Beliefs (read).** Unlock, rail, status bar with live verify,
   Inspector, palette (nouns), Beliefs table + Graveyard + FoldTrack, Record log + query +
   recall. *Value: the thirty-second test passes on any existing record.*
2. **M2 — Verbs.** Place belief (admission-rule form), resolve (+cascade animation),
   forget ceremony, consolidate + Candidates review, contradictions view, horizon strip.
3. **M3 — Delegation.** Tasks trail, Inbox (effect-list review, approve/reject, settlement
   motion), working-set lens, capability card, Integrity screen (verify/ledger/tombstones/
   rebuild).
4. **M4 — Brain + polish.** Reasoners (scorecard/routes/providers), onboarding acts 0–5,
   Graph with persisted layout + cascade preview, density modes, light-theme audit,
   performance pass against budgets.

Out of scope (backend-absent, revisit when backend grows them): export/import UI, privacy-
tier routing controls, multi-device, DecisionMemo/SkillProposal surfaces, R2/R3 effect
queue UI beyond what txns expose today.

---

*This completes the specification set. Implementation begins only after these six documents
are reviewed and approved.*
