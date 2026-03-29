# Protocol Primitives

This document defines the core protocol primitives used across the Resonant Membership docs.

See also:

- [`ABSTRACT.md`](ABSTRACT.md) for framing
- [`GLOSSARY.md`](GLOSSARY.md) for the compact terminology index
- [`INVARIANTS.md`](INVARIANTS.md) for the repo's semantic invariants
- [`SEMANTICS.md`](SEMANTICS.md) for decision surfaces and lifecycle skeletons
- [`MEMBERSHIP.md`](MEMBERSHIP.md), [`TRUST.md`](TRUST.md), and [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md) for behavioral chapters
- [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) and [`ARBORITIONS.md`](ARBORITIONS.md) for the two distinctive protocol primitives

## What Problem This Section Solves

Systems papers often blur at the boundary between vocabulary and contract.

If a repo says `claim`, `witness`, `scope`, `merge`, or `residue` without making those objects explicit, later chapters can remain locally coherent while still disagreeing about what the protocol is actually manipulating. This document exists to prevent that drift.

It is not a wire-format spec and not an implementation schema. Its job is narrower and more load-bearing: to define the protocol objects that the rest of the treatise is allowed to rely on.

## How To Read These Primitives

Each primitive below answers five questions:

- what it is
- why it exists
- which conceptual contents are required
- which invariants apply
- where it participates in the protocol lifecycle

The field lists are semantic rather than syntactic. A conforming design could encode them in different ways, but it should not quietly omit their meaning.

## Subject

### What It Is

A **subject** is the node, service, endpoint, identity, or membership-bearing entity about which the protocol makes claims.

### Why It Exists

The protocol needs one stable referent for introduction, witness, dispute, merge, quarantine, and removal. Without a stable subject, witness history and scoped belief cannot accumulate coherently.

### Required Conceptual Contents

- stable identity within the relevant protocol context
- enough classification to know what kind of thing the subject is
- scope-relevant lineage or provenance when that affects admission or trust

### Invariants

- a subject must be stable enough to accumulate history across rounds
- subject identity must not be conflated with any one witness or introducer
- subject identity must survive disagreement even when belief about the subject changes

### Lifecycle Participation

Subjects are introduced, witnessed, accepted provisionally or strongly, disputed, quarantined, healed, or removed.

## Claim

### What It Is

A **claim** is transmissible protocol content asserting something about a subject, its state, its reachability, its trust standing, or its scoped membership status.

### Why It Exists

Observation alone is local. A claim is what can be introduced, propagated, challenged, merged, and preserved as residue.

### Required Conceptual Contents

- subject reference
- asserted state or proposition
- provenance or source identity
- scope of intended meaning
- freshness context such as epoch, generation, or time window

### Invariants

- a claim is not self-justifying
- scope is part of a claim's meaning, not metadata glued on later
- freshness and provenance must remain inspectable enough to compare competing claims

### Lifecycle Participation

Claims are introduced, witnessed, disseminated, merged, narrowed, quarantined, revoked, or preserved as unresolved residue.

## Observation

### What It Is

An **observation** is locally derived evidence such as direct contact, timeout, challenge-response, topology evidence, or administrative inspection.

### Why It Exists

The protocol needs a distinction between private local evidence and transmissible protocol content. That distinction prevents received rumor from masquerading as direct contact.

### Required Conceptual Contents

- observing party
- observed subject
- evidence type or observation mode
- freshness context
- enough local context to explain why the observation was produced

### Invariants

- an observation is local evidence, not yet composite belief
- directness versus indirectness must remain legible
- observations may inform claims without being equivalent to claims

### Lifecycle Participation

Observations feed witness records, confidence formation, dispute, suspicion, quarantine, and repair.

## WitnessRecord

### What It Is

A **WitnessRecord** is the protocol-visible object produced when a witness attaches stance, evidence, freshness, and trust context to a subject or claim.

### Why It Exists

The protocol needs a durable contribution that can be compared, merged, challenged, audited, and carried into repair. A witness alone is an actor; a witness record is the usable protocol object.

### Required Conceptual Contents

- witness identity
- subject or claim reference
- stance such as corroboration, dispute, suspicion, or revocation support
- supporting observation or evidence reference
- freshness context
- scoped trust or confidence contribution

### Invariants

- a witness record must distinguish the witness from the subject being discussed
- a witness record must remain attributable enough to evaluate trust and diversity
- witness records must not silently collapse distinct stances into one undifferentiated vote

### Lifecycle Participation

