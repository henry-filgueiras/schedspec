# Semantic Invariants

This document collects the core semantic invariants implied across the Resonant Membership repo.

See also:

- [`PRIMITIVES.md`](PRIMITIVES.md) for the protocol objects these invariants constrain
- [`SEMANTICS.md`](SEMANTICS.md) for lifecycle and decision-surface structure
- [`MEMBERSHIP.md`](MEMBERSHIP.md), [`TRUST.md`](TRUST.md), and [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md) for the behavioral chapters that apply them

## What Problem This Section Solves

The repo already assumes a strong semantic kernel. Without one place that states those assumptions directly, later chapters have to keep restating them in slightly different words.

This document exists to stop that drift. It is the compact contract for what the proposal keeps treating as non-negotiable.

## How To Read These Invariants

These are semantic invariants, not performance goals and not implementation instructions.

They say what a conforming design must preserve if it wants to remain recognizably within the Resonant Membership proposal. Exact thresholds, pacing, and heuristics may vary by deployment policy; these invariants should not.

## Introduction Is Not Acceptance

Admitting a subject into consideration is not the same thing as accepting it as a member.

The protocol must preserve a visible distinction between:

- introduction
- witness formation
- provisional belief
- stronger scoped acceptance

If those stages collapse together, bootstrap becomes hidden authority.

## Witness Count Does Not Equal Witness Quality

A larger witness set is not automatically a better witness set.

Diversity, trust standing, locality, freshness, and failure-domain spread matter more than raw count alone. A design that counts witnesses without modeling witness quality will amplify correlated error.

## Local Convergence May Remain Globally Tentative

Scoped convergence is real, but not automatically universal.

A local scope may hold a usable belief while a wider scope remains provisional, disputed, or quarantined. The protocol must permit that asymmetry without treating it as corruption.

## Residue Must Remain Visible

Fresh admissible disagreement should not be flattened into false certainty merely to simplify state.

Residue is the protocol's honest record of what remains unresolved. Summaries may compress it, but they must not erase the fact that it exists.

## Healing Is Explicit Reconciliation, Not Resumed Rumor Spread

Partition recovery is not the same thing as ordinary dissemination resuming after delay.

Healing requires explicit comparison of scoped realities, visible merge discipline, and a repair path that can explain why one interpretation dominated or why residue remained.

## Trust Is Scoped, Not Silently Global

Trust standing, trust-root influence, and blast radius must remain scoped unless explicitly widened.

The protocol should not let a local trust assumption leak outward as though it were a universal property of the system.

## Deterministic Selection Must Be Reconstructable And Auditable

When the protocol uses deterministic ordering or deterministic selection, later observers must be able to reconstruct how the result was obtained.

This applies especially to witness-set formation, rendezvous choice, tie-breaking, and repair path activation. Determinism without auditability is only hidden control flow with a nicer name.

## Topology Is Protocol-Relevant

Hierarchy, locality, parent-proxy pools, and overlay shape are not mere transport optimizations.

They affect witness eligibility, dissemination cost, blast radius, and repair credibility. A design that treats topology as invisible plumbing will misdescribe its own behavior.

## Operator Visibility Is Part Of Correctness

The protocol is not correct enough if it converges but cannot explain itself.

Operators should be able to inspect:

- who introduced a subject
- which witnesses mattered
- which scope accepted or disputed a claim
- what residue remains
- which repair path or override changed the outcome

If those surfaces are absent, correctness is being defined too narrowly.

## Non-Claims

This document does not freeze one trust calculus, one merge formula, or one overlay algorithm. It states the semantic conditions those choices must preserve.
