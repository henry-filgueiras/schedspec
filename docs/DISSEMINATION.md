# Dissemination

This document describes how claims should spread under partial observability. The subject is not raw message fanout. It is scoped fanout with provenance, trust weighting, and topology awareness.

See also:

- [`TRUST.md`](TRUST.md) for trust behavior
- [`TOPOLOGY.md`](TOPOLOGY.md) and [`ARBORITIONS.md`](ARBORITIONS.md) for topology-aware structure
- [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) for accountable ordering of relay and witness choices

## What Problem This Section Solves

The problem is not how to send a message to many peers. The problem is how to spread belief without turning weak evidence into system-wide noise.

Dissemination therefore needs structure, scope, and trust sensitivity.

A flat gossip story is tempting because it sounds simple: pick peers, send rumors, repeat until the cluster settles. But once claims differ in credibility, scope, urgency, and provenance, that mental model becomes too coarse. What matters is not merely that information moves. What matters is where it moves, in what form, under whose authority, and with what right to widen its blast radius.

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

A system that treats all membership traffic as one undifferentiated message type gives up too much control over both cost and meaning.

## Parent-Proxy Pools

A scope often needs bounded upward communication rather than full mesh contact. A **parent-proxy pool** is a selected set of higher-level peers used to:

- aggregate witness summaries
- relay scoped claims upward
- carry repair traffic across hierarchy boundaries
- provide bounded external visibility for a local scope

Parent-proxy pools are not authorities by default. They are structured relay and witnessing surfaces.

## Anti-Entropy And Repair Traffic

Steady-state dissemination and anti-entropy are not the same thing.

Steady-state dissemination spreads fresh claims. Anti-entropy repairs drift, omission, and summary mismatch.

Repair may need different relay choices, different pacing, and different visibility than ordinary spread. A serious design should therefore keep both paths explicit instead of hiding repair behavior inside generic rumor traffic.

## Design Invariants

1. dissemination should preserve provenance
2. scope is part of propagation policy, not merely metadata
3. weak confidence should imply bounded blast radius
4. anti-entropy should be explicit rather than smuggled into ordinary spread

## Tradeoffs And Failure Modes

Important failure modes include:

- over-broadcasting weak claims
- starving a scope of witness visibility
- parent-proxy pools becoming hidden choke points
- repair traffic overwhelming steady-state fanout

Dissemination should therefore be observable enough that operators can tell whether a problem is due to policy, topology, or trust.

A protocol that only exposes message counts will have metrics without explanation.

## Operator Interpretation

Operators should be able to ask:

- why did this claim propagate to this scope?
- which parent-proxy pool carried it upward?
- why was fanout bounded here?
- where is anti-entropy still outstanding?

Dissemination that cannot explain its own path is only rumor with metrics.
