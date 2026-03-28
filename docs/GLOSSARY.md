# Glossary

This is the compact vocabulary index. For operational detail and invariants, see [`PRIMITIVES.md`](PRIMITIVES.md). For the two most distinctive protocol primitives, see [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) and [`ARBORITIONS.md`](ARBORITIONS.md).

## Quick Index

The entries here are intentionally short. For semantics, invariants, and protocol consequences, follow the linked docs in [`PRIMITIVES.md`](PRIMITIVES.md), [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md), and [`ARBORITIONS.md`](ARBORITIONS.md).

- **membership view:** a node's structured belief about who belongs and with what confidence
- **claim:** a transmissible statement about membership state
- **observation:** local evidence backing or disputing a claim
- **witness:** an observer whose corroboration or dispute matters
- **scope:** the audience or jurisdiction in which a claim is relevant
- **epoch:** bounded time or generation context for ordering and freshness
- **digest:** compact summary of a membership view for anti-entropy or repair
- **confidence:** current belief strength after trust, freshness, and corroboration are considered
- **provenance:** introducer, witnesses, and path history attached to a claim
- **residue:** unresolved disagreement preserved as visible structure
- **merge:** reconciliation of competing views
- **quarantine:** bounded suspension of belief propagation or acceptance
- **hysteresis:** deliberate resistance to oscillation near thresholds
- **anti-entropy:** explicit reconciliation mechanism for repairing drift
- **scoped fanout:** propagation bounded by audience and policy
- **trust root:** source of introduction or authority treated as foundational in some scope
- **witness set:** selected peers whose corroboration is relevant for a claim
- **parent-proxy pool:** higher-level or delegated peers used to bridge scope boundaries
- **deterministic reunion:** accountable repair path chosen when separated scopes reconnect
- **permutation rank:** seeded deterministic ordering for fanout, rendezvous, witness-set selection, arbitration, and audit
- **arborition:** adaptive topology-aware dissemination, witness, and repair forest; also described as an adaptive overlay forest
