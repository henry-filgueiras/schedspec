# Arboritions

Arboritions are a first-class protocol primitive in Resonant Membership.

An **arborition** is an adaptive topology-aware dissemination, witness, and repair forest. The term is slightly coined on purpose: the concept is more specific than "some tree" and broader than one static spanning structure.

Alias and lineage:

- preferred term in this repo: **arborition**
- near-synonyms: **adaptive overlay forest**, **scoped dissemination trees**, **witness trees**, **repair trees**

If the repo later adopts a more explicit label, the current term should remain cross-linked as lineage rather than silently discarded.

For hierarchy and locality, see [`TOPOLOGY.md`](TOPOLOGY.md). For dissemination behavior, see [`DISSEMINATION.md`](DISSEMINATION.md).

## What Problem They Solve

One flat fanout graph is usually the wrong mental model.

Membership systems do not just spread rumors. They also:

- gather witness
- aggregate scoped belief
- route repair traffic
- limit blast radius
- bridge hierarchy boundaries

Those jobs do not always want the same overlay shape.

Arboritions are the answer to that mismatch: adaptive forests that let dissemination, witness, and repair follow different but related paths.

## Why Not One Flat Fanout Graph

A flat fanout graph hides too many important distinctions:

- local witness versus cross-region propagation
- low-latency dissemination versus high-trust aggregation
- steady-state spread versus partition repair
- scope-local confidence versus global tentative belief

What looks simple as a graph often becomes confusing as a protocol.

## Forest Versus Tree

The forest part matters.

An arborition need not be:

- global
- singular
- permanent

It may instead consist of multiple overlapping or transient trees serving different roles:

- local witness subtree
- upward aggregation subtree
- repair subtree
- cross-scope bridging subtree

That is why "overlay forest" is a useful explanatory alias.

## Dissemination, Witness, and Repair Overlays

These roles often deserve separate emphasis:

- **dissemination overlay:** carries fresh claims through scoped fanout
- **witness overlay:** gathers corroboration or dispute from chosen observers
- **repair overlay:** carries anti-entropy and healing traffic after drift or partition

An arborition may unify these overlays in simple cases or separate them in more complex deployments.

## Adaptation

Arboritions should be allowed to adapt to:

- scope
- trust distribution
- partition state
- locality and cost
- overloaded rendezvous paths
- operator intervention

Adaptation is not decorative. It is what keeps the overlay useful once the system stops looking like a uniform graph.

## Relation To Hierarchy and Locality

Arboritions are shaped by hierarchy rather than layered awkwardly on top of it.

Important influences include:

- rack and subnet locality
- zone and region boundaries
- service or shard affinity
- trust-domain boundaries

That means arboritions are not only transport structures. They are part of the system's policy for how belief should move.

## Operator Visibility

Arboritions are also valuable because they give operators inspectable structure.

A useful system should answer:

- which arborition carried this claim?
- which subtree handled witness gathering?
- which repair subtree is currently active?
- where does upward aggregation bottleneck?
- which scope boundary forced a topology transition?

This is debugging value, not just elegance.

## Tradeoffs

Benefits:

- better locality awareness
- clearer role separation
- bounded blast radius
- more inspectable repair paths

Costs:

- more protocol structure to reason about
- risk of stale or skewed overlay shaping
- possible operator confusion if too many overlay roles are hidden

## Non-Claims

Arboritions are not a universal topology, not a single global spanning tree, and not a guarantee that every dissemination path is optimal. They are a way of making dissemination, witness, and repair overlays explicit enough to reason about.
