# Semantics

This document is the semantic contract layer for Resonant Membership.

It exists to answer a narrower and more load-bearing question than the framing docs: what are the actual protocol objects, fields, and decision surfaces that the rest of the repo keeps referring to?

See also:

- [`PRIMITIVES.md`](PRIMITIVES.md) and [`GLOSSARY.md`](GLOSSARY.md) for compact vocabulary
- [`MEMBERSHIP.md`](MEMBERSHIP.md), [`DISSEMINATION.md`](DISSEMINATION.md), [`TRUST.md`](TRUST.md), and [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md) for the chapter-scale semantics
- [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) and [`ARBORITIONS.md`](ARBORITIONS.md) for the two most distinctive protocol primitives
- [`DIAGRAMS.md`](DIAGRAMS.md) and [`EXAMPLES.md`](EXAMPLES.md) for visual and worked semantic companions
- [`EVALUATION.md`](EVALUATION.md) for how these objects and decisions should be judged under stress

## What Problem This Section Solves

The repo already has a framing layer, a set of core chapters, and a strong proving ground in examples and diagrams. What it still needs is one place that makes the protocol model feel semantically crisp rather than merely suggestive.

This document fills that gap. It does not define a wire format or an implementation API. It defines the semantic objects that a conforming design would need to preserve if the rest of the repo is to mean what it says.

## Scope Of This Document

This is not:

- a transport specification
- a storage schema
- a consensus algorithm
- an implementation status report

This is:

- a semantic contract for the protocol model
- a statement of what kinds of objects the system reasons over
- a statement of what kinds of decisions must remain legible

Where the repo is intentionally open, this document says so explicitly. Where the repo is semantically committed, this document tries to say what the commitment actually is.

## Core Protocol Objects

### Subject

A **subject** is the entity about which membership claims are made.

It may be:

- a node
- a service instance
- a gateway
- an endpoint identity
- another scoped participant in the membership system

Dimensions that matter:

- stable identity
- scope-relative meaning
- epoch-relative freshness
- current and prior claim history

Invariants:

- a subject should have a stable identity within the scope in which it is discussed
- a subject is not equivalent to its latest claim
- a subject may have different belief states in different scopes at the same time

Operator visibility:

- subject identity
- current scoped status
- introduction provenance
- recent claim and witness history

Intentionally open:

- exact identity encoding
- whether subject identifiers are globally unique or only scope-unique

Semantically required:

- a subject must be referable across multiple claims, witnesses, and repair rounds

### Claim

A **claim** is transmissible protocol content about a subject.

A claim is not the same thing as an observation. A claim is what moves between observers and scopes.

Fields or dimensions that matter:

- subject
- asserted state or membership fact
- scope
- introducer or provenance root
- epoch or freshness context
- optional attached evidence

Invariants:

- a claim should carry enough context to be evaluated outside the node that first emitted it
- a claim without scope is semantically incomplete
- a claim should be distinguishable from the evidence used to form it

Operator visibility:

- claim body
- scope
- provenance
- freshness or epoch context
- whether the claim is accepted, disputed, quarantined, or preserved as residue

Intentionally open:

- exact claim syntax
- exact proof attachment format

Semantically required:

- a claim must be attributable, scoped, and freshness-aware

### Observation

An **observation** is locally derived evidence about a subject.

Examples include:

- direct reachability
- timeout or absence
- challenge-response result
- topology-local evidence
- local operator action

Dimensions that matter:

- observer identity
- observation type
- freshness
- locality
- uncertainty or ambiguity

Invariants:

- an observation is local evidence, not a system-wide fact
- observations may conflict without implying protocol failure
- observations should remain distinguishable from claims and from merged outcomes

Operator visibility:

- who observed what
- when it was observed
- what kind of evidence it was

Intentionally open:

- exact evidence model
- how raw or summarized an observation may be

Semantically required:

- observations must be attributable and freshness-aware

### Witness Record

A **witness record** is the protocol-visible contribution a witness makes to a claim or subject state.

It is more structured than "this node voted yes."

Dimensions that matter:

- witness identity
- observed or corroborated claim
- trust-relevant context
- freshness
- corroborating, disputing, or ambiguous stance

Invariants:

- witness records should remain attributable
- witness quality matters as much as witness count
- witness records should be mergeable without being flattened into a single scalar too early

Operator visibility:

- which witnesses participated
- what each witness contributed
- which witness records were decisive

Intentionally open:

- whether witness records are signed, summarized, or nested

Semantically required:

- witness records must preserve enough structure to support explanation and repair

### Trust Weight And Confidence

**Trust weight** is the credibility attached to a source or witness.

**Confidence** is the strength of the current belief once trust, freshness, corroboration, and conflict have been considered together.

Dimensions that matter:

- source credibility
- freshness
- corroboration diversity
- conflict pressure
- scope-relative authority

