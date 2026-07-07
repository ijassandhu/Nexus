# NEXUS Desktop — Information Architecture & Screen Specifications

**Version 1.0 · July 2026**

Every screen maps to implemented backend machinery. §1 is the binding contract; a screen with
no row in that table does not ship.

---

## 1. Concept → surface map (the anti-vaporware contract)

| Backend concept (implemented) | Source | Surface |
|---|---|---|
| Bets: schema, admission, statuses, fold | `substrate/bets.rs` | **Beliefs** screen + Belief Inspector |
| RECONCILE cascade, premises graph | `bets.rs::resolve` | **Graph** screen + cascade preview |
| Contradiction surfacing (lexical) | `bets.rs::contradictions` | Beliefs → Contradictions view |
| Horizon sweep, read-time expiry | `bets.rs::sweep_horizons` | Horizon strip (Overview + Beliefs) |
| Dead bets + killing note | `nx bets --lost` fold | **Graveyard** tab |
| Episodic log, forget, verify, rebuild, query | `substrate/{log,lib,derived}.rs` | **Record** screen + **Integrity** screen |
| Recall (ranked episodes) | `retrieval.rs::recall_episodes` | Record → Recall mode; palette search |
| Consolidation (APPRAISE) + report | `consolidate.rs` | **Consolidation review** flow (Beliefs screen) |
| Pipeline: Intent→Plan→Diff→Critique, working sets, derivations | `pipeline.rs`, `blackboard.rs` | **Tasks** screen + "What the model saw" |
| Inbox approve/reject, premortem settlement, Incidents | `inbox.rs` | **Inbox** screen |
| Transactions: begin/diff/commit/abort, drift | `kernel/txn.rs` | Inbox detail + Tasks detail |
| Capabilities, effect classes, ceilings | `kernel/capability.rs` | Task detail (capability card) |
| Signed effects ledger + verify | `kernel/ledger.rs` | **Integrity** → Ledger |
| Router, providers, sealed keys, fallbacks | `router.rs`, `providers.rs`, `onboarding.rs` | **Reasoners** screen |
| Reasoner scorecard + verdict attribution | `scorecard.rs` | **Reasoners** → Scorecard |

**Deliberately removed** (requested in the brief but absent from the backend — designing them
would violate "everything maps to a real concept"): *Goals*, *Decision Journal* (no backend
entity; the nearest real thing — Incidents + premortems — lives in Tasks/Beliefs), *standalone
Evidence Explorer* (evidence = events; the Record screen with provenance filters is that),
*privacy-tier routing UI* (privacy is recorded but not yet enforced by the router — shown as
metadata only, never as a control that pretends to gate). These return if/when the backend
grows them.

## 2. The shell

```
┌──┬────────────────────────────────────────────────────────┬──────────────┐
│  │  ⌘K Search or command…                    [record name] │              │
│ R├────────────────────────────────────────────────────────┤  INSPECTOR   │
│ A│                                                        │  (contextual │
│ I│                 PRIMARY REGION                         │   dockable,  │
│ L│                                                        │   ⌘\)        │
│  │                                                        │              │
├──┴────────────────────────────────────────────────────────┴──────────────┤
│ ● verified 12,408 events · addr 8f3a…c21d │ anthropic/claude-sonnet-5 │ ⬒ 3 │
└───────────────────────────────────────────────────────────────────────────┘
```

- **Rail** (left, 56 px collapsed / 200 px expanded): Overview · Beliefs · Inbox (badge =
  pending count) · Tasks · Record · Graph · Reasoners · Integrity · Settings (bottom).
  ⌘1–⌘8. The rail order *is* the product's value ranking.
- **Inspector** (right, 320–480 px, dockable/closable): renders the selected object —
  belief, event, task, txn — and always answers the six trust questions. One inspector,
  polymorphic; identical anatomy on every screen (this is how the UI teaches itself).
- **Status bar** = the standing integrity strip: live verify state + event count + latest
  content address (from `verify`), active route (provider/model), inbox count. The record's
  health is *permanently* visible — this is the single most identity-defining chrome decision.
