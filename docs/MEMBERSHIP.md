# Membership

This document describes the protocol-facing membership model for Resonant Membership. It focuses on bootstrap, witness, trust, scoped dissemination, and operator visibility under partial observability.

For shared terms, see [`PRIMITIVES.md`](PRIMITIVES.md) and [`GLOSSARY.md`](GLOSSARY.md). For trust behavior, see [`TRUST.md`](TRUST.md). For dissemination, see [`DISSEMINATION.md`](DISSEMINATION.md). For the two most distinctive protocol primitives, see [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) and [`ARBORITIONS.md`](ARBORITIONS.md). For reconciliation and partition repair, see [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md).

## Membership Lifecycle

A subject may move through states such as:

- unknown
- introduced
- locally witnessed
- provisionally accepted
- widely accepted
- suspected
- disputed
- quarantined
- removed

These states are not merely liveness labels. They represent stages of converging belief.

## Bootstrap

Bootstrap is the first trust decision.

The protocol must answer:

- who may introduce a subject?
- to which scope is the introduction initially relevant?
- what evidence accompanies introduction?
- which witnesses are expected to corroborate or challenge it?

A design that treats bootstrap as "just seed nodes" usually hides its most fragile assumptions.

## Introduction

An introduction should contain at least:

- subject identity
- introducer identity
- initial scope
- freshness or epoch context
- optional evidence or capability proof

An introduction is not equivalent to acceptance. It is an invitation to witness.

## Witness Pipeline

Claims should move through a witness pipeline:

1. a subject is introduced
2. candidate witnesses are selected
3. witnesses attach local observation, confidence, and trust weight
4. the scope decides whether the claim is tentative, accepted, disputed, or quarantined

This pipeline exists to prevent first contact from becoming irreversible truth.

## Trust

Trust is part of the membership protocol, not merely external configuration.

Trust may be influenced by:

- static identity or cryptographic authority
- operator policy
- prior witness quality
- freshness of observation
- scope-local knowledge
- equivocation history

Trust need not be globally uniform. A witness may be strong in one scope and weak in another.

## Scoped Dissemination

Not every claim should flood the entire system immediately.

Scoped dissemination answers:

- who needs this claim now?
- who is allowed to act on it?
- who is allowed to witness it?
- who should wait for stronger corroboration?

Useful scopes may include:

- rack
- availability zone
- region
- service shard
- trust domain

Scope is therefore a protocol field, not a documentation note.

## Permutation Rank

Permutation rank is one of the core mechanisms in this repo: seeded deterministic peer ordering for accountable fanout, rendezvous, tie-breaking, and auditability.

It should be usable for:

- choosing initial witnesses
- selecting rendezvous peers for repair
- determining tie-break order when claims arrive simultaneously
- making fanout choices reconstructable after the fact

The important property is not just determinism, but accountable determinism. Two operators should be able to explain why the same witness set was chosen from the same seed and candidate set. See [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) for the full treatment.

## Accountability

Membership under partial observability requires explanation surfaces, not just convergence.

An operator should be able to ask:

- who introduced this subject?
- why were those witnesses selected?
- why was the claim accepted in this scope but not another?
- what evidence caused suspicion or quarantine?

Protocol design should expose these answers structurally rather than requiring log archaeology.

## Non-Claims

This document does not claim a finished protocol or fixed wire format. It defines the intended conceptual contract for membership behavior under weak coordination.
