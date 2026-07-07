# NEXUS Desktop — Design Foundations

**Version 1.0 · July 2026 · Status: normative for the desktop surface**

This document is the research synthesis and design philosophy for the NEXUS desktop
application. It is downstream of the frozen backend architecture (ARCHITECTURE.md wins on any
conflict) and upstream of everything else in `design/`. No screen, token, or interaction ships
unless it traces to a principle here, and no principle here survives unless it traces to a real
backend concept (see the concept inventory in `03-ARCHITECTURE-AND-SCREENS.md` §1).

---

## 1. What we are designing

NEXUS is not an AI you talk to. It is a record you own, with an AI attached to it.

The backend already made the hard choices: beliefs are **bets** that must state what would kill
them; confidence is **never asserted, only earned** as a fold of resolutions; a dead premise
**cascades** unjustification through everything built on it; nothing touches the world without
a **truthful effect list** the user approves; and the models are **swappable peripherals with
scorecards**. ARCHITECTURE.md §11 is explicit: *"Chat exists as the drill-down debugger
attached to artifacts, never as the front door."*

The desktop application's job is to make those invariants *legible in thirty seconds*. Not
decorated. Legible.

## 2. Research synthesis — principles extracted, not copied

Twelve world-class applications were studied for what makes them exceptional. What follows is
the distillation: each principle names its lineage, states what we take, and states what we
deliberately refuse.

### P1. The primary surface is the object of trust, not the conversation
*Lineage: GitHub Desktop, Cursor, Warp.*
GitHub Desktop's entire UI is a diff; Cursor's best moment is diff review, not chat; Warp made
each terminal command a discrete, inspectable block. NEXUS generalizes this: the front door is
the **Record and its beliefs**, and every AI contribution arrives as a reviewable object (an
effect list, a candidate belief, a critique) — never as flowing prose you're asked to trust.
**Refused:** Cursor's pattern of chat as the primary workspace.

### P2. Every noun is addressable; every verb is reachable from anywhere
*Lineage: Raycast, VS Code, Linear.*
Raycast proved that a command palette is not a feature but an operating model: objects have
identities, verbs act on them, and the keyboard reaches both. In NEXUS every belief, event,
transaction, and artifact already has a ULID — the UI treats those as first-class addresses.
⌘K opens the palette; every list row, chip, and citation is a navigable reference.
**Refused:** Raycast's extension marketplace sprawl; NEXUS's verb set is closed and small
(the CLI surface, exactly).

### P3. Status is a typed visual language, applied with total consistency
*Lineage: Linear, Vercel.*
Linear's genius is that state (backlog/started/done) has one iconographic system used
everywhere, so users read status pre-attentively. NEXUS has richer state than any tracker:
bet status (live/falsified/expired/retracted), the unjustified flag, trust origin
(user/local/derived/external), stakes (R0–R3), privacy (P0–P3), verdicts
(approve/revise/veto). Each gets exactly one visual encoding, defined once in the design
system, and never improvised per screen.
**Refused:** Vercel's tendency to encode everything as colored pills; NEXUS reserves *color*
for epistemic outcome and uses *glyph shape* for provenance (see `02-DESIGN-SYSTEM.md` §4).

### P4. Density with calm: chrome recedes, data breathes
*Lineage: Linear, Apple HIG, Figma.*
Apple's principle of deference — content first, chrome minimal — combined with Linear's proof
that 13px type on a strict grid can feel serene. NEXUS screens are dense (a beliefs table, an
event log, a ledger) but the *frame* around them is nearly silent: hairline borders, no
gradients, elevation used sparingly and semantically.
**Refused:** Notion's infinite flexibility (NEXUS is opinionated; layouts are fixed and
learnable) and any glassmorphism/AI-glow aesthetic.

### P5. Provenance is always one interaction away
*Lineage: Obsidian (backlinks), Figma (inspector).*
Obsidian made "where does this connect" a hover. Figma made "what are this object's
properties" a permanent right panel. NEXUS fuses them: every screen has an **Inspector** —
a dockable right panel that answers, for the selected object, the six questions of trust:
*Why? Since when? How certain? Based on what? What would change this? How has it evolved?*
**Refused:** Obsidian's graph-as-ornament. A graph view exists only because the justification
graph (premises → dependents) is real data with a real question attached: "what dies if this
dies?"

### P6. Motion communicates state transition, never decoration
*Lineage: Apple HIG, Arc.*
Arc's animations teach spatial models; Apple's teach causality. In NEXUS, the only things that
animate are the things the philosophy says can change: confidence folds re-compute, cascades
propagate, settlements retire cautions, horizons approach. Each has one canonical motion
(see `04-VISUALIZATIONS-AND-MOTION.md`). Nothing else moves.
**Refused:** ambient particle/shimmer effects, typing indicators, "thinking" theatrics.