- **Ask-the-record drawer** (⌘J): bottom sheet attached to the current object; chat as
  drill-down debugger per ARCHITECTURE.md §11. Never a nav item, never a badge, never opens
  by default.

## 3. Screens

### 3.1 Overview (Home) — "the state of what is believed"

Not a dashboard of charts; a briefing of positions. Four fixed zones:

```
┌ STANDING ─────────────────────────┐ ┌ AT RISK ───────────────────────┐
│ 47 live beliefs   ◆12 ●8 ◐27      │ │ ◔ 3 approaching horizon        │
│ held ×132 · falsified ×9          │ │ ◌ 2 unjustified (RECONCILE)    │
│ [FoldTrack: record-wide fold]     │ │ ⚡ 1 contradiction open         │
└───────────────────────────────────┘ └────────────────────────────────┘
┌ AWAITING YOU ─────────────────────┐ ┌ RECENT RESOLUTIONS ────────────┐
│ ⬒ 3 effect lists in inbox         │ │ ✕ "staging db safe to wipe"    │
│   "add logging config" · 1 effect │ │    killed by July wipe · 2h    │
│   critic: approve                 │ │ ✓ "prefers small PRs" held ×5  │
└───────────────────────────────────┘ └────────────────────────────────┘
```

Data: `views()` aggregates, `sweep`-pending horizons, `contradictions()`, `inbox::list`,
recent `memory.resolution/1` events. Every number is a link into its filtered screen.
No greeting, no "insights," no generated prose.

### 3.2 Beliefs — the core screen

Anatomy: filter bar → virtualized belief table → Inspector. Tabs: **Live · Graveyard ·
Contradictions · Candidates** (candidates = consolidation review, appears only when a
`consolidate` run has produced placements to review).

Table row (dense):
```
● live  belief ▁▃   "deploys should happen before noon"       ✓✓✓✓ 0.71  ◐  gen  12d
◌ unjs  belief ▁▃▅  "release friday is safe"                  ✓ 0.50→dim ◆  api  3d
```
Columns: StatusGlyph · kind · StakesMeter · **statement** (`type-body-strong`) · FoldTrack
(micro) · TrustGlyph · scope · age. Filters: status, kind, stakes, scope, unjustified,
has-contradiction, origin adapter. Sorts include *calibration* and *nearest horizon*.

**Belief Inspector** — the most designed object in the product; fixed anatomy top-to-bottom:

1. Statement (`type-heading`) + StatusGlyph + kind/stakes/scope chips
2. **Dies if** — every falsifier, verbatim, mono
3. **Confidence** — full FoldTrack: notch per resolution with note + `by` on hover;
   caption: "earned, never asserted"
4. **Evidence** — provenance EvidenceRefs (TrustGlyph + excerpt); "trust inherited:
   min(evidence) = ◐ derived"
5. **Structure** — premises (with their statuses) and dependents; cascade preview:
   *"if this falsifies, 4 dependents become unjustified"* (`ep-unjustified` count)
6. **History** — vertical timeline: placed → resolutions → (terminal + killed-by)
7. Actions: Resolve (held/falsified/expired/retracted + mandatory note) · Open in Graph ·
   Ask the record (⌘J)

**Graveyard** tab: terminal bets, sorted by death date. Card layout (not table): statement
struck-through in `text-secondary`, **killed by:** note + `by` in `ep-falsified`, the
falsifier that fired highlighted in its Dies-if list. Empty state: *"Nothing has died yet.
That either means the record is young, or nothing risky is believed."*

**Contradictions** tab: ContradictionPairs from `contradictions()`. Copy is honest about
mechanism: "surfaced by negation analysis — NEXUS never auto-resolves a contradiction."

**Candidates** (consolidation review): after `consolidate`, the `Report` renders as a review
list — placed bets (with provenance chips), duplicates (dimmed, with the live bet they
duplicate), rejected candidates (with the admission-rule reason, verbatim). This is where the
user watches *notes become beliefs through a rule*, the product's core loop.

