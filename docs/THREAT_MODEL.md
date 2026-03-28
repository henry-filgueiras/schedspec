# Threat Model

This document describes the failure and abuse conditions Resonant Membership is intended to reason about.

The goal is not to exhaust every attack. The goal is to keep the protocol honest about the world it inhabits.

For vocabulary, see [`PRIMITIVES.md`](PRIMITIVES.md) and [`GLOSSARY.md`](GLOSSARY.md). For trust-sensitive behavior, see [`TRUST.md`](TRUST.md). For behavior under disagreement, see [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md).

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

## Operator Threat Surface

Operators themselves are part of the threat and recovery model.

Relevant questions include:

- who can authorize bootstrap or scope changes?
- who can override quarantine?
- who can trigger healing or witness re-evaluation?
- how are these actions logged and explained?

Operator actions should be visible enough that the system can distinguish protocol convergence from administrative override.

## Desired Defenses

The protocol should trend toward:

- accountable witness selection
- explicit provenance on claims
- scoped trust rather than global blind acceptance
- preserved residue instead of false certainty
- repair paths that can be inspected and, if needed, constrained

This repo does not claim those defenses are fully implemented. It argues they should be first-class design goals.