### P7. Spatial memory is preserved
*Lineage: Arc, Figma, VS Code.*
Panels remember their sizes. The graph never re-randomizes its layout — node positions persist
per record, so "the belief in the top-left" stays there across sessions. Docked panels restore.
**Refused:** VS Code's unbounded panel complexity; NEXUS ships one shell layout with a small
number of well-chosen degrees of freedom.

### P8. Honesty as an aesthetic: receipts on demand
*Lineage: Warp, GitHub Desktop, Vercel dashboards.*
The backend signs ledger entries, hashes bodies, and content-addresses frames. The UI does not
hide this machinery — it *composes* it: ULIDs render as compact monospace pills, hashes truncate
with expand-on-hover, signature verification is a visible ritual (Integrity screen). The feel
is a scientific instrument that shows its calibration certificate, not a consumer app hiding
its plumbing.
**Refused:** hiding ids and hashes entirely (breaks trust), and dumping raw JSON (breaks calm).

### P9. The advisory is visually subordinate to the mechanical
*Lineage: none — this is NEXUS-original, forced by D-015.*
The backend distinguishes the kernel's raw effect list (truth) from the model's advisory
summary (presentation). No studied product makes this distinction visible; NEXUS must. The
effect list gets primary typography and position; the model's summary is styled as an
annotation — italic label "advisory · model-written," secondary color, collapsible. The user's
eye lands on the truth first, always.

### P10. Speed is a trust property
*Lineage: Linear, Raycast.*
A system asking to be trusted with your beliefs cannot lag. Interaction budget: palette opens
< 50 ms, list navigation < 16 ms/frame, any query over the derived index renders progressively.
Perceived speed is part of the epistemic brand: the record answers instantly because it *knows*.

## 3. Design philosophy — the NEXUS stance

The design language is named **Instrument**. Its reference object is not a chat app or a
notebook but a *scientific instrument with an archival ledger*: a seismograph, a lab balance
with its calibration log, a flight recorder. Things that are trusted because they are
inspectable, calibrated, and indifferent to flattery.

Five commitments, in tension order (earlier wins conflicts):

1. **Truthful before beautiful.** If a visual simplification misrepresents the epistemic
   state (e.g., showing a confidence percentage as if it were asserted rather than folded from
   resolutions), it is forbidden regardless of how clean it looks.
2. **Legible before dense.** Density is welcome only after the status language is learned;
   defaults favor the six trust questions being answerable at a glance.
3. **Calm before expressive.** One accent color, ink-on-paper neutrals, motion only at state
   changes. The record is permanent; the interface should feel like it plans to be.
4. **Keyboard before pointer.** Every action has a key path; the pointer is a convenience.
5. **Explorable before conversational.** The UI privileges browsing, filtering, and following
   provenance edges over asking questions in prose. Chat exists — as a drawer attached to
   objects — and is deliberately the least prominent surface in the app.

### The thirty-second test (success criterion, restated as design constraints)

A first-time viewer, within thirty seconds of the app opening on a lived-in record, must be
able to say all five of these without documentation:

| They should notice… | …because the UI guarantees |
|---|---|
| "These aren't chat logs, they're *beliefs*" | Home is the record overview; belief cards lead with a statement, not a message bubble |
| "Each one says what would kill it" | Every belief card shows its **Dies if** line — falsifiers are never collapsed away |
| "Certainty is a score, not a vibe" | Confidence renders as a **fold track** (held ✓✓✓ / falsified ✗) with the calibration dot, never a bare percentage |
| "Beliefs die, and death has causes" | The **Graveyard** is a first-class tab; dead beliefs show **killed by** with the killing evidence |
| "It shows receipts before it acts" | The Inbox badge + effect-list-first review layout is visible in the shell chrome |

### Anti-goals (explicit)

- No anthropomorphizing: the system never says "I think" or "I remember"; it says "the record
  holds," "this belief survives," "3 resolutions." Copy is written in the register of DEMO.md:
  *positions that can lose · earned, never asserted · killed by · dies if*.
- No fake completeness: features the backend lacks (claims, DecisionMemos, privacy-tier
  routing, export) do not appear as disabled stubs. Absent means absent.
- No engagement mechanics: no streaks, no celebratory confetti. A settled premortem gets a
  precise 400 ms retirement animation, not applause.
- No timeless-trend violations: no gradients-as-identity, no glass blur, no neon "AI" cyan
  glow. The palette would look correct printed in a 1990s journal and on a 2036 display.

## 4. Who it is for (v1)

The niche is the backend's niche: **developers and technically-literate professionals** who
already distrust AI memory and want receipts. They read monospace comfortably, expect ⌘K,
and will test the system adversarially ("show me a belief you lost"). Design for their
skepticism first; approachability for broader users arrives via onboarding
(`05-ONBOARDING.md`), not via dilution of the interface.

---

*Next: `02-DESIGN-SYSTEM.md` — the Instrument design language as tokens and components.*
