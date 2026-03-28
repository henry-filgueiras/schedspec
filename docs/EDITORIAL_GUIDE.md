# Editorial Guide

This is the editorial control layer for the active Resonant Membership docs set. Use it to keep future doc work consistent, short, and maintainable.

## Canonical Project Identity

Active project:

- **Resonant Membership: Gossip, Trust, and Convergence Under Partial Observability**

Canonical framing:

- membership is a belief state, not a list
- gossip is an epistemic control plane, not merely message spread
- the active center of gravity is bootstrap, witness, trust, scoped dissemination, merge, healing, topology, and operator visibility

Archived adjacent lineages:

- ChronOS
- SameDiff

Do not write as if those archived lineages are co-equal active projects unless the user explicitly asks for umbrella architecture.

## Canonical Reading Order

1. `MANIFESTO.md`
2. `ABSTRACT.md`
3. `GLOSSARY.md`
4. `PRIMITIVES.md`
5. `SEMANTICS.md`
6. `PERMUTATION_RANK.md`
7. `ARBORITIONS.md`
8. `MECHANICS.md`
9. `MEMBERSHIP.md`
10. `DISSEMINATION.md`
11. `TRUST.md`
12. `MERGE_AND_HEALING.md`
13. `TOPOLOGY.md`
14. `THREAT_MODEL.md`
15. `EVALUATION.md`
16. `CRITIQUE.md`
17. `EXAMPLES.md`
18. `DIAGRAMS.md`

If a doc changes that order or active-doc inventory, update `README.md` and `PAPER_MAP.md` together.

## Chapter Roles

- `README.md`: front door, project identity, active-doc map, status.
- `MANIFESTO.md`: dangerous preface and anti-simplification layer.
- `ABSTRACT.md`: paper-style framing and project summary.
- `GLOSSARY.md`: fast lookup index only.
- `PRIMITIVES.md`: compact vocabulary and fast distinctions.
- `SEMANTICS.md`: canonical object, state, and decision-surface contract.
- `PERMUTATION_RANK.md`: accountable deterministic ordering primitive.
- `ARBORITIONS.md`: adaptive overlay-forest primitive.
- `MECHANICS.md`: algorithm-shaped bridge from semantics to plausible implementation discipline.
- `MEMBERSHIP.md`: behavioral membership lifecycle and scoped belief.
- `DISSEMINATION.md`: propagation, blast radius, digests, parent-proxy use.
- `TRUST.md`: trust-root, witness-quality, confidence, and hysteresis behavior.
- `MERGE_AND_HEALING.md`: merge precedence, residue, reunion, repair.
- `TOPOLOGY.md`: locality, hierarchy, overlay roles, and scope-vs-topology interaction.
- `THREAT_MODEL.md`: hostile timing, abuse, and failure pressure.
- `EVALUATION.md`: metrics, falsifiability, and tradeoff judgment.
- `CRITIQUE.md`: strongest internal objections.
- `EXAMPLES.md`: worked scenarios that exercise the model under pressure.
- `DIAGRAMS.md`: canonical editable Mermaid diagrams.

If a chapter starts doing another chapter's job, move the boundary, not just the prose.

## Canonical Terms And Preferred Spellings

Prefer these exact forms:

- `subject`
- `claim`
- `observation`
- `witness`
- `witness record`
- `membership view`
- `confidence`
- `trust root`
- `scope`
- `epoch`
- `digest`
- `residue`
- `merge`
- `quarantine`
- `revocation`
- `hysteresis`
- `anti-entropy`
- `scoped fanout`
- `parent-proxy pool`
- `witness set`
- `deterministic reunion`
- `permutation rank`
- `arborition`

Canonical subject states:

- `unknown`
- `introduced`
- `witnessed`
- `provisional`
- `accepted`
- `suspected`
- `disputed`
- `quarantined`
- `removed`

Use `revocation` as a transition event, not as a separate durable subject state.

## Modal Verbs

- Use `must` for semantic requirements or invariants that a conforming design cannot violate.
- Use `should` for strong guidance, expected behavior, or preferred discipline that may remain deployment-shaped.
- Use `may` for allowed variation, optional behavior, or examples.

Do not use `must` for taste, emphasis, or mere preference.

## Requirement Labels

- **Semantic requirement**: part of the protocol meaning. If violated, the design is no longer describing the same thing.
- **Policy choice**: deployment-shaped tuning or admissible variation inside the semantic contract.
- **Open question**: unresolved issue the repo is explicitly carrying.
- **Non-claim**: statement of what the repo is not asserting.

When a section mixes these, label the boundary explicitly.

## Proposal vs Implementation Wording

Preferred forms:

- `the repo proposes`
- `a conforming design would need`
- `a plausible design may`
- `this document does not claim a finished implementation`

Avoid:

- implying a complete runtime exists unless the repo proves it
- writing examples as if they are production logs
- using implementation-flavored certainty for still-open policy

## Headings And Formatting

- Keep headings short and functional.
- Prefer one clear job per section.
- Use flat bullet lists unless sequence matters.
- Use numbered lists for lifecycles, procedures, and reading order.
- Put canonical terms in bold only when defining them, not every time they appear.
- Keep examples compact; do not let them silently become normative spec text.

## Diagram Conventions

- Prefer Mermaid.
- Diagrams should match the canonical state and term vocabulary.
- Treat diagrams as semantic companions, not decoration.
- Label transitions and nodes with protocol language, not dashboard slang.
- If a diagram simplifies the prose model, say what was simplified.

## Cross-Linking Conventions

- Every major doc should link to its semantic home and its closest neighboring chapters.
- `README.md` and `PAPER_MAP.md` must stay aligned on active-doc inventory and reading order.
- When introducing a coined or easy-to-drift term, link back to its canonical home.
- When a doc uses a term in a stricter way than the glossary entry, point to `SEMANTICS.md`.
- If a rename or alias is introduced, preserve lineage with explicit cross-links.
