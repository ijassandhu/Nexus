#!/usr/bin/env bash
# ============================================================================
# NEXUS — the five-minute proof: MEMORY THAT CAN LOSE
#
# For the skeptical senior engineer. Read this first:
#
#   * The AI model is a DETERMINISTIC MOCK — deliberately. Nothing below can
#     be attributed to LLM magic. Every behavior is substrate mechanics you
#     can read in ~200 lines of Rust per module.
#   * Every claim is asserted by this script. Any failure exits nonzero.
#   * Everything runs on a fresh record in a temp dir. Rerun it yourself.
#
# The claim under test: NEXUS is the only memory whose entries are POSITIONS
# THAT CAN LOSE — falsifiable at admission, settled by events, cascading on
# retraction, shaping behavior, and citable per judgment.
# ============================================================================
set -u
cd "$(dirname "$0")/.."
export NX_PASSPHRASE="prove-it"

PASS=0; FAIL=0
ok()  { echo "   PASS  $1"; PASS=$((PASS+1)); }
bad() { echo "   FAIL  $1"; FAIL=$((FAIL+1)); }
step(){ echo; echo "━━ $1"; }

echo "building..."
cargo build --quiet 2>/dev/null || cargo build || exit 1
NX="./target/debug/nx"
WORK="$(mktemp -d)"; DATA="$WORK/.nexus"; PROJ="$WORK/proj"; mkdir -p "$PROJ"
"$NX" --data "$DATA" init --no-configure >/dev/null

# ────────────────────────────────────────────────────────────────────────────
step "1. ADMISSION: a belief that cannot lose is refused storage"
OUT=$("$NX" --data "$DATA" bet place "I am always right" --falsifier "   " 2>&1)
if echo "$OUT" | grep -q "no falsifiers, no bet"; then
  ok "unfalsifiable belief rejected at the only write path:"
  echo "         \"${OUT#*Error: }\"" | head -1
else bad "unfalsifiable belief was accepted: $OUT"; fi
[ "$("$NX" --data "$DATA" bets | grep -c "held:")" = "0" ] \
  && ok "record contains zero beliefs (nothing slipped through)" \
  || bad "something was stored anyway"

# ────────────────────────────────────────────────────────────────────────────
step "2. SETTLEMENT: confidence is a score, not an adjective"
A=$("$NX" --data "$DATA" bet place "the staging db is safe to wipe" \
      --falsifier "a wipe destroys data someone needed" --scope "staging-db")
B=$("$NX" --data "$DATA" bet place "cleanup jobs may run without review" \
      --falsifier "a cleanup job causes an incident" --scope "staging-db" --premise "$A")
"$NX" --data "$DATA" bet resolve "$A" held --note "wiped clean in June, no complaints" >/dev/null
"$NX" --data "$DATA" bets | grep -A1 "^$A" | grep -q "held:1" \
  && ok "belief A earned a score by surviving contact with events (held:1)" \
  || bad "score did not accrue"

# ────────────────────────────────────────────────────────────────────────────
step "3. THE LOSS: evidence kills A — and the cascade flags what A justified"
RES=$("$NX" --data "$DATA" bet resolve "$A" falsified \
      --note "July wipe destroyed the analytics team's prod-mirrored dataset")
echo "   $RES"
echo "$RES" | grep -q "1 dependent bets marked unjustified" \
  && ok "RECONCILE: belief B (built on A) auto-flagged — conclusions cannot outlive premises" \
  || bad "no cascade: $RES"
"$NX" --data "$DATA" bets | grep -A1 "^$B" | grep -q "UNJUSTIFIED" \
  && ok "B now carries [UNJUSTIFIED] — visible in every future retrieval" \
  || bad "B not flagged"

# ────────────────────────────────────────────────────────────────────────────
step "4. THE LITMUS TEST: 'show me a belief you lost, and what killed it'"
echo "   \$ nx bets --lost"
"$NX" --data "$DATA" bets --lost | sed 's/^/   /'
"$NX" --data "$DATA" bets --lost | grep -q "killed by: July wipe destroyed" \
  && ok "the question every assistant returns nothing for — answered from the ledger" \
  || bad "litmus test unanswered"

