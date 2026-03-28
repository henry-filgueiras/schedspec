# Trust

This document describes how witness quality, trust roots, confidence, and blast radius should interact in Resonant Membership.

See also:

- [`MEMBERSHIP.md`](MEMBERSHIP.md) for lifecycle behavior
- [`THREAT_MODEL.md`](THREAT_MODEL.md) for threat cases
- [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) for witness-set selection discipline
- [`EXAMPLES.md`](EXAMPLES.md) for trust-sensitive propagation, quarantine, and residue scenarios

## What Problem This Section Solves

Partial observability creates a simple but dangerous question: when two claims conflict, why should the system believe one witness more than another?

This section exists to keep trust from becoming an invisible scalar hidden behind implementation code.

The problem is not merely whether a source is trusted.

The problem is how trust should shape introduction, witness choice, blast radius, merge priority, and the threshold for quarantine or hysteresis. If those choices stay implicit, the protocol will still have a trust model, but no one will be able to say what it is.

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

That scoped quality is what keeps trust from collapsing into a single hidden ranking for the entire system.

## Confidence And Witness Quality

Confidence is not the same thing as trust.

- trust is about source credibility
- confidence is about the current composite belief after source, freshness, corroboration, and conflict are considered

A system should make that distinction visible.

Witnesses should not be treated as interchangeable.

Relevant factors may include source trust weight, freshness, proximity to the subject, diversity across failure domains, and equivocation history. This is why a witness set is more than a count. A large set of correlated weak witnesses may deserve less belief than a smaller, better-distributed set.

## Blast Radius And Hysteresis

Trust should affect blast radius, not just acceptance.

For example:

- a high-trust introduction may widen scope faster
- a low-trust but fresh claim may remain local pending corroboration
- a disputed claim may propagate as residue rather than as accepted truth

The protocol should make these transitions explicit.

Trust-sensitive systems also need hysteresis to avoid oscillation.

Without it, witness churn becomes state churn, partitions produce flip-flopping acceptance, and healing traffic reopens recently closed disputes. Hysteresis is therefore not just an implementation tweak. It is part of correctness under uncertain evidence.

## Design Invariants

1. trust should be visible enough to explain decisions
2. confidence should be distinct from raw trust weight
3. trust should influence blast radius as well as acceptance
4. trust-sensitive transitions should resist oscillation

## Tradeoffs And Failure Modes

Important trust failures include:

- stale but historically trusted witnesses
- over-trusting local witnesses globally
- low-diversity witness sets producing correlated error
- quarantine policies that lag behind confidence collapse

The protocol should not pretend these are edge cases. A system that treats trust as a static allowlist will tend to discover its real trust semantics only after disagreement becomes expensive.

## Operator Interpretation

Operators should be able to ask:

- which trust root introduced this subject?
- which witnesses were decisive?
- why did trust widen or constrain the blast radius?
- why did the system retain residue instead of converging?

If those questions are unanswerable, trust is operating as hidden control flow.