**Place belief** (⌘N here, or palette): a form that enacts the admission rule — the Place
button stays disabled until ≥1 falsifier is non-empty, with the rule quoted beneath the
falsifier field. Attempting to submit whitespace shows the backend refusal verbatim.
Premises picker (existing bets), stakes meter, horizon-days, scope. Contradiction warning
surfaces pre-submit if negation analysis matches an existing live bet.

### 3.3 Inbox — approve effects, not summaries

List (left): one card per pending Diff — intent text, effect count, critic VerdictBadge, age.
Detail (right, replaces Inspector):

```
Intent  "add a config file for logging"            txn 01KW…2N6E
────────────────────────────────────────────────────────────────
EFFECT LIST (kernel truth)                          drift: none
  A  logging.toml            → 9f2c…            
  M  src/main.rs      4b1a… → 77e0…
────────────────────────────────────────────────────────────────
▸ advisory · model-written — "Adds structured logging config…"
CRITIC — approve (0 issues)
CAUTIONS HEEDED (2)
  ⚠ tasks like "wipe staging" fail: destroyed prod-mirror data
    settles if: a similar task is later approved without edits
────────────────────────────────────────────────────────────────
        [ Reject… r ]                    [ Approve a ]
```

- Effect list first, mono, hashes visible — the advisory summary is collapsed under it (P9).
- Critic issues (revise/veto) render expanded with their `target` refs.
- **Cautions heeded** shows `cited_premortems` with each one's settling falsifier — priming
  the settlement moment.
- **Approve** → commit; settled premortems animate retirement (see `04-…` §3.4); toast:
  "2 effects applied and ledgered · 1 caution settled."
- **Reject** → reason field focused, mandatory, placeholder: *"why? — this becomes memory"*.
  On submit, the UI shows the consequence chain as it happens: txn aborted → Incident filed →
  new premortem placed (card slides toward Beliefs). Rejection visibly *creates* knowledge.
- Drift non-empty: amber banner "target changed since snapshot — commit will refuse," approve
  disabled, drift lines listed.

### 3.4 Tasks — the pipeline made inspectable

List of Intents (state chips: open/planned/review/done/cancelled). Detail = the artifact
trail as a horizontal lifecycle:

```
Intent ──▶ Plan ──▶ Diff ──▶ Critique ──▶ done
 user      worker    proposed   approve      
 ◆         ◐ claude-sonnet-5
```

Each stage opens in the Inspector: Plan steps; Diff → its txn and effect list; Critique →
verdict + issues. Two sub-panels:

- **What the model saw** — from the `memory.derivation/1` + `memory.workingset/1` events:
  operator, reasoner (model id), charter@version, manifest hash, and the decrypted working
  set (files snapshot list, in-scope beliefs, cautions, recalls). This is the replayable
  context manifest as UX — no other AI product can show this, because no other product
  records it.
- **Capability** — the txn's minted capability: adapter, ops, scope, StakesMeter for
  max_effect, expiry, ceiling explanation ("context contains ◇ external → ceiling R1").

Incidents (from rejections) appear here attached to their Intent, with failure_class and the
premortem they minted (RefChip to Beliefs).

### 3.5 Record — the episodic log

The ground truth, virtualized (millions of rows). Modes: **Log** (chronological EventRows)
· **Recall** (query box → ranked excerpts with scores, exactly `recall_episodes`) · **Query**
(structured: schema/trust/kind/references/limit — the derived-index power tool, replaces
`nx query`).

Event Inspector: full header (every field, mono), refs in/out as navigable chips, body
decrypt-on-view (explicit "decrypt" action — reading is deliberate), derived objects
("this episode is evidence for 3 beliefs" — reverse provenance via refs).

**Forget ceremony** (`elev-2`, the one heavy modal, deliberately): states the three
mechanical consequences before the confirm — *key destroyed (unrecoverable) · tombstone
appended (id + hash only) · N beliefs whose only evidence this is will be retracted, and
their dependents marked unjustified* — with the affected beliefs listed as chips. Confirm
phrase required for cascades > 0. Afterward the row renders as `— forgotten —` permanently;
the tombstone is enumerable (Integrity). "What have I forgotten" is answerable; "what was
it" is not.

