# Trust

This document describes how witness quality, trust roots, confidence, and blast radius should interact in Resonant Membership.

For lifecycle behavior, see [`MEMBERSHIP.md`](MEMBERSHIP.md). For threat cases, see [`THREAT_MODEL.md`](THREAT_MODEL.md).

## Trust Is Protocol State

Trust should not be treated as invisible background configuration.

It affects:

- whose introductions are meaningful
- which witnesses matter
- how far a claim may travel
- what merge rule dominates under conflict
- when quarantine or hysteresis should apply

## Trust Roots

A **trust root** is a source of introduction or authority treated as foundational within a scope.

Trust roots may come from:

- cryptographic identity
- operator policy
- deployment lineage
- previously converged witness history

Trust roots need not be global. A trust root may be authoritative in one scope and merely informative in another.

## Confidence

Confidence is not the same thing as trust.

- trust is about source credibility
- confidence is about the current composite belief after source, freshness, corroboration, and conflict are considered

A system should make that distinction visible.

## Witness Weighting

Witnesses should not be treated as interchangeable.

Relevant factors may include:

- source trust weight
- freshness
- proximity to the subject
- diversity across failure domains
- equivocation history

This is why a witness set is more than a count.

## Blast Radius

Trust should affect blast radius, not just acceptance.

For example:

- a high-trust introduction may widen scope faster
- a low-trust but fresh claim may remain local pending corroboration
- a disputed claim may propagate as residue rather than as accepted truth

The protocol should make these transitions explicit.

## Hysteresis

Trust-sensitive systems need hysteresis to avoid oscillation.

Without it:

- witness churn becomes state churn
- partitions produce flip-flopping acceptance
- healing traffic reopens recently closed disputes

Hysteresis is therefore not just an implementation tweak. It is part of correctness under uncertain evidence.
