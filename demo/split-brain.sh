#!/usr/bin/env bash
# Split-brain moderation demo: three resonant-chat nodes on localhost.
#
# alice (room creator) and bob partition away from carol, ban her during
# the split while her island keeps believing in itself, then heal. The
# deterministic reunion refuses to counterfeit certainty: carol is
# disputed with visible residue on every node, and alice's operator
# override quarantines her while marking the scar handled — never erased.
#
# Self-driving: feeds commands to all three nodes over fifos and narrates.
set -euo pipefail

cd "$(dirname "$0")/.."
cargo build -q -p resonant-node

BIN=target/debug/resonant-chat
DIR=$(mktemp -d)
PIDS=()
cleanup() {
  for pid in "${PIDS[@]:-}"; do kill "$pid" 2>/dev/null || true; done
  rm -rf "$DIR"
}
trap cleanup EXIT

step() { printf '\n\033[1m== %s ==\033[0m\n' "$*"; }
show() { tail -n "$2" "$DIR/$1.log" | sed "s/^/  [$1] /"; }

for n in alice bob carol; do mkfifo "$DIR/$n.in"; done

step "starting alice (room creator, seed 1)"
$BIN --room demo --nick alice --seed 1 --listen /ip4/127.0.0.1/tcp/45551 \
  < "$DIR/alice.in" > "$DIR/alice.log" 2>&1 &
PIDS+=($!)
exec 3> "$DIR/alice.in"   # held open so alice's stdin never hits EOF
sleep 2
ALICE=$(grep -m1 'peer id' "$DIR/alice.log" | awk '{print $NF}')
echo "  alice is $ALICE"

step "starting bob and carol, vouched by the creator"
$BIN --room demo --nick bob --seed 2 --creator "$ALICE" \
  --listen /ip4/127.0.0.1/tcp/45552 --dial /ip4/127.0.0.1/tcp/45551 \
  < "$DIR/bob.in" > "$DIR/bob.log" 2>&1 &
PIDS+=($!)
exec 4> "$DIR/bob.in"
$BIN --room demo --nick carol --seed 3 --creator "$ALICE" \
  --listen /ip4/127.0.0.1/tcp/45553 --dial /ip4/127.0.0.1/tcp/45551 --dial /ip4/127.0.0.1/tcp/45552 \
  < "$DIR/carol.in" > "$DIR/carol.log" 2>&1 &
PIDS+=($!)
exec 5> "$DIR/carol.in"
sleep 2
BOB=$(grep -m1 'peer id' "$DIR/bob.log" | awk '{print $NF}')
CAROL=$(grep -m1 'peer id' "$DIR/carol.log" | awk '{print $NF}')
echo "  bob is $BOB"
echo "  carol is $CAROL"

alice() { echo "$*" >&3; }
bob()   { echo "$*" >&4; }
carol() { echo "$*" >&5; }

step "standing accrues: witness records gossip, hysteresis passes"
sleep 8
alice /roster; sleep 1
show alice 4

step "partition: carol alone on one island"
alice "/split $CAROL"; bob "/split $CAROL"
carol "/split $ALICE"; carol "/split $BOB"
sleep 2

step "the majority island bans carol; her island still believes in her"
alice "/ban $CAROL posting spam during the split"
sleep 3
alice /roster; carol /roster; sleep 1
show alice 4
show carol 4

step "heal: unblock and redial — deterministic reunion negotiates"
alice /heal; bob /heal; carol /heal
sleep 6
alice /status; carol /status; sleep 1
show alice 3
show carol 3

step "every island now agrees carol is DISPUTED, with visible residue"
bob /roster; sleep 1
show bob 4

step "the creator resolves it visibly: override to quarantined"
alice "/override $CAROL quarantined"
sleep 4
alice /status; bob /status; carol /status; sleep 1
show alice 3
show bob 3
show carol 3

step "the ban's whole causal story, from bob's transcript"
bob "/why $CAROL"; sleep 1
show bob 12

step "done — full logs in $DIR (kept until you press enter)"
read -r || true