Witness records are assembled during introduction, widening, suspicion, merge, healing, and post-partition reconciliation.

## MembershipView

### What It Is

A **MembershipView** is a scoped structured belief about subjects, their states, and the confidence attached to those states.

### Why It Exists

This repo treats membership as belief state rather than flat list membership. The membership view is the object that carries that belief.

### Required Conceptual Contents

- scoped set of known or believed subjects
- current belief state for each subject
- confidence or uncertainty structure
- visible residue or disagreement still attached to the view
- freshness or epoch context for the view itself

### Invariants

- a membership view is scoped, not automatically global
- local convergence may remain globally tentative
- a view must be able to carry visible disagreement rather than flattening it into false certainty

### Lifecycle Participation

Membership views are propagated, compared, summarized, merged, healed, and inspected by operators.

## Scope

### What It Is

A **Scope** is the bounded audience, jurisdiction, or semantic domain in which a claim or belief state is meaningful.

### Why It Exists

Without scope, the protocol cannot distinguish local acceptance from wider acceptance, or keep trust and blast radius from silently becoming global.

### Required Conceptual Contents

- boundary of relevance or jurisdiction
- relationship to surrounding hierarchy
- authority or trust assumptions active within the scope
- rules for widening, narrowing, or crossing scope boundaries

### Invariants

- scope is part of meaning, not just routing
- trust and acceptance are scoped unless explicitly widened
- a claim meaningful in one scope may remain tentative in another

### Lifecycle Participation

Scope shapes introduction, witness eligibility, dissemination, merge authority, healing, quarantine, and operator interpretation.

## Epoch

### What It Is

An **Epoch** is the freshness or generation context under which membership, policy, or witness decisions are interpreted.

### Why It Exists

The protocol needs a visible way to distinguish current collective conditions from stale ones. Without epochal context, old authority can masquerade as current authority.

### Required Conceptual Contents

- generation identifier or equivalent freshness boundary
- relation to membership view or policy version
- enough context to compare old and new decisions safely

### Invariants

- stale epochs must not silently dominate current ones
- epoch context must be inspectable enough to explain acceptance, merge, and ceremony-like decisions
- epoch changes must not be implicit if they materially change authority

### Lifecycle Participation

Epochs shape candidate formation, witness selection, merge admissibility, healing, and replay or stale-claim rejection.

## Residue

### What It Is

A **Residue** is visible unresolved disagreement retained by the protocol rather than flattened into false certainty.

### Why It Exists

Under partial observability, some conflicts remain meaningful after contact resumes. The protocol needs a way to preserve that fact honestly.

### Required Conceptual Contents

- the conflicting subject, claim, or scope relation
- the unresolved evidence or witness tension
- the scopes in which the conflict remains meaningful
- enough provenance to explain why certainty was withheld

### Invariants

- residue must remain visible rather than silently discarded
- residue is not a bug marker; it is protocol honesty about unresolved state
- summaries may compress residue, but they must not erase its existence

### Lifecycle Participation

Residue appears during merge, quarantine, scoped disagreement, partition healing, and operator review.

## MergeInput

### What It Is

A **MergeInput** is the admissible set of observations, witness records, provenance, trust context, freshness context, and prior residue brought into a merge decision.

### Why It Exists

The repo repeatedly claims that merge is not set union. Merge input is the object that makes that discipline explicit.

### Required Conceptual Contents

- candidate claims or belief fragments
- supporting witness records and observations
- provenance and admissibility context
- scope and freshness context
- prior residue or merge history that still matters

### Invariants

- merge input must distinguish admissible evidence from merely present evidence
- arrival order alone must not define merge semantics
- merge input should remain reconstructable enough for audit

### Lifecycle Participation

Merge input is assembled during direct contact, anti-entropy, deterministic reunion, quarantine review, and repair.

## MergeOutcome

### What It Is

A **MergeOutcome** is the protocol-visible result of reconciling a merge input.

### Why It Exists

The protocol needs more than one terminal answer. A merge may yield convergence, scoped disagreement, quarantine, or preserved residue.

### Required Conceptual Contents

- resulting scoped belief state
- any retained residue
- explanation of what dominated and what did not
- resulting propagation or repair posture

### Invariants

- a merge outcome must be able to preserve uncertainty honestly
- tie-breaking must be explainable when used
- merge outcome should remain attributable to the evidence classes that produced it

### Lifecycle Participation

Merge outcomes feed dissemination, operator inspection, healing, quarantine decisions, and later rounds of witness or repair.

## TrustRoot

