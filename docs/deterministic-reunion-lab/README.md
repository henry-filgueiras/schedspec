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

Difference from Quorum Lab:

- [`../quorum-lab/README.md`](../quorum-lab/README.md) asks when hidden capability may become observable through plural contact.
- Deterministic Reunion Lab asks what a system should do after it has already learned its observers can diverge.

Non-claim:

This is not a production reconciliation engine, not a network simulator, and not evidence that the repo contains a finished runtime.
