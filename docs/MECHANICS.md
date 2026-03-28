# Mechanics

This document sketches the core protocol loops and decision procedures that would make Resonant Membership feel buildable without pretending it is already implemented.

It is not a wire protocol, storage schema, or benchmark story. It is a mechanics layer: serious algorithm-shaped prose for the semantic model.

See also:

- [`SEMANTICS.md`](SEMANTICS.md) for the protocol objects and decision surfaces
- [`MEMBERSHIP.md`](MEMBERSHIP.md) and [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md) for the behavioral chapters these mechanics support
- [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) and [`ARBORITIONS.md`](ARBORITIONS.md) for the two most distinctive primitives
- [`EXAMPLES.md`](EXAMPLES.md) and [`DIAGRAMS.md`](DIAGRAMS.md) for worked and visual companions
- [`EVALUATION.md`](EVALUATION.md) for how these mechanics should be judged under stress

## What Problem This Section Solves

The repo now has a framing layer, a semantic contract, and a strong proving ground in examples and diagrams. The missing bridge is mechanical shape.

This chapter exists to answer a narrower question:

If a conforming implementation were built, what are the core loops and decision procedures it would likely need to instantiate?

The goal is not to freeze one implementation. The goal is to make the protocol feel buildable enough that the semantics stop floating.

## How To Read These Mechanics

Each mechanism below is described in the same way:

- `Inputs`
- `Outputs`
- `Deterministic surfaces`
- `Policy-shaped surfaces`
- `Operator inspection`

Where helpful, the chapter uses light pseudocode. The pseudocode is semantic pseudocode, not implementation code.

## Subject Introduction Into A Scope

This is the entry loop by which a subject first becomes protocol-visible inside a scope.

**Inputs**

- subject identity
- introducer identity
- target scope
- epoch or freshness context
- optional attached evidence
- current trust and quarantine state for the introducer

**Outputs**

- a scoped introduction claim
- initial operator-visible provenance
- either admission to witness formation or immediate scoped quarantine/rejection

**Mechanics**

```text
introduce(subject, introducer, scope, epoch, evidence):
  assert subject is referable in scope
  assert introducer is visible and not silently anonymous

  claim = make_claim(
    subject=subject,
    asserted_state="introduced",
    scope=scope,
    provenance=introducer,
    epoch=epoch,
    evidence=evidence
  )

  if introducer is explicitly quarantined in scope:
    return quarantined_introduction(claim, reason="introducer quarantine")

  return admitted_for_witnessing(claim)
```

**Deterministic surfaces**

- the claim must carry visible subject, scope, provenance, and freshness context
- a visible quarantine state must not be bypassed implicitly

**Policy-shaped surfaces**

- what counts as sufficient attached evidence
- whether an introducer needs a trust floor for admission
- whether some scopes require stronger bootstrap authority than others

**Operator inspection**

- who introduced the subject
- into which scope
- with what epoch or freshness context
- whether the introduction was admitted, quarantined, or rejected

## Candidate-Set Formation For Witnesses

This mechanism forms the eligible witness population before ordering occurs.

**Inputs**

- subject
- scope
- current epoch or repair round
- topology-local peer inventory
- trust and quarantine state
- diversity and locality policy

**Outputs**

- candidate witness set
- exclusion reasons for ineligible peers

**Mechanics**

```text
form_candidates(subject, scope, epoch, peers):
  candidates = []

  for peer in peers:
    if not peer.visible_in(scope):
      continue
    if peer.quarantined_for_witnessing(scope):
      continue
    if not peer.meets_locality_policy(subject, scope):
      continue
    if not peer.meets_trust_floor(scope):
      continue

    candidates.append(peer)

  return candidates
```

**Deterministic surfaces**

- candidate-set formation must be conceptually distinct from ranking
- excluded peers should have inspectable reasons

**Policy-shaped surfaces**

- trust floors
- locality constraints
- diversity constraints
- whether the candidate set is narrow and scope-local or broad and cross-boundary

**Operator inspection**

- which peers were eligible
- which peers were excluded
- why exclusions happened

## Permutation-Rank Computation And Ranked Selection

This mechanism applies accountable deterministic ordering to an already-formed candidate set.

**Inputs**

- visible seed material
- candidate set
- deterministic ranking function
- requested slice size or selection goal

**Outputs**

- ranked candidate order
- selected witness or rendezvous slice

**Mechanics**

```text
select_ranked(seed_material, candidates, selection_goal):
  seed = encode_seed(seed_material)
  rank = permutation_rank(seed, candidates)
  selected = prune_rank_for_goal(rank, selection_goal)
  return (seed, rank, selected)
```

The crucial step is `prune_rank_for_goal`. The ranked order is deterministic; the final selected slice may still be shaped by policy such as diversity, locality, or load constraints.

**Deterministic surfaces**