### 3.6 Graph — the justification structure

Nodes = live + unjustified bets (dead nodes optional toggle, rendered as ghosts). Edges:
solid = premise→dependent; dashed red = contradiction pairs; dotted gray into small square
nodes = evidence episodes (expandable per belief). Node encodes status (glyph+color),
size = dependents count (load-bearing beliefs are literally bigger).

Interactions: select → Inspector; **press-and-hold on a node = cascade preview** (its
transitive dependents pulse `ep-unjustified` — "what dies if this dies?" answered without
commitment); arrow keys traverse edges (a11y). Layout is force-directed *once*, then
positions persist per record (P7) — users build spatial memory of their own knowledge.
No physics toys, no minimap ornamentation; a search-to-focus field instead.

### 3.7 Reasoners — the replaceable brain, scored

Three panels:
- **Scorecard** — table per (provider, model, operator): calls, ok-rate, avg ms, approved,
  rejected — with approval-rate micro-bars. Caption: "judgment quality is attributed from
  your approve/reject verdicts via derivation events."
- **Routes** — the router.json editor as UI: three task-class rows (worker.execute,
  critic.review, archivist.appraise), each primary + ordered fallbacks, max_tokens.
  Unconfigured state renders the backend's own refusal-with-guidance.
- **Providers** — the 9 providers; key status = "sealed ✓" (never the key itself; env-var
  override indicated when active); base_url for compat/local; suggested model prefill;
  mock provider labeled "deterministic offline stub — for demos and tests."

Swap-the-brain moment: changing a route shows "knowledge belongs to the record, not the
model — scorecards continue across the swap."

### 3.8 Integrity — the record proves itself

The trust-but-verify screen; everything here is a *ritual made visible*:
- **Verify** — runs log verification with a per-segment progress strip; result: N events ·
  checksums OK · torn frames · forgotten count · latest content address (large, mono,
  copyable). Failures render hard-red, verbatim.
- **Ledger** — txn.* / ledger.effect events with SigBadges; "verify signatures" re-checks
  Ed25519 over canonicalized bodies; any BAD = red banner, non-dismissable.
- **Tombstones** — enumerated forgets (id, ts, body_hash).
- **Rebuild** — drop + re-derive the index, with the escape-hatch explanation ("derived
  stores are disposable; the log is the truth").

### 3.9 Settings

Record (data dir, passphrase change = rewrap), appearance (theme, density), keyboard map
(editable), providers shortcut, about (spec versions). Nothing else — settings sprawl is a
design failure elsewhere, not a goal here.

### 3.10 Command palette (⌘K)

Two-mode single field: verbs ("place belief," "run task…," "consolidate," "verify record,"
"recall …") and nouns (fuzzy over belief statements, event ids, intents, txns — via the
derived index). Results grouped by type with the same glyph language. Every palette verb =
one CLI command; parity is a shipping requirement (the palette *is* `nx` with a face).

## 4. Canonical journeys (acceptance-tested)

1. **Skeptic's first minute:** open app → status bar shows verified count + address →
   Beliefs → Graveyard → reads a killed-by. *"Show me a belief you lost"* answered in ≤3
   clicks, zero typing.
2. **Delegation loop:** palette → "run task" → watch Tasks trail form → Inbox badge →
   review effect list → approve → settlement animation → Scorecard increments.
3. **Rejection becomes memory:** reject with reason → Incident + premortem visibly created →
   next similar task shows the caution in "What the model saw."
4. **Death of a premise:** Beliefs → resolve falsified → cascade animation → unjustified
   dependents flagged everywhere they appear.
5. **Notes become knowledge:** Record → append notes → "Consolidate" → Candidates review —
   placements with citations, rejections with the rule quoted.
6. **The forget test:** forget an evidence event → ceremony discloses cascade → belief shows
   retracted; tombstone enumerable; body gone.

---

*Next: `04-VISUALIZATIONS-AND-MOTION.md`.*
