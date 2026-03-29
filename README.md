# Resonant Membership

Resonant Membership is a design-first systems treatise on gossip, trust, and convergence under partial observability. The claim is not that cluster membership is just a faster heartbeat table; it is that once bootstrap, witness, scoped dissemination, trust, merge rules, partition healing, hierarchy, and operator visibility become first-class, membership stops being a side channel and becomes a protocol for coordinated belief under weak coordination.

This repository is organized around **Resonant Membership** as the active center of gravity. Earlier or adjacent threads, especially **ChronOS** and **SameDiff**, are preserved as archived design lineages rather than treated as co-equal active projects.

For a fast human re-entry, start with [`docs/CURRENT_STATE.md`](docs/CURRENT_STATE.md).

## Start Here

If you are new to the repo:

1. Read the spine entry with [`docs/MANIFESTO.md`](docs/MANIFESTO.md) and [`docs/ABSTRACT.md`](docs/ABSTRACT.md) for the thesis and framing.
2. Use [`docs/PAPER_MAP.md`](docs/PAPER_MAP.md) if you want the chapter roles and intended reading order.
3. Use the interstitial primitives [`docs/GLOSSARY.md`](docs/GLOSSARY.md), [`docs/PERMUTATION_RANK.md`](docs/PERMUTATION_RANK.md), [`docs/ARBORITIONS.md`](docs/ARBORITIONS.md), and [`docs/MECHANICS.md`](docs/MECHANICS.md) to keep vocabulary, primitives, and mechanics aligned while you read the spine.
4. Read the spine chapters in order: [`docs/PRIMITIVES.md`](docs/PRIMITIVES.md), [`docs/SEMANTICS.md`](docs/SEMANTICS.md), [`docs/MEMBERSHIP.md`](docs/MEMBERSHIP.md), [`docs/DISSEMINATION.md`](docs/DISSEMINATION.md), [`docs/TRUST.md`](docs/TRUST.md), [`docs/MERGE_AND_HEALING.md`](docs/MERGE_AND_HEALING.md), and [`docs/TOPOLOGY.md`](docs/TOPOLOGY.md).
5. Use the appendices and support docs [`docs/THREAT_MODEL.md`](docs/THREAT_MODEL.md), [`docs/EVALUATION.md`](docs/EVALUATION.md), [`docs/CRITIQUE.md`](docs/CRITIQUE.md), [`docs/EXAMPLES.md`](docs/EXAMPLES.md), [`docs/DIAGRAMS.md`](docs/DIAGRAMS.md), and [`docs/OPEN_QUESTIONS.md`](docs/OPEN_QUESTIONS.md) for pressure, judgment, objections, scenarios, visuals, and unresolved tensions.

## Repository Architecture

This repo follows one explicit model:

- **Active project:** Resonant Membership, at the root and under [`docs/`](docs)
- **Archived adjacent lineage:** ChronOS / `chrono flow`, under [`notes/archive/chronos/`](notes/archive/chronos)
- **Archived adjacent lineage:** SameDiff / contrast-calculus notes, under [`notes/archive/SAMEDIFF.md`](notes/archive/SAMEDIFF.md)

This is intentional. The repo should read as one active treatise with preserved neighboring lineages, not as three half-active projects competing for the same front page.

## Project Map

If you are here for a specific thread:

- **Resonant Membership:** start with [`docs/MANIFESTO.md`](docs/MANIFESTO.md), then [`docs/ABSTRACT.md`](docs/ABSTRACT.md), and then the protocol stack under [`docs/`](docs)
- **ChronOS:** start with the archived lineage at [`notes/archive/chronos/README.md`](notes/archive/chronos/README.md)
- **SameDiff:** start with the archived note at [`notes/archive/SAMEDIFF.md`](notes/archive/SAMEDIFF.md)

## Active Docs

Canonical active-doc inventory:

