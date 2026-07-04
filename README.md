# NEXUS

**A personal epistemic institution: it remembers for you, works for you, and keeps score on
itself — earning trust instead of asking for it.**

NEXUS manages knowledge, beliefs, decisions, evidence, and reasoning in a record you own. An
AI assistant is its first application — and its evidence pump — not its identity (D-018).

## The system in four verbs (D-019)

Everything identity-level in NEXUS implements exactly one of these; anything that can't is
either infrastructure (invisible plumbing) or doesn't ship:

| Verb | Meaning | Machinery | CLI today |
|---|---|---|---|
| **REMEMBER** | One private, permanent, encrypted record of what you saw, said, and did — provably forgettable, exportable, yours | append-only log, per-item crypto, keyring, derived index, retrieval | `append` `recall` `log` `forget` `rebuild` `verify` |
| **BELIEVE** | Only losable beliefs: every position must say what would prove it wrong, and cites its evidence | bets + admission rule, consolidation, justification graph, RECONCILE | `bets` `bet place` `consolidate` |
| **ACT** | Nothing real changes without a truthful preview you approve; authority is granted narrowly and expires | capability kernel, copy-on-write transactions, effect classes, inbox | `do` `inbox` `approve` `reject` `txn` |
| **SCORE** | Being right or wrong changes what gets trusted — beliefs, memories, workflows, and the AI models themselves | resolutions, calibration, scorecards, signed ledger | `bet resolve` `scorecard` `ledger --verify` |

## The same system, four vocabularies

**For a software engineer.** NEXUS is a local-first personal system built on an encrypted
append-only event log; all derived state — including beliefs, which are falsifiable "bets"
whose confidence is computed from resolution history and retracted through a justification
graph — is rebuildable from that log. Nothing touches the world except through an
object-capability kernel that stages effects in copy-on-write transactions, so every action is
a reviewable diff that commits, aborts, or queues at a gate by consequence class. LLMs are
stateless, swappable backends behind one router interface; every judgment records a manifest
of the exact context it saw, and memories, workflows, and models all carry track records that
determine their future influence and autonomy.

**For a product manager.** People won't give AI real authority because they can't verify its
work or its memory; NEXUS's answer is that every action ships as a reviewable preview before it
lands, and every memory carries a visible score of how often it's been right. The product is a
private record that compounds — it captures what the user saw, believed, decided, and rejected,
storing only beliefs that can be proven wrong, and it measurably improves with use: fewer
reviews needed, more earned autonomy. The moat is the record, not the model: users own their
data outright and can swap the AI vendor anytime, so retention comes from accumulated trust
that can't be copied, not from lock-in.

**For a non-technical person.** NEXUS is a private notebook-and-helper that lives on your
computer: it remembers what you tell it, what it believes about your life, and everything it
ever does for you — and all of it belongs to you, not to a company. It's only allowed to
believe things it could be proven wrong about, and it keeps score on itself, so you can always
ask "why do you think that, and how often have you been right?" When it works for you, it shows
you a preview first and you say yes or no — every yes it earns lets it help a little more on
its own, and the AI "brain" inside can be swapped out anytime without losing your notebook.

**For a 15-year-old.** NEXUS is an AI helper that keeps receipts on itself — everything it
remembers or does gets written in a private log only you can read. Its beliefs work like bets:
each one has to say what would prove it wrong, and being right or wrong changes its score —
high-score stuff gets trusted, wrong stuff gets benched. It can't touch your real files or
messages without showing you a preview you approve or reject; approvals level up its trust like
a game rank, and the AI model inside is just a swappable part, like changing a graphics card
without losing your saves.

### Why these are the same system

| Invariant | Engineer | PM | Non-technical | 15-year-old |
|---|---|---|---|---|
| One private permanent record | append-only encrypted log | user-owned compounding record | notebook that belongs to you | receipts in a private log |
| Beliefs must be losable | falsifiable bets, justification graph | stores only disprovable beliefs | only believes what could be proven wrong | beliefs are bets that can lose |
| Action = preview + approval | CoW transactions under capability law | reviewable preview before it lands | shows a preview, you say yes/no | approve/reject before it touches anything |
| Trust is earned by score | track records gate influence & autonomy | measurable improvement, earned autonomy | keeps score on itself | rank up or get benched |
| The AI brain is replaceable | swappable LLM backends + replay manifests | swap vendors, zero lock-in | brain swaps, notebook stays | graphics-card swap, saves stay |

## The five-minute proof

Skeptical? Run it:

```
bash demo/prove-it.sh
```

Nineteen self-asserting checks on a fresh record, with a deterministic mock standing in for
the AI so nothing can be attributed to model magic: unfalsifiable beliefs refused at write,
scores earned from events, a belief killed by evidence with the cascade flagging everything
built on it, `nx bets --lost` answering *"what did you believe that died, and what killed
it,"* a rejection changing the next task's plan (with in-scope/out-of-scope controls),
**automated settlement** (an approval mechanically retires the premortem whose falsifier it
meets — and the settled caution provably leaves future working sets), consolidated beliefs
citing their evidence episodes, the horizon sweeper ledgering stale beliefs, and 4 flipped
bytes anywhere in the log causing hard failure. Exits nonzero if any claim doesn't hold, and
writes [DEMO.md](DEMO.md) — an annotated transcript of every step.

## Quick start

```
nx init                 # create your record (prompts for passphrase + model provider)
nx do "task" --target . # delegate work; nothing applies until you approve
nx inbox                # review pending effect lists
nx consolidate          # graduate recent notes into scored beliefs
nx bets / nx scorecard  # what the system believes / how its reasoners perform
```

Requires Rust (`cargo build`); providers: Anthropic, OpenAI, Gemini, OpenRouter, Ollama,
LM Studio, or any OpenAI-compatible endpoint. Keys are sealed under your passphrase — never
stored in plaintext, never hardcoded.

## Documents

| File | Role |
|---|---|
| [COGNITIVE_ARCHITECTURE.md](COGNITIVE_ARCHITECTURE.md) | The theory: why this design (V2 intellectual foundation) |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Normative architecture — wins on conflict |
| [DECISIONS.md](DECISIONS.md) | ADR log D-001…D-019 |
| [ROADMAP.md](ROADMAP.md) | Phases, gates, risk register (v2.0) |
| [RESEARCH_NOTES.md](RESEARCH_NOTES.md) | Prior art, rejected ideas, open problems, experiments |
| [DESIGN.md](DESIGN.md) | Original V1 vision manifesto (non-normative) |
| [specs/](specs/) | The four sacred interfaces (record, capability, blackboard, epistemics) |
| [PROGRESS.md](PROGRESS.md) | Session-by-session build log |

---

STATUS: COMPLETE