- seed construction from visible inputs
- the ranking function over a specific candidate set
- the ranked order itself

**Policy-shaped surfaces**

- how the seed is assembled from the allowed dimensions
- how many peers to select
- which policy constraints may prune the ranked order

**Operator inspection**

- the seed
- the candidate set
- the ranked order
- the selected slice
- the reasons policy skipped any high-ranked candidates

## Scoped Dissemination Decisions

This mechanism decides whether, when, and how far a claim should spread.

**Inputs**

- claim
- current scoped belief state
- witness records
- confidence state
- dissemination policy for the scope
- available dissemination overlay

**Outputs**

- bounded local dissemination
- upward propagation
- deferred propagation
- explicit suppression or quarantine

**Mechanics**

```text
decide_dissemination(claim, belief_state):
  if belief_state.quarantined:
    return suppress("quarantined")

  if belief_state.confidence < scope_local_threshold(claim.scope):
    return keep_local("insufficient confidence for wider spread")

  if not blast_radius_policy_allows_widening(claim, belief_state):
    return keep_scoped("blast radius constrained")

  return disseminate_via_overlay(claim, role="dissemination")
```

**Deterministic surfaces**

- visible relation between belief state and dissemination decision
- explicit suppression versus implicit silence

**Policy-shaped surfaces**

- confidence thresholds
- blast-radius policy
- whether dissemination stays local, regional, or moves upward

**Operator inspection**

- why the claim spread or did not spread
- which scope boundary was crossed
- which overlay path carried the claim

## Parent-Proxy Upward Aggregation

This mechanism moves scoped summaries upward through bounded hierarchy surfaces.

**Inputs**

- scoped claim or scoped digest
- parent-proxy pool for the next higher scope
- confidence and residue state
- upward aggregation policy

**Outputs**

- scoped digest sent upward
- deferred upward propagation
- refusal to widen scope

**Mechanics**

```text
aggregate_upward(scoped_state, parent_proxy_pool):
  if not upward_policy_allows(scoped_state):
    return retain_local("not ready for upward aggregation")

  summary = make_scoped_digest(scoped_state)
  targets = select_parent_proxies(summary, parent_proxy_pool)
  return send(summary, targets)
```

**Deterministic surfaces**

- parent-proxy target selection once the candidate set and seed are fixed
- visible distinction between full claim propagation and scoped digest relay

**Policy-shaped surfaces**

- what qualifies for upward aggregation
- whether residue blocks widening or merely annotates it
- the size and composition of the parent-proxy pool

**Operator inspection**

- what summary was sent upward
- which parent-proxy peers were used
- why the claim widened or stayed local

## Trust-Weighted Merge Input Assembly

This mechanism assembles the materials a merge decision will actually reason over.

**Inputs**

- local observations
- witness records
- introducer provenance
- scope context
- freshness or epoch metadata
- prior residue

**Outputs**

- merge input bundle with explicit provenance, trust, and freshness

**Mechanics**

```text
assemble_merge_input(subject, scope):
  observations = load_observations(subject, scope)
  witnesses = load_witness_records(subject, scope)
  provenance = load_provenance(subject, scope)
  residue = load_residue(subject, scope)

  return merge_input(
    observations=observations,
    witness_records=annotate_with_trust(witnesses),
    provenance=provenance,
    freshness=derive_freshness(observations, witnesses),
    residue=residue
  )
```

**Deterministic surfaces**

- visible provenance of each input element
- preservation of freshness and trust annotations through assembly

**Policy-shaped surfaces**

- how trust annotations are computed
- how freshness is derived or bounded
- whether stale evidence is excluded or retained with down-weighting

**Operator inspection**

- what entered the merge
- what was marked stale
- which witnesses carried high weight
- what residue was already in play

## Deterministic Reunion After Partition

This mechanism governs bounded recontact after partition or serious drift.

**Inputs**

- recontact event or drift trigger
- local scope summaries
- reunion candidate sets
- healing-round seed material

**Outputs**

- deterministic rendezvous set
- exchanged summaries
- merge-ready healing input

**Mechanics**

```text
deterministic_reunion(scope_a, scope_b, heal_round):
  summaries = exchange_summaries(scope_a, scope_b)
  candidates = form_reunion_candidates(scope_a, scope_b)
  seed = make_healing_seed(scope_a, scope_b, heal_round)
  rank = permutation_rank(seed, candidates)
  rendezvous = choose_reunion_slice(rank)
  return (summaries, rendezvous)
```

**Deterministic surfaces**

- summary exchange trigger
- reunion seed and ranked rendezvous order
- tie-breaking when several reunion candidates are otherwise equally plausible

**Policy-shaped surfaces**

- reunion slice size
- how broad the reunion candidate set is
- whether some scopes get priority because of authority or trust asymmetry

**Operator inspection**

