# Resonant Membership

Resonant Membership is a design-first systems treatise on gossip, trust, and convergence under partial observability. The claim is not that cluster membership is just a faster heartbeat table; it is that once bootstrap, witness, scoped dissemination, trust, merge rules, partition healing, hierarchy, and operator visibility become first-class, membership stops being a side channel and becomes a protocol for coordinated belief under weak coordination.

This repository is organized around **Resonant Membership** as the active center of gravity. Earlier or adjacent threads, especially **ChronOS** and **SameDiff**, are preserved as archived design lineages rather than treated as co-equal active projects.

## Repository Architecture

This repo follows one explicit model:

- **Active project:** Resonant Membership, at the root and under [`docs/`](docs)
- **Archived adjacent lineage:** ChronOS / `chrono flow`, under [`notes/archive/chronos/`](notes/archive/chronos)
- **Archived adjacent lineage:** SameDiff / contrast-calculus notes, under [`notes/archive/SAMEDIFF.md`](notes/archive/SAMEDIFF.md)

This is intentional. The repo should read as one active treatise with preserved neighboring lineages, not as three half-active projects competing for the same front page.

## Project Map

If you are here for:

- **Resonant Membership:** start with [`docs/MANIFESTO.md`](docs/MANIFESTO.md), then [`docs/ABSTRACT.md`](docs/ABSTRACT.md), [`docs/PRIMITIVES.md`](docs/PRIMITIVES.md), and the protocol stack under [`docs/`](docs)
- **ChronOS:** start with [`notes/archive/chronos/README.md`](notes/archive/chronos/README.md), then [`notes/archive/chronos/VISION.md`](notes/archive/chronos/VISION.md) and [`notes/archive/chronos/SPEC.md`](notes/archive/chronos/SPEC.md)
- **SameDiff:** start with [`notes/archive/SAMEDIFF.md`](notes/archive/SAMEDIFF.md)

## Three Theses

- **Membership is a problem of converging belief, not merely detecting liveness.**
- **Partial observability is normal, so trust, witness, and repair must be explicit.**
- **Deterministic ordering and topology matter because operators need accountable dissemination, not anonymous rumor.**

## Why This Exists

Most gossip discussions flatten the hard parts:

- who gets to introduce a node
- who is allowed to witness or dispute a claim
- how trust is accumulated, scoped, or revoked
- how partitions heal without pretending the split never happened
- how hierarchy and locality shape dissemination cost
- how an operator explains why the system believed one view and not another

Resonant Membership starts from a harder setting:

- observability is partial
- witnesses disagree
- trust is uneven
- dissemination is scoped
- topology is heterogeneous
- partitions are expected
- convergence must still be auditable

## Key Ideas

- **Permutation rank:** seeded deterministic peer ordering for accountable fanout, rendezvous, tie-breaking, and auditability
- **Witness and trust pipeline:** claims should move through introduction, corroboration, weighting, and acceptance rather than becoming truth on first contact
- **Scoped dissemination:** not every claim should flood everywhere at once; scope and audience are protocol concerns
- **Merge and healing:** membership views must reconcile after drift, omission, and partition rather than assuming a single canonical observer
- **Hierarchy and arboritions:** adaptive topology-aware dissemination, witness, and repair trees or overlay forests should reflect locality and trust structure
- **Operator observability:** the system should answer why a view converged, which witnesses mattered, and where disagreement still lives

## Anti-Goals

- not a generic gossip tutorial
- not a marketing page for a finished implementation
- not a simplistic heartbeat-only membership protocol
- not a trustless model pretending witness quality does not matter
- not a centralized control-plane paper in disguise

## Example

Illustrative protocol sketch:

```text
seed = epoch || cluster_id || subject_id
rank = permutation_rank(seed, candidate_peers)

introducer -> witnesses[rank[0..k]]:
  propose subject S in scope edge.us-west

witnesses:
  record observation
  attach trust weight and freshness
  disseminate upward only if local corroboration threshold is met

merger:
  join signed observations
  prefer higher-confidence convergent view
  preserve unresolved disagreement as visible residue
```

## Docs

- [`docs/MANIFESTO.md`](docs/MANIFESTO.md): thesis lines and anti-goals
- [`docs/ABSTRACT.md`](docs/ABSTRACT.md): manifesto and framing
- [`docs/GLOSSARY.md`](docs/GLOSSARY.md): compact vocabulary index
- [`docs/PRIMITIVES.md`](docs/PRIMITIVES.md): glossary and protocol primitives
- [`docs/SEMANTICS.md`](docs/SEMANTICS.md): semantic contract for protocol objects and decision surfaces
- [`docs/PERMUTATION_RANK.md`](docs/PERMUTATION_RANK.md): deterministic ordering as a protocol primitive
- [`docs/ARBORITIONS.md`](docs/ARBORITIONS.md): adaptive overlay forests for dissemination, witness, and repair
- [`docs/MEMBERSHIP.md`](docs/MEMBERSHIP.md): bootstrap, witness, trust, scoped dissemination, operator visibility
- [`docs/DISSEMINATION.md`](docs/DISSEMINATION.md): scoped fanout, witness spread, parent-proxy pools
- [`docs/TRUST.md`](docs/TRUST.md): trust roots, witness weighting, confidence, blast radius
- [`docs/MERGE_AND_HEALING.md`](docs/MERGE_AND_HEALING.md): merge rules, reconciliation, partition healing
- [`docs/TOPOLOGY.md`](docs/TOPOLOGY.md): hierarchy, permutation rank, and arborition overlays
- [`docs/THREAT_MODEL.md`](docs/THREAT_MODEL.md): failures, adversaries, and abuse cases
- [`docs/EXAMPLES.md`](docs/EXAMPLES.md): worked scenarios
- [`docs/DIAGRAMS.md`](docs/DIAGRAMS.md): canonical Mermaid diagrams

## Status

This repository is a systems design document set. It describes intended primitives, invariants, and protocol shape; it should not be read as a claim that a complete runtime or implementation already exists.

## Start Here

For the active project, read [`docs/MANIFESTO.md`](docs/MANIFESTO.md) and [`docs/ABSTRACT.md`](docs/ABSTRACT.md) for the thesis.

Then read [`docs/GLOSSARY.md`](docs/GLOSSARY.md), [`docs/PRIMITIVES.md`](docs/PRIMITIVES.md), and [`docs/SEMANTICS.md`](docs/SEMANTICS.md) for vocabulary and semantic structure, followed by [`docs/MEMBERSHIP.md`](docs/MEMBERSHIP.md), [`docs/DISSEMINATION.md`](docs/DISSEMINATION.md), [`docs/TRUST.md`](docs/TRUST.md), [`docs/MERGE_AND_HEALING.md`](docs/MERGE_AND_HEALING.md), and [`docs/TOPOLOGY.md`](docs/TOPOLOGY.md).

Keep [`docs/THREAT_MODEL.md`](docs/THREAT_MODEL.md), [`docs/DIAGRAMS.md`](docs/DIAGRAMS.md), and [`docs/EXAMPLES.md`](docs/EXAMPLES.md) nearby while reading.