### What It Is

A **TrustRoot** is a source or lineage treated as foundational within a scope for introduction or authority.

### Why It Exists

Trust cannot remain an invisible scalar if it changes blast radius, admissibility, and merge priority. The protocol needs a visible object for scoped foundational standing.

### Required Conceptual Contents

- source identity or lineage
- scope in which the standing applies
- basis for trust-root status such as policy, identity lineage, or earned standing
- visible conditions under which that standing narrows, suspends, or is revoked

### Invariants

- trust-root standing is scoped, not silently global
- trust-root status must be distinguishable from ordinary witness weight
- promotion and demotion may be policy-shaped, but their effects must remain visible

### Lifecycle Participation

Trust roots influence introduction, witness weighting, blast radius, merge admissibility, quarantine review, and repair after trust abuse.

## WitnessSet

### What It Is

A **WitnessSet** is the selected set of witnesses for a particular round, subject, or repair context.

### Why It Exists

Witness quality depends not only on who speaks, but on who was asked and why. The witness set is the accountable selection object.

### Required Conceptual Contents

- candidate pool considered
- selected witnesses
- selection context such as subject, scope, epoch, or repair round
- enough information to reconstruct why these witnesses were selected

### Invariants

- witness count does not equal witness quality
- deterministic selection should be reconstructable and auditable
- diversity and scope appropriateness matter more than raw count alone

### Lifecycle Participation

Witness sets are formed during introduction, dispute, suspicion review, reunion, and repair.

## RepairDigest

### What It Is

A **RepairDigest** is a compact summary exchanged during healing or anti-entropy to indicate scoped membership state, divergence, or missing history without flooding full detail immediately.

### Why It Exists

Healing needs summaries before it can decide where deeper merge is required. The protocol needs a compact object for that first comparison step.

### Required Conceptual Contents

- scope or subtree reference
- freshness or epoch context
- summary of relevant subjects or divergent state
- indication of residue, gaps, or conflicts requiring deeper fetch

### Invariants

- a digest may summarize, but must not misrepresent unresolved disagreement as settled fact
- a digest must remain attributable to a scope and freshness context
- summaries are preliminary repair aids, not authoritative replacements for underlying evidence

### Lifecycle Participation

Repair digests appear during recontact, anti-entropy, partition healing, upward aggregation, and repair overlay selection.

## OperatorOverride

### What It Is

An **OperatorOverride** is an explicit administrative intervention that narrows, widens, suspends, or supersedes ordinary protocol behavior.

### Why It Exists

Operators are part of the system's reality. The protocol becomes less honest, not more, if administrative intervention is possible but invisible.

### Required Conceptual Contents

- acting operator or authority identity
- target subject, scope, trust root, or repair process
- action taken
- reason or justification surface
- freshness context and visibility of when the override occurred

### Invariants

- operator intervention must be legible as intervention
- overrides must not masquerade as organically converged witness evidence
- overrides may be policy-shaped, but their existence and effect must remain inspectable

### Lifecycle Participation

Operator overrides appear during bootstrap, quarantine, trust repair, scope changes, emergency healing, and post-incident review.

## Primitive Relationships

These primitives fit together in a small recurring loop:

1. a **Subject** is introduced through a **Claim**
2. local **Observation** produces one or more **WitnessRecord** objects
3. a scoped **WitnessSet** and relevant **TrustRoot** values shape belief formation
4. the resulting state becomes part of a **MembershipView**
5. competing views become **MergeInput**
6. reconciliation produces a **MergeOutcome**
7. unresolved conflict remains as **Residue**
8. divergence is summarized through **RepairDigest**
9. extraordinary intervention is recorded as **OperatorOverride**

## Design Invariants

The rest of the repo should be able to assume:

1. introduction is not acceptance
2. witness quality is not reducible to raw count
3. scope and epoch are part of meaning, not peripheral metadata
4. merge outcomes may preserve residue instead of forcing false certainty
5. trust-root effects and operator overrides must remain visible

## Operator Interpretation

A serious system should be able to answer:

- what primitive object is this operator looking at right now?
- which subject and scope does it belong to?
- which witnesses, trust roots, or overrides shaped it?
- whether it represents settled belief, tentative belief, or preserved residue

If those questions cannot be answered, the protocol is still hiding its own state transitions.

## Non-Claims

This document does not prescribe one syntax, one database schema, one network envelope, or one trust calculus. It defines the semantic objects that the rest of the treatise is allowed to rely on without renegotiating their meaning chapter by chapter.
