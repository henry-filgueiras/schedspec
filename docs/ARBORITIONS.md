# Arboritions

Arboritions are a first-class protocol primitive in Resonant Membership.

An **arborition** is an adaptive topology-aware dissemination, witness, and repair forest. The term is slightly coined on purpose: the concept is more specific than "some tree" and broader than one static spanning structure.

Alias and lineage:

- preferred term in this repo: **arborition**
- near-synonyms: **adaptive overlay forest**, **scoped dissemination trees**, **witness trees**, **repair trees**

If the repo later adopts a more explicit label, the current term should remain cross-linked as lineage rather than silently discarded.

See also:

- [`PRIMITIVES.md`](PRIMITIVES.md) for shared vocabulary
- [`DISSEMINATION.md`](DISSEMINATION.md) for propagation behavior
- [`TOPOLOGY.md`](TOPOLOGY.md) for hierarchy and locality
- [`DIAGRAMS.md`](DIAGRAMS.md) for overlay-forest and repair-path diagrams

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

What looks simple as a graph often becomes confusing as a protocol. The graph is cleaner than the semantics it is asked to carry.

Arboritions make the protocol admit that different kinds of movement want different kinds of shape.

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

That is why "adaptive overlay forest" is a useful explanatory alias. The protocol is not looking for one perfect spanning tree. It is shaping a family of related paths with different responsibilities.

## Dissemination Overlays, Witness Overlays, And Repair Overlays

These roles often deserve separate emphasis:

- **dissemination overlay:** carries fresh claims through scoped fanout
- **witness overlay:** gathers corroboration or dispute from chosen observers
- **repair overlay:** carries anti-entropy and healing traffic after drift or partition

An arborition may unify these overlays in simple cases or separate them in more complex deployments. What matters is that the protocol treats the distinction as meaningful instead of pretending one relay pattern naturally fits all three.

## Adaptation To Locality, Trust, And Partition State

Arboritions should be allowed to adapt to:

- scope
- trust distribution
- partition state
- locality and cost
- overloaded rendezvous paths
- operator intervention

Adaptation is not decorative. It is what keeps the overlay useful once the system stops looking like a uniform graph.

Locality may favor nearby witness paths. Trust may favor different upward aggregation surfaces. Partition state may temporarily reshape repair subtrees around bounded reunion points. A serious protocol should be able to say not only that the overlay changed, but why it changed.

## Parent-Proxy Interaction

Parent-proxy pools are one of the clearest examples of why arboritions matter.

A local scope often cannot or should not communicate upward through unrestricted mesh contact. Instead, it uses a bounded set of higher-level peers for:

- scoped digest relay
- witness summary aggregation
- cross-scope repair traffic
- bounded external visibility

An arborition is the broader overlay context in which those parent-proxy choices make sense. It shows how local witness subtrees connect to upward aggregation paths and how repair paths cross hierarchy boundaries without implying that every node should talk to every higher-level peer directly.

## Relation To Hierarchy And Locality

Arboritions are shaped by hierarchy rather than layered awkwardly on top of it.

Important influences include:

- rack and subnet locality
- zone and region boundaries
- service or shard affinity
- trust-domain boundaries

That means arboritions are not only transport structures. They are part of the system's policy for how belief should move.

## Debugging And Operator Visibility

Arboritions are also valuable because they give operators inspectable structure.

A useful system should answer:

- which arborition carried this claim?
- which subtree handled witness gathering?
- which repair subtree is currently active?
- where does upward aggregation bottleneck?
- which scope boundary forced a topology transition?

This is debugging value, not just elegance. A system that cannot expose these structures leaves operators with message counts and intuition, which is rarely enough during partition or trust-sensitive repair.

## Design Invariants

1. dissemination, witness, and repair overlays should be explicit enough to inspect
2. overlay shape should respond to locality, trust, and partition state
3. forest structure should be allowed when one tree would hide real semantic differences
4. parent-proxy interaction should be part of the overlay model, not an afterthought

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
- more policy coupling between topology, trust, and dissemination

Arboritions are therefore not free. They are useful precisely because they make overlay policy explicit, and explicit policy is harder to ignore than a generic fanout loop.

## Non-Claims

Arboritions are not a universal topology, not a single global spanning tree, and not a guarantee that every dissemination path is optimal. They are a way of making dissemination, witness, and repair overlays explicit enough to reason about.
