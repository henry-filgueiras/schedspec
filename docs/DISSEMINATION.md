# Dissemination

This document describes how claims should spread under partial observability. The subject is not raw message fanout. It is scoped fanout with provenance, trust weighting, and topology awareness.

For trust behavior, see [`TRUST.md`](TRUST.md). For topology-aware structures, see [`TOPOLOGY.md`](TOPOLOGY.md) and [`ARBORITIONS.md`](ARBORITIONS.md). For accountable ordering of relay and witness choices, see [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md).

## Scoped Fanout

Claims should spread according to scope, credibility, and urgency.

Questions dissemination must answer:

- who needs this claim now?
- who is allowed to witness it?
- who should hear only a digest, not the full claim set?
- when should propagation stop at a scope boundary?

Flooding is sometimes acceptable. It is rarely the right default.

## Claim Forms

A dissemination path may carry:

- full claim plus provenance
- witness digest
- residue summary
- healing request
- quarantine notice

Different forms imply different blast radii and different trust assumptions.

## Parent-Proxy Pools

A scope often needs bounded upward communication rather than full mesh contact. A **parent-proxy pool** is a selected set of higher-level peers used to:

- aggregate witness summaries
- relay scoped claims upward
- carry repair traffic across hierarchy boundaries
- provide bounded external visibility for a local scope

Parent-proxy pools are not authorities by default. They are structured relay and witnessing surfaces.

## Bounded Influence

Trust should influence blast radius, not only acceptance.

Low-confidence claims may still travel, but perhaps only:

- within the local scope
- to a restricted witness set
- as a digest rather than a strong assertion
- with hysteresis before further fanout

This keeps weak evidence from becoming global noise.

## Anti-Entropy

Steady-state dissemination and anti-entropy are not the same thing.

Steady-state dissemination spreads fresh claims.
Anti-entropy repairs drift, omission, and summary mismatch.

A serious design should keep both paths explicit.

## Operator Visibility

Operators should be able to ask:

- why did this claim propagate to this scope?
- which parent-proxy pool carried it upward?
- why was fanout bounded here?
- where is anti-entropy still outstanding?

Dissemination that cannot explain its own path is only rumor with metrics.
