# AGENTS.md

## Project identity

This repository is a design-first systems project centered on:

**Resonant Membership: Gossip, Trust, and Convergence Under Partial Observability**

The project is not “just about gossip protocols” in the narrow epidemic-broadcast sense.

Its real subject is:

> How distributed systems construct, maintain, and repair a usable shared belief about membership and system state when no node can directly inspect global truth.

The center of gravity is:
- bootstrap
- membership
- trust / witness / provenance
- scoped dissemination
- deterministic merge
- partition healing
- hierarchy and topology
- operator observability

Resonant Membership is the active center of gravity of the repo. ChronOS and SameDiff, if preserved, should be treated as archived adjacent lineages rather than co-equal active projects unless the user explicitly asks for an umbrella architecture.

## Tone

Write like a serious systems treatise with dangerous preface energy:
- technically grounded
- crisp
- structured
- memorable
- no fake implementation claims
- no startup fluff
- no academic sludge

Prefer:
- thesis lines
- invariants
- state machines
- threat models
- merge rules
- cost / tradeoff discussion
- diagrams that clarify semantics

Avoid:
- generic networking tutorial voice
- hand-wavy “eventual consistency solves it” energy
- overclaiming Byzantine guarantees without paying for them
- pretending the project is already implemented unless repo contents prove that

## Canonical framing

Strong coordination is expensive.
Perfect knowledge is usually unavailable.
Systems must still decide:
- who exists
- who belongs
- what changed
- which claims deserve belief
- whether a partition has healed
- how to merge competing realities safely

Gossip is therefore not merely message dissemination.
It is an epistemic control plane for systems that cannot afford certainty.

## Core invariants

These ideas should recur across docs:

1. Membership is a belief state, not a list.
2. Every claim has scope, provenance, and staleness.
3. Dissemination without trust weighting is noise amplification.
4. Convergence requires merge semantics, not just restored connectivity.
5. Healing must be rate-limited enough to avoid oscillation.
6. Trust should influence blast radius, not only acceptance.
7. Operator visibility is part of correctness.
8. Partition healing is negotiated reality merge.
9. Partial observability is the normal case, not an edge case.

## Vocabulary to preserve and refine

Use and normalize these terms across the repo:

- node
- claim
- observation
- witness
- membership view
- confidence
- scope
- epoch
- digest
- residue
- merge
- trust root
- quarantine
- hysteresis
- anti-entropy
- scoped fanout
- parent-proxy pool
- witness set
- deterministic reunion

### Permutation rank
This is a first-class concept.

Treat **permutation rank** as a seeded, deterministic ordering of peers or candidates used for things like:
- accountable fanout choice
- rendezvous / contact selection
- merge tie-breaking
- audit sampling
- bounded influence ordering
- deterministic arbitration under partial visibility

The seed and ranking function matter because they create:
- reproducibility
- auditability
- reduced ambiguity
- less accidental bias from local enumeration order

Document it as a serious primitive, not a throwaway trick.

### Arboritions
This is a coined / provisional term.
Treat it as a real concept worth either keeping or carefully renaming.

Intended meaning:
- topology-aware, often ephemeral dissemination / witness trees
- tree-like or forest-like structures built over a changing peer set
- used for scoped propagation, witness aggregation, repair paths, or region-aware dissemination
- not necessarily a single global spanning tree
- more like adaptive, policy-shaped “arborized” coordination overlays

If the repo would be clearer with a more explicit label, Codex may propose alternatives such as:
- witness trees
- scoped dissemination trees
- adaptive overlay forests
- repair trees
- arborized overlays

But preserve the underlying concept and cross-link any rename.

## Desired repository shape

Prefer a docs stack like:

- README.md
- docs/MANIFESTO.md
- docs/ABSTRACT.md
- docs/GLOSSARY.md
- docs/PRIMITIVES.md
- docs/SEMANTICS.md
- docs/PERMUTATION_RANK.md
- docs/ARBORITIONS.md
- docs/MECHANICS.md
- docs/MEMBERSHIP.md
- docs/DISSEMINATION.md
- docs/TRUST.md
- docs/MERGE_AND_HEALING.md
- docs/TOPOLOGY.md
- docs/THREAT_MODEL.md
- docs/EVALUATION.md
- docs/CRITIQUE.md
- docs/EXAMPLES.md
- docs/DIAGRAMS.md
- docs/PAPER_MAP.md
- docs/EDITORIAL_GUIDE.md
- docs/MAINTENANCE_CHECKLIST.md

If the repo already has overlapping files, preserve good material and normalize rather than exploding file count unnecessarily.

## Editorial control

For future documentation passes, treat these as maintenance constraints:

- `docs/EDITORIAL_GUIDE.md` is the editorial source of truth for chapter roles, canonical reading order, term spellings, modal verb use, and proposal-vs-implementation wording.
- `docs/MAINTENANCE_CHECKLIST.md` is the pre-acceptance checklist for terminology drift, cross-links, duplicate thesis, ambiguity, accidental scope growth, and policy-vs-semantics blur.

When editing docs, prefer updating those control docs if repo-wide editorial rules change instead of re-explaining the same rule ad hoc in content chapters.

## Diagram preference

Prefer editable Mermaid diagrams where practical.

Important diagrams:
- membership state machine
- bootstrap ladder
- trust / claim pipeline
- partition healing / deterministic reunion
- hierarchical topology diagram
- arborition / overlay-forest diagram
- permutation-rank selection diagram

## Working style

- inspect first
- reuse strong existing wording
- improve structure before adding more prose
- cross-link aggressively but cleanly
- show diffs before committing
- do not auto-commit or auto-push unless explicitly asked