- **Front door:** [`README.md`](README.md)
- **Spine chapters:** [`docs/MANIFESTO.md`](docs/MANIFESTO.md), [`docs/ABSTRACT.md`](docs/ABSTRACT.md), [`docs/PRIMITIVES.md`](docs/PRIMITIVES.md), [`docs/SEMANTICS.md`](docs/SEMANTICS.md), [`docs/MEMBERSHIP.md`](docs/MEMBERSHIP.md), [`docs/DISSEMINATION.md`](docs/DISSEMINATION.md), [`docs/TRUST.md`](docs/TRUST.md), [`docs/MERGE_AND_HEALING.md`](docs/MERGE_AND_HEALING.md), [`docs/TOPOLOGY.md`](docs/TOPOLOGY.md)
- **Interstitial primitive chapters:** [`docs/GLOSSARY.md`](docs/GLOSSARY.md), [`docs/PERMUTATION_RANK.md`](docs/PERMUTATION_RANK.md), [`docs/ARBORITIONS.md`](docs/ARBORITIONS.md), [`docs/MECHANICS.md`](docs/MECHANICS.md)
- **Appendices / support docs:** [`docs/THREAT_MODEL.md`](docs/THREAT_MODEL.md), [`docs/EVALUATION.md`](docs/EVALUATION.md), [`docs/CRITIQUE.md`](docs/CRITIQUE.md), [`docs/EXAMPLES.md`](docs/EXAMPLES.md), [`docs/DIAGRAMS.md`](docs/DIAGRAMS.md), [`docs/PAPER_MAP.md`](docs/PAPER_MAP.md), [`docs/EDITORIAL_GUIDE.md`](docs/EDITORIAL_GUIDE.md), [`docs/MAINTENANCE_CHECKLIST.md`](docs/MAINTENANCE_CHECKLIST.md), [`docs/OPEN_QUESTIONS.md`](docs/OPEN_QUESTIONS.md)
- **Human snapshot:** [`docs/CURRENT_STATE.md`](docs/CURRENT_STATE.md)

Canonical reading spine:

1. [`docs/MANIFESTO.md`](docs/MANIFESTO.md)
2. [`docs/ABSTRACT.md`](docs/ABSTRACT.md)
3. [`docs/GLOSSARY.md`](docs/GLOSSARY.md)
4. [`docs/PRIMITIVES.md`](docs/PRIMITIVES.md)
5. [`docs/SEMANTICS.md`](docs/SEMANTICS.md)
6. [`docs/PERMUTATION_RANK.md`](docs/PERMUTATION_RANK.md)
7. [`docs/ARBORITIONS.md`](docs/ARBORITIONS.md)
8. [`docs/MECHANICS.md`](docs/MECHANICS.md)
9. [`docs/MEMBERSHIP.md`](docs/MEMBERSHIP.md)
10. [`docs/DISSEMINATION.md`](docs/DISSEMINATION.md)
11. [`docs/TRUST.md`](docs/TRUST.md)
12. [`docs/MERGE_AND_HEALING.md`](docs/MERGE_AND_HEALING.md)
13. [`docs/TOPOLOGY.md`](docs/TOPOLOGY.md)
14. [`docs/THREAT_MODEL.md`](docs/THREAT_MODEL.md)
15. [`docs/EVALUATION.md`](docs/EVALUATION.md)
16. [`docs/CRITIQUE.md`](docs/CRITIQUE.md)
17. [`docs/EXAMPLES.md`](docs/EXAMPLES.md)
18. [`docs/DIAGRAMS.md`](docs/DIAGRAMS.md)

## Three Theses

- **Membership is a problem of converging belief, not merely detecting liveness.**
- **Partial observability is normal, so trust, witness, and repair must be explicit.**
- **Deterministic ordering and topology matter because operators need accountable dissemination, not anonymous rumor.**

## Why This Exists

Most gossip discussions flatten the hard parts:

- who gets to introduce a node
- who is allowed to witness or dispute a claim
- how trust is accumulated, scoped, or revoked
- how partitions heal without pretending the split never happened
- how hierarchy and locality shape dissemination cost
- how an operator explains why the system believed one view and not another

Resonant Membership starts from a harder setting:

- observability is partial
- witnesses disagree
- trust is uneven
- dissemination is scoped
- topology is heterogeneous
- partitions are expected
- convergence must still be auditable

## Key Ideas

- **Permutation rank:** seeded deterministic peer ordering for accountable fanout, rendezvous, tie-breaking, and auditability
- **Witness and trust pipeline:** claims should move through introduction, corroboration, weighting, and acceptance rather than becoming truth on first contact
- **Scoped dissemination:** not every claim should flood everywhere at once; scope and audience are protocol concerns
- **Merge and healing:** membership views must reconcile after drift, omission, and partition rather than assuming a single canonical observer
- **Hierarchy and arboritions:** adaptive topology-aware dissemination, witness, and repair trees or overlay forests should reflect locality and trust structure
- **Operator observability:** the system should answer why a view converged, which witnesses mattered, and where disagreement still lives

