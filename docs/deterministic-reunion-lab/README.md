# Deterministic Reunion Lab

Deterministic Reunion Lab is a small interactive browser artifact for one claim:

> Partition healing is not connectivity restored, therefore truth restored. Healing is a negotiated merge of partially divergent realities, with residue preserved rather than flattened away.

This artifact is intentionally narrow:

- static and local-only
- publishable via GitHub Pages
- conceptual rather than runtime-shaped
- focused on divergence, reunion, residue, `RepairDigest`, and constrained `OperatorOverride`

Files:

- [`lab.html`](lab.html): interactive browser artifact
- [`lab.js`](lab.js): deterministic reunion logic and local fallback loader
- [`fallback-scenarios.js`](fallback-scenarios.js): embedded fallback copy of the scenario corpus for direct local opening

Scenario corpus:

- [`../scenarios/index.json`](../scenarios/index.json): scenario manifest
- [`../scenarios/deterministic-reunion-clean.json`](../scenarios/deterministic-reunion-clean.json)
- [`../scenarios/deterministic-reunion-stale-witness.json`](../scenarios/deterministic-reunion-stale-witness.json)
- [`../scenarios/deterministic-reunion-conflicting-acceptance.json`](../scenarios/deterministic-reunion-conflicting-acceptance.json)
- [`../scenarios/deterministic-reunion-trust-laundering.json`](../scenarios/deterministic-reunion-trust-laundering.json)
- [`../scenarios/deterministic-reunion-operator-override.json`](../scenarios/deterministic-reunion-operator-override.json)
- [`../scenarios/deterministic-reunion-epoch-race.json`](../scenarios/deterministic-reunion-epoch-race.json)

Tiny scenario format:

- `subjects`: the named membership subjects visible in both islands
- `initial`: island-local views before divergence events replay
- `events`: ordered patches applied while the islands are partitioned
- `expected_merge_tensions`: the semantic pressure points the reunion should surface
- `operator_override`: optional constrained human intervention for scenarios that should not auto-resolve honestly

Use:

1. Open [`lab.html`](lab.html) in a browser.
2. Step the divergence timeline while the islands remain partitioned.
3. Reconnect the islands.
4. Run deterministic reunion and inspect the merged view, residue, and `RepairDigest`.
5. Optionally compare naive reunion or apply the constrained override in the override scenario.

Rust conformance twin:

The same scenario corpus doubles as the conformance suite for the sans-IO Rust reference kernel (`crates/resonant-kernel`). Its typed merge engine must reproduce this lab's outcomes on every scenario, at every replay prefix, with and without the override. The same walkthrough from a terminal:

```
cargo run -p resonant-cli -- scenario list
cargo run -p resonant-cli -- scenario run deterministic-reunion-trust-laundering --naive
cargo run -p resonant-cli -- scenario run deterministic-reunion-operator-override --steps 1
cargo run -p resonant-cli -- scenario run deterministic-reunion-operator-override --override
cargo run -p resonant-cli -- scenario verify
```

`--steps N` replays a prefix of the divergence timeline (the CLI analogue of stepping before reconnecting), `--naive` prints the "latest or loudest wins" comparison, and `scenario verify` checks the whole corpus against the golden outcome table plus kernel-vs-lab agreement. Editing a scenario JSON changes both artifacts at once; the Rust suite fails loudly if the two renderings drift.

Difference from Quorum Lab:

- [`../quorum-lab/README.md`](../quorum-lab/README.md) asks when hidden capability may become observable through plural contact.
- Deterministic Reunion Lab asks what a system should do after it has already learned its observers can diverge.

Non-claim:

This is not a production reconciliation engine, not a network simulator, and not evidence that the repo contains a finished runtime. The Rust kernel it conforms with is a semantic reference, not a network system.
