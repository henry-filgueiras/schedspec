# Membership

This document describes the protocol-facing membership model for Resonant Membership. It focuses on bootstrap, witness, trust, scoped dissemination, and operator visibility under partial observability.

See also:

- [`PRIMITIVES.md`](PRIMITIVES.md) and [`GLOSSARY.md`](GLOSSARY.md) for shared terms
- [`SEMANTICS.md`](SEMANTICS.md) for the protocol-object and decision-surface contract
- [`MECHANICS.md`](MECHANICS.md) for introduction, witness-selection, and dissemination loops
- [`TRUST.md`](TRUST.md) for trust behavior
- [`DISSEMINATION.md`](DISSEMINATION.md) for dissemination behavior
- [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) and [`ARBORITIONS.md`](ARBORITIONS.md) for the two distinctive protocol primitives
- [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md) for reconciliation and partition repair
- [`DIAGRAMS.md`](DIAGRAMS.md) for the canonical lifecycle and trust visuals
- [`EXAMPLES.md`](EXAMPLES.md) for worked protocol scenarios

## What Problem This Section Solves

This section defines what it means to maintain membership when no participant can directly inspect global truth. The aim is to make "membership as belief state" operational rather than merely rhetorical.

This chapter is about behavior: introduction, witness formation, scoped belief, and visible lifecycle state. The object contract itself lives in [`SEMANTICS.md`](SEMANTICS.md).

A system that speaks only in terms of alive and dead hides too much.

It hides:

- who introduced a subject
- who corroborated it
- which scope currently accepts it
- what evidence remains disputed
- why some observers are trusted more than others

Resonant Membership treats those details as the substance of membership rather than as annotations on top of a liveness table.

## Behavioral Model

Membership is the evolving answer to a harder question than "is this node up?"

The real question is: what does this scope currently believe about this subject, on what evidence, with what confidence, and under what right to spread that belief further?

That formulation matters because different scopes can legitimately hold different views at the same moment.

A rack-local scope may treat a subject as strongly witnessed while a regional scope still treats it as provisional. A weakly trusted introduction may be useful locally while remaining unsuitable for broader dissemination. Partial observability makes such asymmetry normal rather than pathological.

The core semantic states are therefore not merely liveness labels. They are stages in the formation, spread, and repair of belief.

## Membership Lifecycle

A subject's scoped belief state moves through the canonical protocol states below:

- unknown
- introduced
- witnessed
- provisional
- accepted
- suspected
- disputed
- quarantined
- removed

These states are not merely liveness labels. They are stages in the formation, spread, and repair of belief.

For reading discipline across the repo:

- `witnessed` is the state reached once introduction has acquired protocol-visible witness records
- `provisional` is bounded scoped acceptance that has not yet earned wider confidence
- `accepted` is stronger scoped convergence; phrases like "widely accepted" describe this state rather than introducing a second accepted state

`revocation` is not a separate durable subject state. It is a visible transition event that withdraws prior acceptance or prior trust and may drive a subject or witness toward `suspected`, `disputed`, `quarantined`, or `removed` depending on scope and evidence.

## Bootstrap And Introduction

Bootstrap is the first trust decision.

The protocol must answer who may introduce a subject, to which scope the introduction is initially relevant, what evidence accompanies the introduction, and which witnesses are expected to corroborate or challenge it.

A design that treats bootstrap as "just seed nodes" usually hides its most fragile assumptions.

An introduction should contain at least:

- subject identity
- introducer identity
- initial scope
- freshness or epoch context
- optional evidence or capability proof

An introduction is not equivalent to acceptance. It is an invitation to witness.

## Witness Formation

Claims should move through a witness pipeline:

1. a subject is introduced
2. candidate witnesses are selected
3. witnesses attach local observation, confidence, and trust weight
4. the scope decides whether the resulting belief state is provisional, accepted, disputed, or quarantined

This pipeline exists to prevent first contact from becoming irreversible truth.

Witnessing is not there to decorate a claim after the fact. It is there to make the right to believe and the right to spread visible and contestable.

## Scoped Membership

Not every claim should become globally meaningful at once.

Membership should be able to be:

- strong in one scope
- provisional in another
- disputed in a third

That is not a bug. It is a realistic consequence of partial observability, uneven witness quality, and topology-shaped cost.

## Design Invariants

Key invariants for membership behavior:

1. bootstrap is not implicit trust
2. introduction is not acceptance
3. witness quality matters as much as witness count
4. scope is part of the claim's meaning
5. suspicion and quarantine should be visible states, not hidden timers

## Tradeoffs And Failure Modes

Important membership failure modes include:

- weak introductions spreading too early
- witness scarcity in a failure domain
- scope-local overconfidence
- oscillation between suspicion and acceptance
- silent disagreement hidden behind summary tables

A belief-oriented membership system is more honest about those risks, but it also exposes them more often.

The protocol should therefore make these failures inspectable instead of pretending they are noise.

## Operator Interpretation

Membership under partial observability requires explanation surfaces, not just convergence.

An operator should be able to ask:

- who introduced this subject?
- why were those witnesses selected?
- why was the claim accepted in this scope but not another?
- what evidence caused suspicion or quarantine?

Protocol design should expose these answers structurally rather than requiring log archaeology.

## Non-Claims

This document does not claim a finished protocol or fixed wire format. It defines the intended conceptual contract for membership behavior under weak coordination.
