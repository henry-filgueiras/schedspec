# Permutation Rank

Permutation rank is a first-class protocol primitive in Resonant Membership.

It is a seeded, deterministic ordering over a candidate peer set. The important property is not only determinism. It is accountable determinism: multiple observers can reconstruct why a particular peer order, witness set, fanout slice, or rendezvous set was chosen from the same inputs.

See also:

- [`PRIMITIVES.md`](PRIMITIVES.md) for the shared vocabulary
- [`TOPOLOGY.md`](TOPOLOGY.md) for topology-shaped use
- [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md) for deterministic reunion and tie-breaking
- [`DIAGRAMS.md`](DIAGRAMS.md) for peer-selection diagrams

## What Problem This Primitive Solves

Distributed protocols make selection decisions constantly. They choose who to contact, who should witness a claim, who should rendezvous after partition, and how to break ties when higher-order semantic differences run out.

Too often those choices quietly inherit local accidents:

- hash map iteration order
- arrival timing
- host-local peer enumeration
- implementation-specific sorting quirks

That ambiguity is not free. It turns protocol behavior into hidden control flow. Permutation rank exists to make those choices reproducible, inspectable, and accountable.

## Definition

Given:

- a seed
- a candidate peer set
- a deterministic ranking function

the protocol computes an ordered peer list:

```text
rank = permutation_rank(seed, candidates)
```

The seed may include:

- subject identity
- scope identifier
- epoch
- repair round
- cluster identity

The resulting order can then be reused for multiple selection tasks without falling back to accidental enumeration bias.

## Why Seeded Deterministic Ordering Matters

Determinism by itself is not enough. The seed and ranking function matter because they explain why the order exists in the shape it does.

That makes it possible to answer questions like:

- why were these witnesses chosen?
- why did two observers converge on the same rendezvous peers?
- why did repair traffic begin with this subset of nodes?
- why did this tie resolve in this direction instead of another?

Permutation rank is therefore not a micro-optimization. It is a discipline against accidental choice masquerading as protocol choice.

## Accountable Fanout

Fanout is usually discussed as a count. Permutation rank turns it into an accountable choice.

Instead of saying:

- contact any `3` peers

the protocol can say:

- contact the first `3` ranked peers for this subject, scope, and epoch

That makes fanout:

- reproducible
- explainable
- stable enough for audit
- less vulnerable to local enumeration accidents

It also makes blast radius easier to reason about, because the order of consideration is now visible protocol state rather than runtime residue.

## Rendezvous Selection

Partition healing and anti-entropy often require a bounded rendezvous set. Some peers must meet first, exchange summaries, and begin the work of deterministic reunion.

Permutation rank gives a principled way to choose that set:

- derive rank from healing seed material
- select the first `k` peers or the first acceptable peers by policy
- reuse the same ordering for deterministic reunion across observers

This matters because healing under partial observability benefits from shared expectations about who should meet first. Without that discipline, reunion can become noisy, redundant, and hard to explain.

## Deterministic Witness-Set Selection

Witness selection is one of the most sensitive uses of permutation rank.

A witness set should not be:

- accidental
- hidden
- dependent on local enumeration order

Permutation rank makes it possible to say:

- which candidates were eligible
- which order they were considered in
- which subset was actually selected
- why two observers converged on the same witness set

That is useful both for correctness and for operator trust. It is much easier to reason about witness quality when witness selection itself is legible.

## Merge Tie-Breaking

Distributed systems accumulate ties:

- equally fresh claims
- equally trusted witnesses
- equally plausible rendezvous candidates

Tie-breaking should not collapse into "whatever this node happened to look at first."

Permutation rank provides a deterministic arbitration surface when higher-order semantic distinctions are exhausted. It does not replace merge semantics, but it prevents the tail end of the merge decision from dissolving into implementation accident.

## Auditability And Reproducibility

Permutation rank is valuable because it turns hidden control flow into inspectable protocol state.

Operators should be able to inspect:

- the seed
- the candidate set
- the ranked order
- the actual selected slice

That makes post hoc explanation feasible:

- why this witness set?
- why this rendezvous pair?
- why this fanout path?
- why this tie-break outcome?

Without reproducibility, the system can report its decision but not really explain it.

## Design Invariants

1. ordering decisions should be reconstructable from visible inputs
2. rank should shape selection, not silently replace trust or scope policy
3. witness and rendezvous selection should not depend on host-local enumeration order
4. tie-breaking should remain accountable after higher-order semantics are exhausted

## Tradeoffs And Attack Surfaces

Permutation rank is not free.

Benefits:

- reproducibility
- auditability
- less local bias
- easier debugging

Costs and tradeoffs:

- predictability may expose preferred targets
- a bad seed design can create skew or hot spots
- deterministic ranking does not replace trust, scope, or topology policy
- over-reliance on one ranking can create correlated failure behavior

Important attack or failure surfaces include:

- predicting future rendezvous sets
- manipulating seed inputs to bias witness selection
- concentrating influence on peers that often rank early
- treating rank as authority rather than ordering

A serious design should therefore combine permutation rank with:

- topology constraints
- trust weighting
- diversity rules
- bounded hysteresis or reranking policy where necessary

## Operator Interpretation

A useful system should let an operator answer:

- what seed produced this order?
- which candidates were eligible but not selected?
- why did these two scopes choose the same rendezvous peers?
- where did policy prune the ranked order before use?

That is the practical meaning of accountable ordering.

## Non-Claims

Permutation rank is not a full consensus primitive, not a Byzantine guarantee, and not a substitute for merge semantics. It is an accountable ordering primitive that makes downstream protocol behavior more reproducible and more legible.