Invariants:

- trust weight and confidence must remain distinct
- trust should influence blast radius as well as acceptance
- confidence should be explainable in terms of the evidence and weights that produced it

Operator visibility:

- trust roots
- effective trust weight
- confidence state
- reasons confidence widened, narrowed, or collapsed

Intentionally open:

- the exact trust calculus
- whether weights are numeric, ordinal, or category-based

Semantically required:

- the distinction between source credibility and current belief strength must not collapse

### Scope

A **scope** is the bounded audience or jurisdiction in which a claim is meaningful.

Examples:

- rack
- zone
- region
- service
- trust domain
- global aggregation surface

Dimensions that matter:

- membership audience
- authority boundary
- dissemination boundary
- repair boundary

Invariants:

- scope is part of claim meaning, not just metadata
- a claim may be strongly accepted in one scope and provisional in another
- scope should influence both propagation and merge authority

Operator visibility:

- current scope of the claim
- higher and lower related scopes
- scope transitions during propagation or healing

Intentionally open:

- exact scope hierarchy representation

Semantically required:

- scopes must be explicit enough to explain why a claim mattered where it did

### Freshness And Epoch

**Freshness** is the temporal relevance of a claim, observation, or witness contribution.

An **epoch** is bounded time or generation context used for ordering, freshness, and repair reasoning.

Dimensions that matter:

- generation boundary
- local or global time context
- recency
- staleness

Invariants:

- freshness should influence confidence and merge
- epoch boundaries should be visible enough to explain ordering and repair choices
- stale evidence should not silently behave like fresh evidence

Operator visibility:

- epoch context
- freshness windows
- stale versus current evidence

Intentionally open:

- exact clocking model
- exact freshness thresholds

Semantically required:

- the system must be able to distinguish fresh, stale, and superseded evidence
- epoch or equivalent generation context must be visible enough to support explanation of ordering and repair

### Residue

**Residue** is unresolved disagreement preserved as visible structure.

It exists so the system does not pretend it knows more than it has earned.

Dimensions that matter:

- conflicting witnesses or scopes
- unresolved merge outcomes
- preserved divergence history

Invariants:

- residue is not a logging artifact; it is protocol state
- residue should remain visible when disagreement is semantically important
- residue should not be erased merely to simplify presentation

Operator visibility:

- where disagreement remains
- which evidence remains in conflict
- whether residue is shrinking, persisting, or widening

Intentionally open:

- exact residue encoding

Semantically required:

- unresolved disagreement must remain inspectable when the protocol cannot yet justify convergence

### Quarantine State And Revocation

**Quarantine** is bounded suspension of propagation, acceptance, or both.

**Revocation** is the transition by which prior acceptance or trust is explicitly withdrawn.

Dimensions that matter:

- scope of quarantine or revocation
- reason
- duration or hysteresis relation
- affected subject or witness

Invariants:

- quarantine should be explicit rather than hidden in timers
- revocation should be visible as a state transition, not only as absence of acceptance
- quarantine and revocation should preserve enough history to support repair and explanation

Operator visibility:

- why a subject or witness was quarantined or revoked
- when that state began
- what evidence drove the transition

Intentionally open:

- exact quarantine policy
- exact revocation workflow

Semantically required:

- quarantine and revocation must be explainable protocol states, not informal implementation behavior
- quarantine must be able to suspend propagation, acceptance, or both in a scoped way

## Decision Surfaces

### Merge Input

A **merge input** is the structured set of materials brought into reconciliation.

Semantically relevant fields include:

- observations
- claims
- witness records
- provenance
- trust weight
- scope
- freshness
- prior merge history
- residue already in play

Invariants:

- merge input should preserve enough structure to distinguish evidence from conclusion
- merge should not silently discard provenance-critical data before decision time

Operator visibility:

- what evidence entered the merge
- which evidence was considered stale, dominant, or conflicting

Intentionally open:

- exact merge-input container shape

Semantically required:

- the merge surface must remain explainable in terms of visible inputs

### Merge Output

A **merge output** is the resulting scoped belief state produced by reconciliation.

Possible forms include:

- accepted convergence
- provisional convergence
- scoped disagreement
- quarantine
- preserved residue

Invariants:

- merge output should not claim stronger certainty than the inputs justify
- merge output should preserve residue where necessary
- merge output should be attributable to explicit decision surfaces rather than hidden defaults

Operator visibility:

- resulting state
- dominant evidence
- preserved disagreement
- reason for tie-break or priority choice

Intentionally open:

- exact output representation

Semantically required:

- the output must remain explainable in terms of the inputs and decision policy

### Permutation-Rank Seed And Candidate-Set Semantics

The **permutation-rank seed** is the visible ordering context used to compute accountable peer order.

The **candidate set** is the policy-eligible set of peers over which that ranking is performed.

