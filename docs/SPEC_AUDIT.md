# Spec Audit

This document is a read-only cross-document audit of the active Resonant Membership docs set as it exists now.

Audit scope:

- `README.md`
- `docs/ABSTRACT.md`
- `docs/MANIFESTO.md`
- `docs/GLOSSARY.md`
- `docs/PRIMITIVES.md`
- `docs/SEMANTICS.md`
- `docs/MEMBERSHIP.md`
- `docs/DISSEMINATION.md`
- `docs/TRUST.md`
- `docs/MERGE_AND_HEALING.md`
- `docs/TOPOLOGY.md`
- `docs/THREAT_MODEL.md`
- `docs/EVALUATION.md`
- `docs/EXAMPLES.md`
- `docs/DIAGRAMS.md`
- `docs/PERMUTATION_RANK.md`
- `docs/ARBORITIONS.md`
- `docs/PAPER_MAP.md`
- `docs/CRITIQUE.md`
- `docs/MECHANICS.md`

Method:

- read the full active docs set
- record only grounded issues visible in the current text
- do not rewrite the spec in this pass

## Executive Summary

The repo is strong on framing, voice, and conceptual through-lines. It now has a real semantic backbone, a mechanics bridge, examples, diagrams, evaluation, and critique. The remaining problems are mostly spec-quality closure problems rather than vision problems.

The sharpest issues are:

- the protocol still lacks one fully canonical lifecycle vocabulary
- scope semantics and topology policy remain partially entangled
- digest and summary objects are load-bearing but still under-specified
- trust-root lifecycle semantics remain too open relative to the risks the repo itself highlights

There is also still some repetition and chapter bleed, especially at the public front door and in the relationship between support docs and the canonical paper spine.

## Severity Summary

- Critical: 0
- High: 4
- Medium: 3
- Low: 1

## Findings

### 1. Membership state vocabulary is still not fully canonical

- Severity: High
- Location:
  - `docs/MEMBERSHIP.md`
  - `docs/SEMANTICS.md`
  - `docs/DIAGRAMS.md`
  - `docs/MECHANICS.md`
- Issue type: Inconsistency, Ambiguity
- Short description:
  The repo still uses several near-overlapping vocabularies for state and outcome:
  `introduced`, `locally witnessed`, `provisionally accepted`, `widely accepted`, `suspected`, `disputed`, `quarantined`, `removed` in `MEMBERSHIP.md`; `Introduced`, `Witnessed`, `Provisional`, `Accepted`, `Disputed`, `Quarantined`, `Suspected`, `Removed` in `DIAGRAMS.md`; and output classes such as `accepted convergence`, `provisional convergence`, `scoped disagreement`, `quarantine`, and `residue` in `SEMANTICS.md` and `MECHANICS.md`.
  These are close enough to feel coherent, but not crisp enough to count as one canonical protocol state model.
- Suggested fix:
  Define one canonical lifecycle table that distinguishes:
  - durable subject states
  - merge-output classes
  - transition events such as revocation
  Then reference that table everywhere else.

### 2. Scope semantics and topology policy are still partially entangled

- Severity: High
- Location:
  - `docs/SEMANTICS.md`
  - `docs/DISSEMINATION.md`
  - `docs/TOPOLOGY.md`
  - `docs/ARBORITIONS.md`
- Issue type: Policy vs semantics confusion, Ambiguity
- Short description:
  The repo repeatedly says scope and topology are different, but still lets them bleed into one another. `SEMANTICS.md` treats scope as part of claim meaning and authority. `DISSEMINATION.md`, `TOPOLOGY.md`, and `ARBORITIONS.md` then describe path shape, hierarchy, parent-proxy pools, and overlay choice in ways that sometimes sound like they also determine claim meaning. The intended distinction is present, but not yet tight enough for a future implementation or formalization pass.
- Suggested fix:
  Add one canonical boundary section, ideally in `SEMANTICS.md`, that states:
  - what scope determines
  - what topology determines
  - what may be influenced by both
  - what must never silently migrate from policy into semantics

### 3. Digest and summary semantics are under-specified relative to their importance

- Severity: High
- Location:
  - `docs/GLOSSARY.md`
  - `docs/DISSEMINATION.md`
  - `docs/MERGE_AND_HEALING.md`
  - `docs/MECHANICS.md`
- Issue type: Gap
- Short description:
  Digests, scoped summaries, and compact summaries are central to dissemination, reunion, parent-proxy aggregation, and repair. The repo now says they matter, and `MECHANICS.md` correctly warns that summary-only reasoning is not always sufficient. But there is still no single canonical semantic contract for what a digest must preserve, what it may omit, and which decisions may not safely rely on it.
- Suggested fix:
  Add a small dedicated semantic subsection defining digest and summary objects:
  - minimum preserved fields
  - prohibited information loss
  - when digests are sufficient
  - when they may only trigger fetch, hold, or repair

### 4. Trust-root lifecycle semantics remain too open

- Severity: High
- Location:
  - `docs/TRUST.md`
  - `docs/THREAT_MODEL.md`
  - `docs/CRITIQUE.md`
  - `docs/EXAMPLES.md`
