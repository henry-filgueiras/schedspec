# Resonant Membership

Resonant Membership is a design-first proposal for membership under weak coordination: bootstrap, witness, trust, scoped dissemination, merge rules, partition healing, hierarchy, and operator observability under partial observability.

This document serves as the paper-style abstract and framing layer.

See also:

- [`MANIFESTO.md`](MANIFESTO.md) for the sharper preface energy
- [`PRIMITIVES.md`](PRIMITIVES.md) and [`GLOSSARY.md`](GLOSSARY.md) for shared vocabulary
- [`MEMBERSHIP.md`](MEMBERSHIP.md), [`DISSEMINATION.md`](DISSEMINATION.md), [`TRUST.md`](TRUST.md), [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md), and [`TOPOLOGY.md`](TOPOLOGY.md) for protocol behavior

## Abstract

Distributed systems usually maintain a membership view while lacking direct access to global truth. Under those conditions, liveness detection alone is an insufficient abstraction. A practical system must decide who may introduce a subject, which witnesses deserve belief, how claims should be scoped and disseminated, how contradictory local realities should be merged after partition, and how operators can inspect the resulting convergence process.

Resonant Membership treats gossip not merely as message dissemination but as an epistemic control plane for systems that cannot afford certainty. The proposal centers membership as a structured belief state rather than a flat list, and it promotes scope, provenance, trust weighting, staleness, deterministic ordering, and repair structure to first-class protocol concerns. In this framing, convergence depends on merge semantics rather than restored connectivity alone, and partition healing is understood as negotiated reality merge rather than simple rumor resumption.

Two distinctive primitives receive special attention. **Permutation rank** provides seeded deterministic peer ordering for accountable fanout, witness-set selection, rendezvous choice, tie-breaking, and auditability. **Arboritions** provide adaptive topology-aware dissemination, witness, and repair forests that better match hierarchy, trust boundaries, and failure domains than one flat fanout graph. Together these primitives aim to make convergence more inspectable, less accidental, and more structurally honest under partial observability.

## Problem Setting

Many membership systems quietly assume a world cleaner than the one they actually inhabit.

Bootstrap is trusted implicitly. Dissemination is treated as uniform. Observers are modeled as equally credible. Partitions are temporary inconveniences. Merge is imagined as set union plus timeout. Operators are expected to reconstruct truth from logs after the fact.

That model works only while disagreement is shallow and topology is forgiving.

Resonant Membership assumes a harder world. Some claims arrive through weakly trusted witnesses. Not all peers should receive every claim at the same time. Partitions may last long enough to accumulate contradictory views. Topology and locality shape both cost and credibility. Convergence must be explainable, not merely eventual.

## Design Invariants

The project keeps returning to a small set of invariants:

1. Membership is a belief state, not a list.
2. Every claim has scope, provenance, and staleness.
3. Dissemination without trust weighting is noise amplification.
4. Convergence requires merge semantics, not just restored connectivity.
5. Healing must be rate-limited enough to avoid oscillation.
6. Trust should influence blast radius, not only acceptance.
7. Operator visibility is part of correctness.
8. Partition healing is negotiated reality merge.
9. Partial observability is the normal case, not an edge case.

## What This Paper Spine Is Solving

The docs in this repo are trying to answer one durable systems question: how does a distributed system construct, maintain, dispute, and repair a usable shared belief about membership when no node can inspect global truth directly?

That question immediately forces several others.

- What counts as an introduction rather than an acceptance?
- Which witnesses deserve belief, and within which scope?
- How far should a weak claim be allowed to travel?
- What should happen when two local realities both look internally coherent?
- How should topology shape dissemination and healing?
- What evidence should remain visible when convergence is incomplete?

The design claim is not that these questions disappear under eventual consistency. The claim is that a protocol which fails to model them explicitly is already relying on hidden answers.

## Why The Distinctive Primitives Matter

One of the central ideas in this repo is **permutation rank**: seeded deterministic peer ordering for accountable fanout, rendezvous, tie-breaking, and auditability.

Its value is not merely efficiency. It gives the protocol a legible answer to questions like why a node selected one witness set rather than another, why two peers converged on the same rendezvous choices, and why repair traffic flowed through a particular subset of candidates. Deterministic ordering becomes a discipline against accidental local bias and against explanations that collapse into "that was just the order the runtime happened to see."

Another central idea is **arboritions**: adaptive topology-aware dissemination, witness, and repair trees or overlay forests.

The point is to reject the idea that one flat fanout graph is the natural shape of the protocol. Dissemination, witness gathering, and healing often want related but distinct overlay structures. A serious system should admit that locality, hierarchy, trust, and partition state all reshape the best coordination paths.

## Tradeoffs And Failure Modes

This proposal is not a promise of cheap certainty.

Once trust, scope, and witness quality become first-class, the protocol becomes more honest but also more demanding. It must carry more structure, preserve more disagreement, and expose more of its own uncertainty.

Important costs and failure modes include:

- over-modeling minor disagreement until the system becomes hard to operate
- under-modeling residue and thereby flattening conflict into false certainty
- making deterministic ordering too predictable without sufficient hardening
- allowing topology-aware overlays to become opaque policy rather than inspectable structure
- treating witness quantity as a substitute for witness diversity

The repo does not claim those costs disappear. It argues they should be explicit design surfaces rather than accidental byproducts.

## Operator Understanding

A serious membership system should answer:

- who introduced this subject?
- who corroborated or disputed it?
- which scope accepted the claim?
- which merge rule resolved the conflict?
- where does residue remain?
- which arborition path is currently carrying repair traffic?

Those are structural questions. A system that cannot answer them structurally is under-specified.

## Closing Thread

The design center is simple but nontrivial. Under partial observability, usable convergence depends on witness quality, scope, deterministic ordering, healing discipline, and explicit residue. If any of those disappear behind generic gossip language, the protocol becomes much harder to trust precisely when the system is under stress.

Resonant Membership is therefore not a claim of finality. It is a claim about what must be modeled if distributed systems are going to speak honestly about how they decide who belongs.
