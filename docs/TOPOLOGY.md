# Topology

This document describes how hierarchy, locality, permutation rank, and arboritions shape dissemination, witness selection, and repair.

For membership behavior, see [`MEMBERSHIP.md`](MEMBERSHIP.md). For dissemination behavior, see [`DISSEMINATION.md`](DISSEMINATION.md). For merge and repair semantics, see [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md). For dedicated treatments of the two distinctive primitives, see [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) and [`ARBORITIONS.md`](ARBORITIONS.md).

## Hierarchy Is Normal

Flat fanout is rarely the real deployment shape.

Real environments are structured by:

- racks
- subnets
- zones
- regions
- service shards
- trust domains

Topology-aware membership should treat these structures as first-class constraints on dissemination, witness selection, and repair.

## Scoped Hierarchy

Hierarchy matters for at least three reasons:

- **cost:** local dissemination is cheaper than cross-region spread
- **credibility:** a nearby witness may know more about local reachability while a higher-level witness may carry stronger authority
- **repair:** partition healing often needs scoped summaries before global reconciliation

The protocol should therefore allow beliefs to be locally strong and globally tentative.

## Permutation Rank In Topology

Permutation rank is a seeded deterministic ordering over a candidate peer set.

Useful seed material may include:

- subject identity
- current epoch
- cluster or scope identifier
- repair round identifier

Permutation rank should support:

- accountable fanout
- rendezvous selection
- deterministic witness sets
- reproducible tie-breaking

It is a discipline against anonymous selection. See [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) for the full protocol rationale, tradeoffs, and attack surfaces.

## Arboritions In Topology

An **arborition** is an adaptive topology-aware dissemination, witness, and repair forest.

The term emphasizes four things:

- the structure may be a forest rather than one tree
- the overlay adapts to locality and trust
- the same structure may serve dissemination, witness, and healing
- the topology is protocol-relevant, not just an optimization layer

An arborition may be re-shaped by:

- partition detection
- trust degradation
- scope changes
- hot spots or overloaded rendezvous sets
- operator intervention

See [`ARBORITIONS.md`](ARBORITIONS.md) for the full treatment of why forests beat one flat fanout graph as a mental model.

## Dissemination Overlays

Dissemination need not use the same overlay as repair.

A plausible design may separate:

- low-latency local witness paths
- higher-trust upward aggregation paths
- cross-partition repair paths

The point is not to maximize graph elegance. It is to make the overlay match the semantics of the claim being carried.

## Witness Placement

Witness selection should respect both topology and trust.

Bad witness placement patterns include:

- selecting all witnesses from one failure domain
- selecting only high-latency remote witnesses for a local claim
- selecting peers whose trust history is correlated in exactly the wrong way

Topology is therefore a trust amplifier or trust hazard, depending on how it is used.

## Operator Questions

Topology-aware membership should let an operator answer:

- which scopes saw this claim first?
- which arborition carried the dissemination?
- which rendezvous peers were selected, and why?
- where is healing bottlenecked?
- which trust boundaries constrained propagation?

Those answers are part of the protocol design, not merely dashboard extras.