Dimensions that matter:

- subject identity
- scope identity
- epoch or repair round
- cluster or region context
- candidate eligibility policy

Invariants:

- candidate-set formation and rank computation must remain conceptually distinct
- the seed must be visible enough to explain why an ordering existed in the shape it did
- ranking must not silently substitute for trust or topology policy

Operator visibility:

- seed
- candidate set
- ranked order
- selected slice

Intentionally open:

- exact hash or ranking function
- exact candidate eligibility policy

Semantically required:

- ordering must be reconstructable from visible inputs

### Arborition Role Semantics

An **arborition** is not just a graph. It is a semantic role surface for how belief moves.

The core roles are:

- **witness overlay:** gathers corroboration or dispute
- **dissemination overlay:** spreads fresh claims through scoped fanout
- **repair overlay:** carries anti-entropy and healing traffic

Dimensions that matter:

- scope
- locality
- trust boundary
- repair state
- parent-proxy relation

Invariants:

- the role of an overlay should remain distinguishable
- witness, dissemination, and repair overlays may coincide, but should not be assumed to do so
- parent-proxy interaction is part of the semantic model, not a transport footnote

Operator visibility:

- which arborition carried a claim
- which subtree handled witness formation
- which subtree handled repair
- where upward aggregation crossed a boundary

Intentionally open:

- exact overlay-construction policy
- exact adaptation algorithm

Semantically required:

- overlay role must remain explainable in terms of protocol purpose, not just network structure

## Claim Lifecycle

A compact claim lifecycle looks like:

1. a subject is introduced into a scope
2. one or more observations are gathered
3. witness records are formed
4. a scoped belief state becomes provisional, accepted, disputed, or quarantined
5. dissemination widens or stays bounded according to trust and scope
6. later merge or healing may revise the visible state

The important point is that a claim does not become a fact merely because it exists. It becomes meaningful through witness, trust, scope, and repair behavior.

## Witness-Selection Skeleton

A compact witness-selection skeleton looks like:

1. identify the subject, scope, and current epoch or repair context
2. form a candidate set using deployment policy, topology constraints, trust floors, and scope boundaries
3. compute permutation rank over that candidate set from a visible seed
4. prune or accept the ranked order according to diversity, locality, and trust policy
5. record which witnesses were selected, which were eligible but excluded, and why
6. expose enough of the process for operator reconstruction

The key semantic distinction is that candidate-set formation is policy-shaped, while the ordering over that candidate set should remain deterministic enough to audit.

## Merge-Decision Skeleton

A compact merge skeleton looks like:

1. collect merge input
2. separate fresh from stale evidence
3. compare provenance, trust, corroboration, and scope authority
4. preserve unresolved disagreement as residue where needed
5. produce accepted, provisional, disputed, quarantined, or residual output
6. expose enough structure for explanation

This is not a full algorithm. It is the minimum semantic skeleton the rest of the repo assumes.

## Healing-Round Skeleton

A compact healing round looks like:

1. detect recontact or divergence requiring repair
2. exchange digests or summaries
3. select bounded reunion peers by permutation rank
4. merge local histories, not just latest endpoints
5. disseminate repair output through repair overlays
6. preserve and expose remaining residue if convergence is incomplete

The key semantic commitment is that healing is an explicit round of reconciliation, not a magical return to pre-partition innocence.

## What Must Remain Deterministic

Some parts of the model should remain deterministic enough to be reconstructable:

- permutation-rank output from a visible seed and candidate set
- the relationship between a specific seed, candidate set, and selected witness or rendezvous slice
- deterministic tie-breaking once higher-order semantic distinctions are exhausted
- visibility of which evidence entered a merge

Determinism here does not mean every deployment must choose the same policy. It means that once a policy is in force, the decisions that claim to be accountable should be reconstructable from visible inputs rather than from runtime accident.

## What May Vary By Deployment Policy

Some parts of the model are intentionally policy-shaped:

- trust-weight calculation
- candidate-set eligibility rules
- scope-specific dissemination thresholds
- quarantine and hysteresis thresholds
- overlay adaptation strategy

The repo is intentionally flexible about policy. It is intentionally much less flexible about whether those policies must remain explainable.

## Operator Visibility Requirements

A serious conforming design should let an operator inspect:

- the current subject state in its relevant scope
- the claims and observations that matter
- the witnesses that were selected and why
- the trust and confidence surfaces that shaped the decision
- the merge input and merge output
- the residue that remains unsettled
- the permutation-rank ordering used for selection
- the arborition path used for dissemination or repair

If those objects cannot be surfaced, then the protocol may still be doing something coherent, but it is not yet doing it honestly enough for this repo’s standard.

## Non-Claims

This document is not a wire-format spec, not an API contract, and not a claim of implementation. It is the semantic backbone for the protocol model described in the rest of the repo.