- Issue type: Gap, Policy vs semantics confusion
- Short description:
  `TRUST.md` allows trust roots to come from cryptographic identity, operator policy, deployment lineage, or previously converged witness history. That is a large semantic opening. At the same time, `THREAT_MODEL.md` and `CRITIQUE.md` warn about trust laundering, hidden authority, and soft centralization, and `EXAMPLES.md` depends on trust-sensitive witness and repair behavior. The repo raises the problem sharply but still does not define promotion, demotion, or scope-limited authority well enough to keep trust from collapsing into hidden policy.
- Suggested fix:
  Add a narrow trust-root lifecycle contract:
  - how foundational standing is introduced
  - whether it is earned, configured, or both
  - how scope limits apply
  - how demotion, repair, and revocation work
  - what must remain operator-visible

### 5. Merge precedence now exists, but its canonical home is still unstable

- Severity: Medium
- Location:
  - `docs/MERGE_AND_HEALING.md`
  - `docs/SEMANTICS.md`
  - `docs/MECHANICS.md`
  - `docs/EVALUATION.md`
- Issue type: Redundancy, Ambiguity
- Short description:
  The repo has improved here: `MERGE_AND_HEALING.md` now gives a constrained precedence family, `SEMANTICS.md` gives a merge skeleton, `MECHANICS.md` gives procedure shape, and `EVALUATION.md` names precedence stability as a metric. But the actual canonical contract is still distributed across four chapters. That makes later drift likely.
- Suggested fix:
  Decide which chapter owns the canonical merge precedence contract.
  A good split would be:
  - `SEMANTICS.md`: canonical constraints
  - `MERGE_AND_HEALING.md`: narrative explanation
  - `MECHANICS.md`: procedural sketch
  - `EVALUATION.md`: test criteria

### 6. Witness terminology is better, but still not perfectly disciplined

- Severity: Medium
- Location:
  - `docs/SEMANTICS.md`
  - `docs/DIAGRAMS.md`
  - `docs/EXAMPLES.md`
  - `docs/MECHANICS.md`
- Issue type: Terminology drift
- Short description:
  The repo has improved witness vocabulary significantly, especially in `SEMANTICS.md`, `EXAMPLES.md`, and `MECHANICS.md`. But `DIAGRAMS.md` still uses labels like `Witnessed` and `Witness record formed`, while other chapters talk about witness sets, witness histories, witness records, and witnesses as actors. The intended distinctions are mostly present, but not yet fully normalized across every surface.
- Suggested fix:
  Add one short canonical terminology block, probably in `SEMANTICS.md`, and explicitly point `DIAGRAMS.md` and `EXAMPLES.md` to it:
  - witness = actor
  - observation = local evidence
  - witness record = protocol object
  - witness history = collection of records
  - witness set = selected actors for a round

### 7. Public navigation still leaks beyond the canonical paper spine

- Severity: Medium
- Location:
  - `README.md`
  - `docs/PAPER_MAP.md`
- Issue type: Cross-link / navigation weakness
- Short description:
  The repo now has a strong `PAPER_MAP`, but the front door still depends on support documents that sit slightly outside the canonical paper spine, such as `CURRENT_STATE.md`, `OPEN_QUESTIONS.md`, `EDITORIAL_GUIDE.md`, and `MAINTENANCE_CHECKLIST.md`. That is reasonable for maintainers, but it weakens the answer to a newcomer’s question: what is the canonical note set versus the maintenance layer?
- Suggested fix:
  Keep the current support docs, but explicitly separate:
  - canonical public paper/treatise docs
  - maintainer support docs
  - quick-reentry docs
  in both `README.md` and `PAPER_MAP.md`.

### 8. Framing repetition is still high enough to risk later drift

- Severity: Low
- Location:
  - `README.md`
  - `docs/ABSTRACT.md`
  - `docs/MANIFESTO.md`
  - `docs/PRIMITIVES.md`
- Issue type: Redundancy
- Short description:
  The repo’s strongest lines still appear in several places. The prose remains good, so this is not yet a readability problem. It is a maintenance problem waiting to happen: if the framing evolves, several public-facing chapters will need synchronized updates.
- Suggested fix:
  Keep the manifesto as the sharpest statement of problem framing, the abstract as the concise summary, and the README as the navigational front door. Trim repeated thesis language elsewhere unless it is doing specific local work.

## Recommended Cleanup Order

1. Canonicalize the lifecycle vocabulary and the relation among state, transition, and merge-output classes.
2. Tighten the scope-versus-topology boundary.
3. Add a digest / summary semantic contract.
4. Add trust-root lifecycle semantics.
5. Consolidate the canonical home of merge precedence.
6. Finish witness-term normalization.
7. Separate public paper docs from support and maintenance docs in navigation.

## Closing Assessment

The repository is already well past “interesting notes” and into “real design document set.” The next stage is not more conceptual expansion. It is semantic closure in a few specific places where the project is now strong enough that ambiguity will become costly.

The main risk is not that the ideas are weak. The main risk is that several nearly-canonical versions of the same protocol surfaces are beginning to coexist. That is exactly the moment when a design-first repo needs sharper ownership of terms, states, and object contracts.
