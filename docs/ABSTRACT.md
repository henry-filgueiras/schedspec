# Resonant Membership

Resonant Membership is a proposal for reasoning about membership under weak coordination: bootstrap, witness, trust, scoped dissemination, merge rules, partition healing, hierarchy, and operator observability.

The central claim is that membership under partial observability is not merely a question of who is alive. It is a question of how distributed observers form, transmit, dispute, merge, and repair beliefs about who belongs, who is trusted to report, and which view is sufficiently converged to act on.

This document is the long-form "why" of the project. For the sharper thesis lines, see [`MANIFESTO.md`](MANIFESTO.md). For the shared vocabulary, see [`PRIMITIVES.md`](PRIMITIVES.md) and [`GLOSSARY.md`](GLOSSARY.md). For protocol behavior, see [`MEMBERSHIP.md`](MEMBERSHIP.md), [`DISSEMINATION.md`](DISSEMINATION.md), [`TRUST.md`](TRUST.md), [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md), and [`TOPOLOGY.md`](TOPOLOGY.md).

## Thesis

Many membership systems quietly assume a world that is cleaner than the one they inhabit:

- bootstrap is trusted implicitly
- dissemination is uniform
- observers are equally credible
- partitions are temporary inconveniences
- merge is set union plus timeout
- operators can reconstruct truth from logs after the fact

That works only while disagreement is shallow and topology is forgiving.

Resonant Membership assumes a harder world:

- some claims arrive through weakly trusted witnesses
- not all peers should receive every claim at the same time
- partitions may last long enough to accumulate contradictory views
- topology and locality shape cost and credibility
- convergence must be explainable, not merely eventual

In that world:

- introduction matters
- witness quality matters
- dissemination scope matters
- deterministic ordering matters
- merge rules matter
- operator visibility matters

## Membership As Converging Belief

Membership is usually described as a table:

- node `A` is up
- node `B` is down
- node `C` is suspected

That is an insufficient abstraction.

Under partial observability, membership is better understood as a distributed belief state composed of:

- observations
- witness claims
- trust weight
- propagation scope
- merge policy
- unresolved residue

The interesting system question is therefore not just "what is the current set?" but:

- who introduced this node?
- who corroborated the claim?
- which scope accepted it?
- which observers still disagree?
- what repair path remains after a partition?

## Bootstrap, Witness, Trust

Bootstrap is the first trust decision, not a boring prelude.

Every membership system eventually answers:

- who is allowed to introduce a new participant?
- what counts as sufficient witness?
- when is a claim locally credible but globally tentative?

Resonant Membership treats bootstrap and witness as protocol primitives rather than assumptions hidden in provisioning scripts or static seed lists.

Trust is similarly not binary. It may be:

- rooted in static identity or operator policy
- inferred from prior convergence quality
- scoped by zone, rack, service, or role
- reduced by equivocation, staleness, or isolation

## Permutation Rank

One of the central ideas in this repo is **permutation rank**: seeded deterministic peer ordering for accountable fanout, rendezvous, tie-breaking, and auditability.

Its value is not only efficiency. It gives the protocol a legible answer to questions like:

- why did this node witness that claim first?
- why were those peers selected for repair?
- why did two observers choose the same rendezvous set?

Deterministic ordering is a form of protocol accountability.

## Arboritions

Another central idea is **arboritions**: adaptive topology-aware dissemination, witness, and repair trees or overlay forests.

The term is slightly coined on purpose. The point is to emphasize that the dissemination structure should be:

- shaped like a living overlay forest rather than one flat fanout graph
- adaptive to locality, trust, and partition state
- useful for witness gathering, not only rumor spread
- useful for repair after divergence, not only steady-state propagation

If the term is later refined, the concept should remain.

## Merge And Healing

Merge is where a membership system admits what it believes about conflict.

A serious design must answer:

- which observations dominate under disagreement?
- when does trust weight outrank freshness?
- what does a node do with partially corroborated claims?
- how is unresolved disagreement surfaced instead of erased?

Partition healing is similarly not "resume gossip and hope." It is an explicit reconciliation phase between accumulated belief states.

## Hierarchy And Scope

Flat broadcast is often the wrong mental model.

Real systems are shaped by:

- racks
- zones
- regions
- services
- security domains
- trust boundaries

Resonant Membership therefore assumes hierarchy and scoped dissemination are normal conditions, not optional optimizations.

## Operator Observability

Operator visibility is not just metrics and logs.

A useful system should answer:

- which witnesses caused this node to be accepted?
- which partition introduced divergence?
- which merge rule resolved the conflict?
- where does disagreement remain?
- which arborition path is currently carrying repair traffic?

Those are structural questions. A system that cannot answer them structurally is under-specified.

## Anti-Goals

Resonant Membership is not trying to be:

- a generic gossip tutorial
- a polished product page for a nonexistent system
- a proof that trust can be ignored
- a universal topology that fits every environment
- a claim that convergence erases ambiguity

## Current Status

This repository is a design treatise. The point is to make the semantic and protocol commitments legible before an implementation calcifies accidental assumptions.