## Anti-Goals

- not a generic gossip tutorial
- not a marketing page for a finished implementation
- not a simplistic heartbeat-only membership protocol
- not a trustless model pretending witness quality does not matter
- not a centralized control-plane paper in disguise

## Example

Illustrative protocol sketch:

```text
seed = epoch || cluster_id || subject_id
rank = permutation_rank(seed, candidate_peers)

introducer -> witnesses[rank[0..k]]:
  propose subject S in scope edge.us-west

witnesses:
  record observation
  attach trust weight and freshness
  disseminate upward only if local corroboration threshold is met

merger:
  join signed observations
  prefer higher-confidence convergent view
  preserve unresolved disagreement as visible residue
```

## Docs

- **Concept notes**
- [`docs/quorum-conditioned-observability/README.md`](docs/quorum-conditioned-observability/README.md): canonical technical note on quorum-conditioned observability and threshold ceremonies as evidence of collective presence
- **Spine chapters**
- [`docs/MANIFESTO.md`](docs/MANIFESTO.md): thesis lines and anti-goals
- [`docs/ABSTRACT.md`](docs/ABSTRACT.md): abstract and framing
- [`docs/PRIMITIVES.md`](docs/PRIMITIVES.md): compact vocabulary and fast distinctions
- [`docs/SEMANTICS.md`](docs/SEMANTICS.md): semantic contract for protocol objects and decision surfaces
- [`docs/MEMBERSHIP.md`](docs/MEMBERSHIP.md): bootstrap, witness, trust, scoped belief, lifecycle behavior
- [`docs/DISSEMINATION.md`](docs/DISSEMINATION.md): scoped fanout, digests, parent-proxy pools
- [`docs/TRUST.md`](docs/TRUST.md): trust roots, witness weighting, confidence, blast radius
- [`docs/MERGE_AND_HEALING.md`](docs/MERGE_AND_HEALING.md): merge rules, reconciliation, partition healing
- [`docs/TOPOLOGY.md`](docs/TOPOLOGY.md): hierarchy, permutation rank, and arborition overlays
- **Interstitial primitive chapters**
- [`docs/GLOSSARY.md`](docs/GLOSSARY.md): compact vocabulary index
- [`docs/PERMUTATION_RANK.md`](docs/PERMUTATION_RANK.md): deterministic ordering as a protocol primitive
- [`docs/ARBORITIONS.md`](docs/ARBORITIONS.md): adaptive overlay forests for dissemination, witness, and repair
- [`docs/MECHANICS.md`](docs/MECHANICS.md): algorithm-shaped mechanics layer bridging semantics and plausible implementation discipline
- **Appendices / support docs**
- [`docs/THREAT_MODEL.md`](docs/THREAT_MODEL.md): failures, adversaries, and abuse cases
- [`docs/EVALUATION.md`](docs/EVALUATION.md): how to judge, compare, stress, and potentially falsify the design
- [`docs/CRITIQUE.md`](docs/CRITIQUE.md): strongest internal objections and project-level failure modes
- [`docs/EXAMPLES.md`](docs/EXAMPLES.md): worked scenarios
- [`docs/DIAGRAMS.md`](docs/DIAGRAMS.md): canonical Mermaid diagrams
- [`docs/PAPER_MAP.md`](docs/PAPER_MAP.md): chapter roles and intended reading order
- [`docs/EDITORIAL_GUIDE.md`](docs/EDITORIAL_GUIDE.md): editorial source of truth for chapter roles, terms, and wording discipline
- [`docs/MAINTENANCE_CHECKLIST.md`](docs/MAINTENANCE_CHECKLIST.md): pre-acceptance checklist for doc changes
- [`docs/CURRENT_STATE.md`](docs/CURRENT_STATE.md): compact human-first snapshot of the project's current shape
- [`docs/OPEN_QUESTIONS.md`](docs/OPEN_QUESTIONS.md): maintained ledger of unresolved design questions and current leanings

## Status

This repository is a systems design document set. It describes intended primitives, invariants, and protocol shape; it should not be read as a claim that a complete runtime or implementation already exists.
