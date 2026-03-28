# Manifesto

Membership is not a list. It is a negotiated, scoped, decaying belief state.

Strong coordination is expensive. Perfect knowledge is usually unavailable.

Systems must still decide who exists, who belongs, what changed, which claims deserve belief, whether a partition has healed, and how to merge competing realities safely.

Gossip is therefore not merely message dissemination. It is an epistemic control plane for systems that cannot afford certainty.

## What Problem This Section Solves

This document serves as the dangerous preface to the rest of the repo.

Its job is not to specify semantics in detail, define metrics, or carry the full chapter stack. Its job is to keep the project honest about what it thinks the real problem is, and to refuse the comforting simplifications that make many membership systems sound cleaner than they are.

## Dangerous Preface

The interesting failure mode is not that one node misses one heartbeat.

The interesting failure mode is that one scope accepts a claim another scope still disputes.

A weak witness receives more blast radius than it deserves. A partition heals but reality does not yet agree with itself. An operator sees convergence but cannot explain why the system believed this view instead of its competitor.

That is the point at which "membership" stops being a table-maintenance exercise and becomes a systems problem.

## Core Claims

This repo treats the hard parts as first-class:

- bootstrap
- witness and trust
- scoped dissemination
- deterministic merge
- partition healing
- topology and hierarchy
- operator observability

None of those are cleanup details. They are the actual protocol.

## Design Invariants

The manifesto is anchored by a few propositions that recur across the rest of the docs:

1. Partial observability is the normal case, not an edge case.
2. Dissemination without provenance is rumor.
3. Dissemination without trust weighting is noise amplification.
4. Restored connectivity is not convergence.
5. Healing is negotiated reality merge.
6. Operator visibility is part of correctness.

## Tradeoffs And Failure Modes

This design direction is intentionally less comforting than a generic gossip story.

It asks the system to preserve uncertainty instead of flattening it early.

It asks operators to inspect merge and witness quality rather than trusting summary tables. It asks the protocol to explain its fanout, reunion, and repair behavior in reconstructable terms.

That comes with costs:

- more protocol structure to carry and expose
- more disagreement retained as visible residue
- more responsibility to shape trust and scope carefully
- more need for disciplined hysteresis during repair

The alternative is simpler prose and more mysterious failures.

## Operator Interpretation

If the rest of the repo succeeds, an operator should be able to say:

- who believed what
- why they believed it
- how far that belief was allowed to spread
- what residue remained after healing

If those answers are unavailable, the system is still hiding its most important semantics.

## Anti-Goals

- not heartbeat folklore dressed up as protocol theory
- not event-driven mysticism about eventual consistency
- not fake Byzantine posturing without cost accounting
- not a centralized authority paper pretending to be gossip

## Non-Claims

This document is not the protocol contract and not a claim of implementation. It is the preface that keeps the protocol contract from drifting toward a weaker problem statement.

## Coda

If a system cannot say who believed what, why they believed it, how far that belief was allowed to spread, and what residue remained after healing, then it does not understand its own membership semantics well enough to deserve the name.