- what triggered reunion
- which summaries were exchanged
- which rendezvous peers were selected
- why those peers were selected first

## Quarantine And Hysteresis Handling

This mechanism decides when a claim or witness should be suspended from acceptance, propagation, or both.

**Inputs**

- conflicting witness records
- stale or suspect provenance
- confidence decay
- quarantine policy
- hysteresis policy

**Outputs**

- quarantined subject or witness state
- visible reason and review surface
- eventual release, escalation, or revocation

**Mechanics**

```text
update_quarantine(subject_state):
  if conflict_pressure(subject_state) > conflict_limit:
    return quarantine(subject_state, reason="conflict pressure")

  if freshness_decay(subject_state) > stale_limit and confidence_low(subject_state):
    return quarantine(subject_state, reason="stale low-confidence state")

  if hysteresis_holds(subject_state):
    return retain_current_state(subject_state)

  return release_or_reclassify(subject_state)
```

**Deterministic surfaces**

- visible transition into quarantine or revocation
- visible application of hysteresis once thresholds and timers are known

**Policy-shaped surfaces**

- conflict and staleness thresholds
- hysteresis windows
- release versus escalation policy

**Operator inspection**

- why quarantine started
- which evidence triggered it
- what hysteresis is preventing immediate reversal

## Residue-Preserving Merge Output

This mechanism produces a merge result without laundering disagreement into false certainty.

**Inputs**

- assembled merge input
- trust and freshness annotations
- merge policy

**Outputs**

- accepted or provisional state
- scoped disagreement
- quarantine
- explicit residue

**Mechanics**

```text
merge_with_residue(input):
  dominant = compare_freshness_trust_scope(input)
  unresolved = detect_unresolved_conflict(input)

  if unresolved:
    return output(
      state=dominant_or_provisional(input),
      residue=preserve(unresolved)
    )

  return output(
    state=accepted(input),
    residue=none
  )
```

**Deterministic surfaces**

- visible merge inputs and visible output classification
- deterministic tie-break once higher-order semantic distinctions are exhausted

**Policy-shaped surfaces**

- the merge calculus itself
- how much conflict is sufficient to preserve residue
- how strongly scope authority influences dominance

**Operator inspection**

- final state
- dominant evidence
- preserved disagreement
- why residue remained or disappeared

## Repair Overlay / Arborition Selection

This mechanism chooses the overlay role and path shape used for healing and anti-entropy.

**Inputs**

- current repair state
- scope boundaries
- locality and topology information
- trust boundaries
- parent-proxy availability

**Outputs**

- selected overlay role
- selected repair subtree or cross-scope path
- operator-visible reason for that path shape

**Mechanics**

```text
select_repair_overlay(repair_context):
  if repair_context.is_local_scope_drift:
    return witness_or_local_repair_subtree(repair_context)

  if repair_context.crosses_scope_boundary:
    return parent_proxy_repair_path(repair_context)

  if repair_context.partition_healing:
    return rendezvous_centered_repair_overlay(repair_context)

  return default_repair_overlay(repair_context)
```

**Deterministic surfaces**

- visible role distinction among witness, dissemination, and repair overlays
- visible selection of deterministic reunion points where those apply

**Policy-shaped surfaces**

- overlay adaptation policy
- when to escalate from local repair to cross-scope repair
- how strongly topology or trust boundaries constrain the selected path

**Operator inspection**

- which overlay role was selected
- which subtree or path carried repair
- why the overlay changed
- where parent-proxy interaction occurred

## Deterministic Surfaces vs Policy-Shaped Surfaces

The repo is intentionally trying to separate accountable determinism from deployment policy.

Surfaces that should remain deterministic enough to reconstruct:

- seed-to-rank behavior over a visible candidate set
- selected witness or rendezvous slices once policy pruning is fixed
- visible merge inputs and visible merge output class
- visible quarantine and revocation transitions

Surfaces that are intentionally policy-shaped:

- trust floors and confidence thresholds
- diversity and locality constraints for witnesses
- blast-radius and widening policy
- parent-proxy pool composition
- hysteresis windows
- overlay adaptation policy

The mechanics layer is only useful if those two categories stay distinct.

## Operator Inspection Requirements

After these mechanisms run, a serious system should let an operator inspect:

- who introduced the subject and into which scope
- which peers were eligible as witnesses
- the seed, candidate set, and final ranked selection
- why dissemination widened or stayed bounded
- what was sent upward through parent-proxy pools
- what entered a merge and what stayed as residue
- what triggered quarantine, release, or revocation
- which repair overlay carried the healing path

If those objects are hidden, the mechanics may still exist, but they are not yet meeting the repo’s standard of accountable structure.

## Non-Claims

This document is not a wire format, not a finished algorithm library, and not a claim that these mechanics have already been implemented. It is the bridge between the semantic model and a plausible future implementation discipline.
