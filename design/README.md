# NEXUS Desktop — Design Specification

**The first interface built around epistemic memory.** Design language: **Instrument** —
a scientific instrument with an archival ledger, not a chatbot.

> **ARCHITECTURE FROZEN — July 2026.** No new concepts, no new backend systems, no
> new documents. Every future change must improve the experience of a first-time
> user; success is measured by whether a stranger can understand and use NEXUS
> within 30 seconds. The regression gate is `node crates/desktop/ui/home.e2e.js`
> (the complete first-run flow on a fresh record) plus `layout.test.js`. Amend the
> documents below; do not add to them.

| Doc | Contents |
|---|---|
| [01-FOUNDATIONS.md](01-FOUNDATIONS.md) | Research synthesis (12 products → 10 principles), design philosophy, the thirty-second test, anti-goals |
| [02-DESIGN-SYSTEM.md](02-DESIGN-SYSTEM.md) | Tokens: color (epistemic palette), typography, spacing, elevation, glyph system, component library, motion, a11y, voice |
| [03-ARCHITECTURE-AND-SCREENS.md](03-ARCHITECTURE-AND-SCREENS.md) | Concept→surface contract, shell, all 10 screens with wireframes, canonical journeys |
| [04-VISUALIZATIONS-AND-MOTION.md](04-VISUALIZATIONS-AND-MOTION.md) | 9 original visualizations (FoldTrack, Mortality Wall, Cascade, Settlement…), microinteraction catalog |
| [05-ONBOARDING.md](05-ONBOARDING.md) | The first five minutes: six acts, all on real backend paths (mock provider) |
| [06-IMPLEMENTATION-PLAN.md](06-IMPLEMENTATION-PLAN.md) | Tauri 2 + React architecture, quality gates, four milestones |
| [07-THE-STANDING.md](07-THE-STANDING.md) | **The keynote screen** — altitude is earned confidence, the ground is death — and the shipped `crates/desktop` app built around it (supersedes 03's home screen) |
| [08-THE-INDUCTION.md](08-THE-INDUCTION.md) | **The first five minutes, choreographed** — refusal → witness → loss → fear & settlement, directed live over the real backend (supersedes 05's act structure) |
| [09-THE-KEYNOTE.md](09-THE-KEYNOTE.md) | **The seven-minute keynote score** — gasp map, the tamper attack, the no-model reveal — and the product mechanics it shipped: presence line, litmus command, keynote mode |
| [10-THE-ROOM.md](10-THE-ROOM.md) | **The paradigm rethink** — no pages, no cards: the prose, the line, the voice; the language layer and the five moments (kept at `/room` as the grammar reference) |
| [11-THE-SKY.md](11-THE-SKY.md) | **The Sky** (`/sky`) — knowledge as a place with physics: altitude is the earned fold, gravity is falsification, the fall is the identity animation. Part F: the attention law. **Part H: the Home (`/`) — the Room developed into the product; first screen answers "what changed while I was away"** |
| [12-THE-MINUTE.md](12-THE-MINUTE.md) | **The sixty-second keynote demo** (`/?demo`) — **FROZEN**. Second-by-second script and the self-driving director: real record, real API, captions as sole narrator, any keypress hands over the live product. Daily life plays the film exactly once (11 Part G) |
| [13-THE-TEARDOWN.md](13-THE-TEARDOWN.md) | **The adversarial product review** — attention drops, confusion points, the animation cut list, the scoreboard-chip fix, and the five-minute keynote running order. Critique only; nothing implemented |

Ground rules binding all documents:
1. **ARCHITECTURE.md wins** on any conflict; the backend is frozen.
2. **Every UI element maps to implemented code** (see contract table in 03 §1). Doc-only
   concepts (claims, DecisionMemo, privacy-tier routing) are absent, not stubbed.
3. **Chat is a drawer, never the front door** (ARCHITECTURE.md §11).
4. **Color = epistemic outcome; glyph shape = provenance; monospace = verbatim record.**
5. **Confidence is a fold, never a percentage without its history.**
