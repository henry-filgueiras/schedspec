# Open Questions

This is the maintained ledger of unresolved Resonant Membership design questions. It is not a manifesto and not a speculative notebook. Its job is to keep the live tensions in one place so they can be tracked without rediscovering them across multiple chapters.

## How To Use This Ledger

- Add a question here when it materially affects semantics, operability, or evaluation.
- Update `status` when the repo develops a real leaning or closes the issue.
- Do not mark an issue `resolved` unless the relevant chapters actually say so.
- Prefer linking to canonical homes rather than re-explaining whole chapters here.

## Merge Precedence Inside The Allowed Semantic Family

- **Why it matters:** The repo now constrains merge precedence, but it still leaves real room inside that family. If the practical dominance ordering stays too loose, merge policy can still hide contradictory semantics.
- **Affected docs:** `SEMANTICS.md`, `MERGE_AND_HEALING.md`, `MECHANICS.md`, `EVALUATION.md`
- **Current leaning:** provenance admissibility, scope authority, freshness, trust weight, and corroboration quality are the only plausible dominant dimensions; deterministic ordering should remain tie-break-only.
- **What kind of evidence or argument would help resolve it:** worked merge cases that compare multiple admissible precedence orderings and show which ones preserve honest residue without causing pathological indecision.
- **Status:** leaning

## Trust-Root Promotion, Demotion, And Repair Discipline

- **Why it matters:** The repo treats trust roots as scoped and visible, but the exact line between ordinary high-weight witnesses and trust-root-like standing is still open. This is one of the main paths by which hidden authority could re-enter the design.
- **Affected docs:** `TRUST.md`, `SEMANTICS.md`, `THREAT_MODEL.md`, `CRITIQUE.md`, `EVALUATION.md`
- **Current leaning:** trust-root standing should remain strongly scoped, visible, and repairable; earned witness history may justify stronger standing only if the widening remains inspectable.
- **What kind of evidence or argument would help resolve it:** a sharper lifecycle with concrete cases showing when earned standing is acceptable, when it becomes dangerous, and how demotion and repair remain legible.
- **Status:** open

## Scope-vs-Topology Boundary Under Operational Pressure

- **Why it matters:** The semantic boundary is now named, but the harder question remains whether operators and implementations can keep scope meaning separate from topology policy when failures are real and cross-scope paths are messy.
- **Affected docs:** `SEMANTICS.md`, `TOPOLOGY.md`, `DISSEMINATION.md`, `EVALUATION.md`, `CRITIQUE.md`
- **Current leaning:** scope should own meaning and authority; topology should own path shape and eligibility. The dangerous case is silent substitution, not mere interaction.
- **What kind of evidence or argument would help resolve it:** cross-scope incident scenarios showing whether different topology choices preserve the same scoped meaning and merge admissibility boundaries.
- **Status:** leaning

## Digest / Summary Sufficiency Boundary

- **Why it matters:** The repo now says what digests must preserve and when summary-only reasoning is unsafe, but the practical sufficiency boundary is still open. Over-compression is one of the easiest ways to launder disagreement into premature confidence.
- **Affected docs:** `SEMANTICS.md`, `DISSEMINATION.md`, `MECHANICS.md`, `MERGE_AND_HEALING.md`, `EVALUATION.md`
- **Current leaning:** summaries are good for anti-entropy triggers, upward visibility, and bounded repair initiation; they are unsafe for final merge when residue, scope conflict, or trust-source repair is still live.
- **What kind of evidence or argument would help resolve it:** scenario comparisons showing which summary fields are enough to trigger correct fetch-or-hold behavior without carrying near-full state.
- **Status:** leaning

## Quarantine / Revocation / Removal In Harder Cases

- **Why it matters:** The canonical subject-state model is now explicit, but difficult edge cases remain: mixed subject and witness repair, scoped demotion without removal, and whether repeated revocation events should collapse into simpler visible operator categories.
- **Affected docs:** `SEMANTICS.md`, `MEMBERSHIP.md`, `MECHANICS.md`, `DIAGRAMS.md`
- **Current leaning:** `revocation` should stay an event rather than a durable subject state; subject-state and trust-source-state machinery should remain related but distinct.
- **What kind of evidence or argument would help resolve it:** tighter edge-case examples where witness standing degrades while subject membership remains unsettled, or where scoped removal and scoped quarantine compete.
- **Status:** open

## Residue Growth Versus Operational Usefulness

- **Why it matters:** Residue is one of the project’s clearest honesty mechanisms and one of its biggest operability risks. If it grows faster than it improves decisions, it becomes cognitive debt.
- **Affected docs:** `SEMANTICS.md`, `MERGE_AND_HEALING.md`, `CRITIQUE.md`, `EVALUATION.md`, `CURRENT_STATE.md`
- **Current leaning:** residue is justified only when the preserved disagreement materially improves later repair or operator understanding.
- **What kind of evidence or argument would help resolve it:** evaluation criteria and scenarios showing when residue changes action quality, not just explanation quality.
- **Status:** open

## Permutation-Rank Hotspot And Concentration Risk

- **Why it matters:** Permutation rank is a load-bearing primitive, but deterministic selection can still create repeated hotspot or concentration behavior if seeds and candidate policies are shaped badly.
- **Affected docs:** `PERMUTATION_RANK.md`, `TOPOLOGY.md`, `CRITIQUE.md`, `EVALUATION.md`
- **Current leaning:** deterministic ordering is worth keeping, but only if candidate policy, diversity constraints, and reranking discipline prevent rank from becoming hidden concentration machinery.
- **What kind of evidence or argument would help resolve it:** simulations or constructed examples showing when seeded ordering stays auditable without repeatedly overloading the same early-ranked peers.
- **Status:** open

## Arborition Stability Versus Adaptation Value

- **Why it matters:** Arboritions are useful only if their structure is stable enough to inspect and dynamic enough to matter. If they churn too much, they become operational theater.
- **Affected docs:** `ARBORITIONS.md`, `TOPOLOGY.md`, `CRITIQUE.md`, `EVALUATION.md`, `EXAMPLES.md`
- **Current leaning:** overlay adaptation is necessary, but the meaningful unit is a family of inspectable role overlays, not arbitrary continuous churn.
- **What kind of evidence or argument would help resolve it:** scenario and evaluation work that distinguishes healthy adaptation from instability that destroys operator understanding.
- **Status:** open

## Review Note

This ledger should be revised when:

- a question moves from `open` to `leaning`
- a chapter closes one of these issues explicitly
- a new unresolved tension becomes load-bearing enough to affect semantics or evaluation
