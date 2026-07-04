# resonant-node

P2P group chat where **moderation standing survives partitions**. Two tiers:

- **below:** libp2p (gossipsub for chat/claims/digests, request-response for
  reunion negotiation, ping/identify for presence) — transport and liveness;
- **above:** the sans-IO `resonant-kernel` — membership *standing*: who is a
  member, moderator, muted, or banned, with evidence-derived witness
  summaries, honest deterministic merge after partitions, visible residue,
  and a hash-chained decision transcript.

The kernel never crosses an `.await`: every network or user stimulus becomes
a kernel `Input` (optionally logged as JSONL via `--input-log`, replayable
with `verify_replay`), every kernel `Effect` becomes a publish or request.

## Quickstart (three terminals)

```
# terminal 1 — create the room (prints your peer id)
cargo run -p resonant-node -- --room lobby --nick alice --seed 1 \
    --listen /ip4/127.0.0.1/tcp/45551

# terminals 2 and 3 — join, vouched by the creator
cargo run -p resonant-node -- --room lobby --nick bob --seed 2 \
    --creator <ALICE_PEER_ID> --dial /ip4/127.0.0.1/tcp/45551
cargo run -p resonant-node -- --room lobby --nick carol --seed 3 \
    --creator <ALICE_PEER_ID> --dial /ip4/127.0.0.1/tcp/45551
```

Type to chat. Commands:

| Command | Meaning |
|---|---|
| `/roster` | Everyone's standing + derived witness summary (count/quality/diversity) |
| `/status` | Digest hash, live residue, disputes |
| `/vouch <peer>` | Publish a strong (challenge-response) witness record |
| `/mute <peer>` / `/ban <peer>` | Moderation — quarantine / removal, honored from accepted members |
| `/override <peer> [state]` | Creator-only visible override; marks residue handled, never erases it |
| `/split <peer...>` | Simulate a partition (block-lists those peers, severing connections) |
| `/heal` | Unblock and redial; digest exchange triggers deterministic reunion |
| `/why <peer>` | The subject's causal story from the hash-chained transcript |

Peers can be addressed by unique peer-id suffix.

## The demo

`../../demo/split-brain.sh` self-drives the whole story: standing accrues →
partition → one island bans carol while hers keeps her accepted → heal →
deterministic reunion converges every node on **disputed + visible residue**
→ the creator's override quarantines her and takes responsibility for the
scar. The same flow runs as an integration test over real TCP transport:
`cargo test -p resonant-node`.

## What this demonstrates

- **Naive reunion would lie** ("latest or loudest wins"); this one preserves
  the conflict as first-class residue and converges every replica on the
  same honest view (digest content hashes match).
- **Sockpuppets can't buy standing**: witness summaries are derived from
  evidence — a loud single-lineage cluster of weak observations grades
  `Laundered` and the advancement gate refuses it.
- **Interventions are accountable**: overrides are visible, creator-only,
  and leave the scar marked handled; `/why` replays any decision from the
  tamper-evident transcript.

## Non-claims

A demo vehicle, not a product: no persistence, no NAT traversal config, no
end-to-end chat encryption beyond transport noise, trust grades are a fixed
demo policy. The kernel semantics it exercises are the point.
