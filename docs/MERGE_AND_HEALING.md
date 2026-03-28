# Merge and Healing

This document describes how Resonant Membership should reconcile competing views and repair divergence after omission, delay, distrust, or partition.

See also:

- [`MEMBERSHIP.md`](MEMBERSHIP.md) and [`TRUST.md`](TRUST.md) for lifecycle and trust behavior
- [`TOPOLOGY.md`](TOPOLOGY.md) and [`ARBORITIONS.md`](ARBORITIONS.md) for topology-aware repair structures
- [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) for deterministic reunion and rendezvous ordering
- [`DIAGRAMS.md`](DIAGRAMS.md) for canonical reunion and repair diagrams
- [`EXAMPLES.md`](EXAMPLES.md) for partition and repair case studies

## What Problem This Section Solves

Restored connectivity is not enough.

Systems that have diverged need a disciplined way to reconcile competing realities, preserve unresolved disagreement where needed, and avoid oscillating during repair.

The hard case is not simply packet loss.

The hard case is that multiple scopes can accumulate internally coherent but mutually incompatible histories. When contact resumes, the protocol must decide how to compare them, what evidence should dominate, what should remain unsettled, and how the system should make that process legible to operators.

## Why Merge Matters

A membership system eventually reveals itself in merge behavior.

It must answer:

- what happens when observations disagree?
- what dominates: freshness, trust weight, corroboration count, or scope authority?
- when is disagreement preserved as residue rather than flattened?
- how does the system distinguish delayed propagation from genuine conflict?

Merge is therefore not just data-structure union. It is the point where the protocol states what kind of disagreement it is willing to remember.

## Merge Inputs

Relevant inputs may include:

- direct observations
- witness claims
- introducer provenance
- freshness or epoch metadata
- scope
- trust weight
- equivocation evidence
- prior merge history

Different deployments may weight these differently, but a serious design must make the weighting legible.

## Merge Outcomes

A merge should be able to produce outcomes such as:

- accepted convergence
- provisional convergence
- scoped disagreement
- quarantine
- explicit residue pending further witness

The important point is that merge should not force false certainty.

## Residue

Residue is unresolved disagreement retained as visible structure.

Examples:

- two trusted witnesses disagree about subject state
- a region-local scope has accepted a subject while a global scope remains tentative
- a partitioned subtree has not yet been reconciled with a higher-trust witness set

Residue is not failure. It is the protocol's honest record of what remains unsettled.

## Partition Healing

Partition healing should be modeled as an explicit reconciliation phase between accumulated belief states.

Healing must account for:

- stale observations
- contradictory local convergence
- trust asymmetry across the partition
- missed introductions
- duplicated introductions
- delayed revocations or disputes

The goal is not to pretend the partition never happened. The goal is to restore a legible convergent state while preserving what changed during the split.

## Healing Process

A healing process may look like:

1. detect recontact between previously separated scopes
2. exchange compact summaries of local membership belief
3. select accountable rendezvous peers by permutation rank
4. merge witness histories and unresolved residue
5. disseminate repair decisions along topology-aware repair paths
6. surface remaining conflict to operators if convergence is still partial

This is one place where deterministic ordering and topology-aware structure matter most.

## Tie-Breaking

Tie-breaking should not be opaque.

If two equally fresh claims compete, a design may use:

- higher aggregate trust weight
- stronger corroboration set
- tighter scope authority
- deterministic permutation-rank tie-break

The point is not which choice is universally correct. The point is that the choice should be reconstructable and auditable.

## Design Invariants

1. merge should not erase disagreement merely to simplify state
2. healing should preserve provenance and residue where needed
3. deterministic reunion should be inspectable
4. healing must be rate-limited enough to avoid oscillation

## Tradeoffs And Failure Modes

Important failure modes include:

- flattening scoped conflict too early
- over-weighting freshness against trust
- healing storms after broad recontact
- residue becoming invisible once summaries collapse detail

The protocol should make these costs legible rather than treating them as operational accidents.

## Operator Interpretation

Operators should be able to inspect:

- first divergence point
- witnesses that drove convergence
- scopes still in conflict
- residue carried forward after merge
- repair paths currently active

Healing that cannot explain itself will be distrusted precisely when it matters most.

## Closing Thread

Partial observability means no scope gets the whole world for free. Witness quality decides which claims deserve belief. Scope decides where that belief is allowed to matter. Deterministic ordering gives healing a reproducible shape instead of a situational improvisation. Residue records what remains unresolved rather than laundering conflict into premature agreement.

If Resonant Membership has a single systems claim, it is that healing is not a return to innocence. It is the disciplined construction of a shared reality after the system has already learned that its observers can diverge.

## Non-Claims

This document does not freeze one merge calculus. It specifies the kind of merge and healing behavior the protocol must make explicit.
