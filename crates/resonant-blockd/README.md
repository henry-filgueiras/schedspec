# resonant-blockd

A federated blocklist daemon — the second skin over `resonant-net`, proving
the standing layer isn't chat-shaped. Same kernel, same evidence-derived
corroboration, same deterministic reunion; only the meaning of a *subject*
changes:

- **servers** are peers who earn membership standing in a federation scope
  (vouched by the federation root, advancing through the same probation
  ladder as chat members);
- **block entries** are subjects introduced with
  `AssertedState::Compromised`. A proposal becomes an *accepted* block only
  when independently-vouched servers corroborate it — the evidence gate
  that stops chat sockpuppets stops blocklist brigading;
- cross-federation disagreements survive partitions as **visible disputes
  with residue** instead of winning by arrival order, and every entry
  answers `/why` from the hash-chained transcript.

## Quickstart

```
# founding server
cargo run -p resonant-blockd -- --federation fediblock --seed 11 \
    --listen /ip4/127.0.0.1/tcp/46661

# joining servers
cargo run -p resonant-blockd -- --federation fediblock --seed 12 \
    --root <ROOT_PEER_ID> --dial /ip4/127.0.0.1/tcp/46661
```

| Command | Meaning |
|---|---|
| `/block <handle> [reason]` | Propose a block entry (counts as your confirmation) |
| `/confirm <handle>` | Corroborate a proposed block |
| `/dispute <handle>` | Witness against it |
| `/list` | Block entries with verdicts (proposed / blocked / DISPUTED / retired) |
| `/why <handle>` | The entry's causal story from the transcript |
| `/retire <handle>` | Federation-root override retiring an entry (visible, scar-preserving) |
| `/status` `/peers` `/split` `/heal` | Shared machinery (see resonant-node) |

Integration test: `cargo test -p resonant-blockd` — a proposal propagates
but stays short of acceptance until a second, independently-vouched server
confirms; then every server converges on the same digest.
