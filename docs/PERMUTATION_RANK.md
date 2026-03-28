# Permutation Rank

Permutation rank is a first-class protocol primitive in Resonant Membership.

It is a seeded, deterministic ordering over a candidate peer set. The important property is not only determinism. It is accountable determinism: multiple observers can reconstruct why a particular peer order, witness set, fanout slice, or rendezvous set was chosen from the same inputs.

For topology-aware use, see [`TOPOLOGY.md`](TOPOLOGY.md). For merge and repair behavior, see [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md).

## What It Is

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

The resulting order can then be reused for multiple selection tasks without falling back to local iteration order or accidental enumeration bias.

## Why Seeded Deterministic Ordering Matters

Without deterministic ordering, systems quietly inherit ambiguity from:

- hash map iteration order
- arrival timing
- host-local peer enumeration
- implementation-specific sorting quirks

That ambiguity is not free. It makes it harder to answer:

- why were these witnesses chosen?
- why did this repair path begin there?
- why did two scopes contact different peers for the same subject?

Permutation rank is a discipline against accidental choice masquerading as protocol choice.

## Accountable Fanout

Fanout is usually discussed as a count. Permutation rank turns it into an accountable choice.

Instead of:

- "contact any 3 peers"

the protocol can say:

- "contact the first 3 ranked peers for this subject, scope, and epoch"

That makes fanout:

- reproducible
- explainable
- stable enough for audit
- less vulnerable to local enumeration accidents

## Rendezvous Selection

Partition healing and anti-entropy often require a bounded rendezvous set.

Permutation rank gives a principled way to choose that set:

- derive rank from healing seed material
- select the first `k` peers or the first acceptable peers by policy
- reuse the same ordering for deterministic reunion across observers

This matters because healing under partial observability benefits from shared expectations about who should meet first.

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

That is useful both for correctness and for operator trust.

## Tie-Breaking

Distributed systems accumulate ties:

- equally fresh claims
- equally trusted witnesses
- equally plausible rendezvous candidates

Tie-breaking should not collapse into "whatever this node happened to look at first."

Permutation rank provides a deterministic arbitration surface when higher-order semantic distinctions are exhausted.

## Auditability and Reproducibility

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

## Tradeoffs

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

## Attack Surfaces

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

## Non-Claims

Permutation rank is not a full consensus primitive, not a Byzantine guarantee, and not a substitute for merge semantics. It is an accountable ordering primitive that makes downstream protocol behavior more reproducible and more legible.
