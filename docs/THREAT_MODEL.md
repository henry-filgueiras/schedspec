# Threat Model

This document describes the failure and abuse conditions Resonant Membership is intended to reason about.

The goal is not to exhaust every attack. The goal is to keep the protocol honest about the world it inhabits.

See also:

- [`PRIMITIVES.md`](PRIMITIVES.md) and [`GLOSSARY.md`](GLOSSARY.md) for vocabulary
- [`TRUST.md`](TRUST.md) for trust-sensitive behavior
- [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md) for disagreement and repair behavior

## What Problem This Section Solves

A design-first membership protocol can become misleading if it only describes the cooperative happy path.

This section exists to make sure the rest of the repo is read against hostile timing, stale witnesses, scoped disagreement, and operator intervention under uncertainty.

The point is not to claim exhaustive adversarial coverage. It is to make clear which forms of failure the protocol is trying to surface rather than silently flatten.

## Core Objects And Semantics

The threat model in this repo includes:

- non-malicious delay, omission, and partial visibility
- stale or misleading witnesses
- trust abuse across scope boundaries
- equivocation and replay
- partitions that create competing local convergence stories
- operator actions that can either repair or distort the system's visible state

The crucial semantic claim is that not every disagreement is malice, but every disagreement still has to be accounted for.

## Assumed Conditions

The design assumes:

- partial observability
- message delay, duplication, and omission
- partitions and asymmetric reachability
- stale witnesses
- uneven trust distribution
- topology-induced blind spots
- operator intervention under uncertainty

## Failures

Important non-malicious failure cases include:

- a node is locally reachable but globally isolated
- bootstrap sources are stale or incomplete
- witnesses are accurate but delayed
- clocks or freshness windows create false suspicion
- scopes converge differently because of dissemination lag
- healing traffic overloads the wrong rendezvous set

The system should not misclassify every disagreement as adversarial behavior.

## Adversarial Behaviors

Relevant hostile behaviors may include:

- false introduction of a subject
- replay of stale witness claims
- equivocation between scopes
- trust laundering through weak witnesses
- targeted suppression of repair traffic
- exploitation of deterministic ordering by predicting rendezvous sets

A serious design should recognize that deterministic ordering improves auditability while also creating surfaces that may need diversification or hardening.

## Trust Abuse

Trust can fail in several ways:

- a high-weight witness becomes stale
- trust is borrowed transitively beyond its intended scope
- a local witness is over-trusted globally
- revocation propagates more slowly than acceptance

This is why trust should be scoped and observable rather than treated as an invisible scalar.

## Partition Risks

Partitions create special hazards:

- contradictory local convergence
- duplicate introductions
- conflicting revocations
- trust asymmetry between reunited scopes
- ambiguous residue after healing

Partition healing must be designed to preserve evidence of these hazards rather than silently overwriting them.

## Design Invariants

1. disagreement should not be automatically interpreted as malice
2. deterministic selection should remain auditable even when it is attacked
3. trust abuse must be visible as scoped behavior, not only as final state
4. operator intervention must be legible as intervention

## Tradeoffs And Failure Modes

Defensive structure has its own cost.

Stronger quarantine rules can delay legitimate convergence. More cautious healing can preserve too much residue for too long. Deterministic rendezvous can improve explanation while also making selection more predictable to attackers. Better provenance can increase protocol weight and operator cognitive load.

The repo does not deny those tradeoffs. It argues they should be surfaced explicitly so the protocol can be evaluated honestly.

## Operator Interpretation

Operators themselves are part of the threat and recovery model.

Relevant questions include:

- who can authorize bootstrap or scope changes?
- who can override quarantine?
- who can trigger healing or witness re-evaluation?
- how are these actions logged and explained?

Operator actions should be visible enough that the system can distinguish protocol convergence from administrative override.

## Non-Claims

This document does not claim exhaustive adversarial coverage or Byzantine closure. It states the pressure the protocol is expected to survive well enough to remain semantically honest.

## Desired Defenses

The protocol should trend toward:

- accountable witness selection
- explicit provenance on claims
- scoped trust rather than global blind acceptance
- preserved residue instead of false certainty
- repair paths that can be inspected and, if needed, constrained

This repo does not claim those defenses are fully implemented. It argues they should be first-class design goals.