# ────────────────────────────────────────────────────────────────────────────
step "5. BEHAVIOR: a rejection becomes memory that changes the next judgment"
"$NX" --data "$DATA" do "add a config file" --target "$PROJ" --mock > "$WORK/t1.out"
grep -q "heeding" "$WORK/t1.out" \
  && bad "task 1 heeded a caution before any existed (contaminated)" \
  || ok "CONTROL: task 1 plan has no cautions — no memory exists yet"
T1=$("$NX" --data "$DATA" txn list | awk '$2=="Open"{print $1}' | head -1)
"$NX" --data "$DATA" reject "$T1" --reason "config lives in repo root, never generated" >/dev/null
# a second control: an out-of-scope premortem that must NOT leak in
"$NX" --data "$DATA" bet place "tasks in the other repo fail: wrong toolchain" \
      --falsifier "a task there succeeds" --kind premortem --scope "/some/unrelated/repo" >/dev/null
"$NX" --data "$DATA" do "add a config file for logging" --target "$PROJ" --mock > "$WORK/t2.out"
grep -q "heeding 1 caution" "$WORK/t2.out" \
  && ok "task 2 heeds EXACTLY the one in-scope caution (out-of-scope premortem stayed out)" \
  || bad "memory did not shape the plan correctly: $(grep plan "$WORK/t2.out")"
T2=$("$NX" --data "$DATA" txn list | awk '$2=="Open"{print $1}' | head -1)
"$NX" --data "$DATA" approve "$T2" >/dev/null
grep -q "config lives in repo root" "$PROJ/NOTES.md" \
  && ok "the applied artifact carries the caution — your rejection reason, working" \
  || bad "artifact does not reflect memory"

# ────────────────────────────────────────────────────────────────────────────
step "6. ATTRIBUTION: the judgment names the memory it relied on"
PRE=$("$NX" --data "$DATA" bets | grep -B0 "Premortem" | awk '/Premortem/{print $1}' | head -1)
"$NX" --data "$DATA" log --decrypt | grep '"operator":"deliberate"' | tail -1 | grep -q "$PRE" \
  && ok "derivation event cites premortem $PRE verbatim — falsify the memory, know which judgments to distrust" \
  || bad "derivation does not cite the relied-upon bet"

# ────────────────────────────────────────────────────────────────────────────
step "7. RECEIPTS: the record is tamper-evident and the ledger is signed"
"$NX" --data "$DATA" ledger --verify >/dev/null 2>&1 \
  && ok "every ledger entry Ed25519-verified" || bad "ledger verification failed"
SEG=$(ls "$DATA/log"/*.nxl | head -1); cp "$SEG" "$SEG.bak"
printf '\xff\xff\xff\xff' | dd of="$SEG" bs=1 seek=100 conv=notrunc 2>/dev/null
"$NX" --data "$DATA" verify >/dev/null 2>&1 \
  && bad "4 flipped bytes went undetected" \
  || ok "4 flipped bytes anywhere in the log → hard failure (BLAKE3 per frame)"
mv "$SEG.bak" "$SEG"
"$NX" --data "$DATA" verify >/dev/null 2>&1 \
  && ok "restored record verifies clean" || bad "restore failed"

# ────────────────────────────────────────────────────────────────────────────
echo
echo "════════════════════════════════════════════════════════════════════"
echo "  $PASS passed, $FAIL failed."
echo
echo "  What you just watched, with a scripted mock standing in for the AI:"
echo "  a memory that refuses unfalsifiable entries, earns scores from"
echo "  events, loses beliefs and flags everything built on them, changes"
echo "  its behavior because of a recorded failure, names the memories"
echo "  behind every judgment, and can prove its own integrity."
echo
echo "  Ask any AI assistant: \"show me a belief you lost last month,"
echo "  and what killed it.\" Then run: nx bets --lost"
echo "════════════════════════════════════════════════════════════════════"
[ "$FAIL" -eq 0 ] || exit 1
