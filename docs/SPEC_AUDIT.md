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
- check for cross-document consistency, contradiction, ambiguity, and navigation quality
- record only grounded issues visible in the current text
- do not rewrite the spec in this pass

## Executive Summary

The repo has a strong center of gravity, a disciplined voice, and unusually good thesis coherence for a design-first systems project. The problems are not conceptual collapse. The problems are spec-shape problems: a few core boundaries are still unstable, some load-bearing terms do not yet have one canonical home, and the navigation stack understates part of the active spec.

Most issues cluster around one pattern: the repo often says the right thing, but says it in several partially overlapping ways. That is still survivable now. It becomes dangerous later, when examples, diagrams, mechanics, and chapter prose begin to harden into different implied protocols.

## Severity Summary

- Critical: 0
- High: 6
- Medium: 4
- Low: 1

## Findings

### 1. Active spec set and paper spine do not agree

- Severity: High
- Location: `README.md:11-16`, `README.md:104-121`, `docs/PAPER_MAP.md:7-38`, `docs/PAPER_MAP.md:40-124`
- Issue type: Inconsistency, Cross-link / navigation weakness
- Short description: The repo now clearly treats `SEMANTICS.md`, `PERMUTATION_RANK.md`, `ARBORITIONS.md`, and `MECHANICS.md` as active, load-bearing chapters, but `PAPER_MAP.md` still presents a paper spine that omits `GLOSSARY.md`, `PERMUTATION_RANK.md`, `ARBORITIONS.md`, and `MECHANICS.md`. `README.md` lists most of them, but not `MECHANICS.md` in the main docs inventory. The result is an unstable answer to a basic question: what is the canonical active spec set?
- Suggested fix: Normalize one canonical reading spine and one canonical active-doc inventory. Decide whether `GLOSSARY`, `PERMUTATION_RANK`, `ARBORITIONS`, and `MECHANICS` are appendix material, interstitial chapters, or first-class spine chapters, then make `README.md` and `PAPER_MAP.md` agree.

### 2. Membership lifecycle state vocabulary drifts across chapters

- Severity: High
- Location: `docs/MEMBERSHIP.md:47-61`, `docs/SEMANTICS.md:520-531`, `docs/DIAGRAMS.md:23-44`, `docs/MECHANICS.md:452-505`
- Issue type: Inconsistency, Terminology drift
- Short description: The repo does not currently maintain one canonical state machine. `MEMBERSHIP.md` names `unknown`, `introduced`, `locally witnessed`, `provisionally accepted`, `widely accepted`, `suspected`, `disputed`, `quarantined`, `removed`. `DIAGRAMS.md` uses `Unknown`, `Introduced`, `Witnessed`, `Provisional`, `Accepted`, `Disputed`, `Quarantined`, `Suspected`, `Removed`. `SEMANTICS.md` describes a lifecycle in verbs and output classes rather than the same explicit state list. `MECHANICS.md` talks in output classes such as accepted, provisional, scoped disagreement, quarantine, and residue. These are close, but not identical, and they are close in exactly the dangerous way that causes later drift.
- Suggested fix: Define one canonical membership state machine and treat all other renderings as projections of it. Explicitly say which labels are protocol states, which are operator summaries, and which are merge-output classes.

### 3. The relationship among quarantine, revocation, removal, suspicion, and dispute is not fixed

- Severity: High
- Location: `docs/SEMANTICS.md:340-374`, `docs/MEMBERSHIP.md:49-59`, `docs/DIAGRAMS.md:52-61`, `docs/MECHANICS.md:401-450`
- Issue type: Ambiguity, Gap
- Short description: `SEMANTICS.md` gives `quarantine` and `revocation` their own semantic section. `MEMBERSHIP.md` uses `suspected`, `disputed`, `quarantined`, and `removed` in the lifecycle. `DIAGRAMS.md` shows `Revoked or removed` in the trust pipeline, but not in the main state machine. `MECHANICS.md` has quarantine and release logic but no crisp canonical relation to removal or revocation. The repo therefore lacks a stable answer to whether revocation is a state, an event, a cause of removal, a trust-only transition, or some combination of these.
- Suggested fix: Add a canonical transition table for `suspected`, `disputed`, `quarantined`, `revoked`, and `removed`. State which are subject states, which are witness or trust-source states, and which are transition events rather than durable states.

### 4. Scope semantics and topology semantics are still partially entangled

- Severity: High
- Location: `docs/PRIMITIVES.md:83-90`, `docs/SEMANTICS.md:233-271`, `docs/TOPOLOGY.md:22-33`, `docs/TOPOLOGY.md:49-58`, `docs/ARBORITIONS.md:120-132`
- Issue type: Policy vs semantics confusion, Ambiguity
- Short description: The repo correctly says `scope` and `topology` are different, but several chapters still let them blur. `PRIMITIVES.md` says scope controls where a claim is relevant while topology shapes how it travels. `SEMANTICS.md` makes scope part of claim meaning and merge authority. `TOPOLOGY.md` then treats scopes arranged in hierarchy as topological objects and says topology helps decide what claims should mean. `ARBORITIONS.md` says overlay structure is part of policy for how belief should move. The result is a recurring blur between semantic jurisdiction and transport or coordination shape.
- Suggested fix: Add one explicit boundary section, probably in `SEMANTICS.md` or `TOPOLOGY.md`, that states: what scope determines, what topology determines, what can influence both, and which conflicts are resolved at semantic versus policy layers.

### 5. Merge precedence is central but still under-specified

- Severity: High
- Location: `docs/MERGE_AND_HEALING.md:44-51`, `docs/MERGE_AND_HEALING.md:105-116`, `docs/SEMANTICS.md:546-557`, `docs/MECHANICS.md:471-498`
- Issue type: Gap, Ambiguity
- Short description: The repo repeatedly names the decisive merge dimensions: freshness, trust weight, corroboration strength, scope authority, provenance, and residue preservation. But it never establishes even a constrained precedence family for how those interact. `MERGE_AND_HEALING.md` explicitly asks what dominates. `SEMANTICS.md` gives only a skeleton. `MECHANICS.md` compresses the real decision into `compare_freshness_trust_scope(input)`. That is too open for one of the repo’s most load-bearing semantic surfaces.
- Suggested fix: Define a minimal canonical merge precedence contract. It does not need one global calculus, but it should constrain the space: for example, which dimensions may dominate outright, which may only tie-break, and when residue is mandatory instead of optional.

### 6. Trust-root promotion and trust repair semantics are missing relative to the repo’s own threat claims

- Severity: High
- Location: `docs/TRUST.md:34-47`, `docs/TRUST.md:85-105`, `docs/THREAT_MODEL.md:74-83`, `docs/CRITIQUE.md:90-115`, `docs/EXAMPLES.md:373-394`
- Issue type: Gap, Policy vs semantics confusion
- Short description: `TRUST.md` says trust roots may come from cryptographic identity, operator policy, deployment lineage, or previously converged witness history. That is a major semantic opening. At the same time, `THREAT_MODEL.md` and `CRITIQUE.md` warn about trust laundering, hidden authority, and soft centralization, while `EXAMPLES.md` includes witness re-evaluation and trust repair. The repo therefore raises the problem sharply but does not define the admission or demotion semantics strongly enough to keep the trust model from becoming hidden policy.
- Suggested fix: Add a narrow trust-root lifecycle section: how a source becomes trust-root-like, how scope limits apply, what evidence can demote it, and whether trust-root status is semantic state or deployment policy input.

### 7. Claim semantics and belief-state semantics are still partly conflated

- Severity: Medium
- Location: `docs/SEMANTICS.md:84-121`, `docs/SEMANTICS.md:411-442`, `docs/MEMBERSHIP.md:35-45`, `docs/DIAGRAMS.md:46-62`, `docs/EXAMPLES.md:58-65`, `docs/MECHANICS.md:58-76`
- Issue type: Ambiguity, Policy vs semantics confusion
- Short description: The repo insists that claims, observations, witness records, and merged belief states are distinct, but some artifacts still collapse them. `SEMANTICS.md` keeps the distinction sharp. `MEMBERSHIP.md` sometimes speaks as if a claim and a scoped belief state are nearly interchangeable. `DIAGRAMS.md` labels the trust pipeline output as `Accepted fact in scope`, which cuts against the repo’s broader belief-state framing. `EXAMPLES.md` bundles multiple semantic layers into one claim block such as `introduced + reachable + eligible-for-local-routing`. `MECHANICS.md` uses `asserted_state="introduced"` without clarifying the canonical state taxonomy behind it.
- Suggested fix: Define a single mapping among claim body, scoped belief state, and operator-visible status. Also replace any “fact” wording that accidentally implies stronger ontology than the rest of the repo claims.

### 8. Witness terminology drifts across the docs

- Severity: Medium
- Location: `docs/SEMANTICS.md:163-196`, `docs/MERGE_AND_HEALING.md:27-37`, `docs/DIAGRAMS.md:54-57`, `docs/EXAMPLES.md:116-160`, `docs/MECHANICS.md:300-352`
- Issue type: Terminology drift
- Short description: `SEMANTICS.md` defines `witness record` as the structured protocol-visible witness contribution. Other chapters refer to `witness claims`, `witness histories`, `witnessed claim`, `witness set`, and plain `witnesses` without always saying whether they mean observers, their observations, their structured attestations, or their aggregate effect on belief. The repo’s intent is clear, but the noun discipline is not yet stable.
- Suggested fix: Choose one canonical object term for the protocol-visible witness contribution, then explicitly define the related but different terms: witness as actor, observation as evidence, witness record as protocol object, witness history as a collection, witness set as selected peers.

### 9. Digest and summary objects are load-bearing but not canonically specified

- Severity: Medium
- Location: `docs/GLOSSARY.md:21`, `docs/DISSEMINATION.md:44-55`, `docs/MERGE_AND_HEALING.md:94-101`, `docs/MECHANICS.md:254-299`
- Issue type: Gap
- Short description: `digest`, `witness digest`, `residue summary`, `compact summaries`, and `scoped digest` are used in dissemination, healing, and parent-proxy mechanics. But outside the glossary, there is no dedicated semantic object definition for what a digest must preserve, what it may omit, and when summarization is no longer semantically safe. That is a spec gap because digesting is one of the main places where explanation can quietly die.
- Suggested fix: Add a small semantic subsection for digest and summary objects: minimum preserved fields, prohibited information loss, and which decisions may not rely on summaries alone.

### 10. Chapter repetition is beginning to produce spec noise

- Severity: Medium
- Location: `README.md:36-79`, `docs/ABSTRACT.md:58-96`, `docs/MANIFESTO.md:27-67`, `docs/PRIMITIVES.md:68-100`, `docs/DISSEMINATION.md:77-95`, `docs/TRUST.md:78-95`, `docs/TOPOLOGY.md:118-131`
- Issue type: Redundancy
- Short description: Many chapters repeat the same invariants, anti-goals, and explanatory turns. The prose quality is strong enough that the repetition still reads well, but from a spec-maintenance perspective the center is spreading. Repetition is now high enough that future edits are likely to create small doctrinal splits simply because the same point must be updated in too many places.
- Suggested fix: Reduce repeat doctrine in the chapter bodies and concentrate canonical invariants in one home. Let other chapters reference the invariant or restate only the local consequence.

### 11. `subject` is a core object but is absent from the compact glossary

- Severity: Low
- Location: `docs/ABSTRACT.md:36-40`, `docs/PRIMITIVES.md:29-41`, `docs/SEMANTICS.md:43-83`, `docs/GLOSSARY.md:15-35`
- Issue type: Terminology drift, Cross-link / navigation weakness
- Short description: `subject` is treated as one of the core semantic objects in `ABSTRACT.md`, `PRIMITIVES.md`, and `SEMANTICS.md`, but it is not present in the compact glossary. Readers using `GLOSSARY.md` as the fast index therefore miss one of the most reused nouns in the repo.
- Suggested fix: Add `subject` to `GLOSSARY.md` and ensure the quick index covers all truly load-bearing semantic objects.

## Recommended Cleanup Order

1. Fix the active-doc inventory and paper spine so the repo has one clear navigational truth.
2. Canonicalize the membership state machine and the quarantine / revocation / removal transition model.
3. Tighten the semantic boundaries: scope versus topology, claim versus belief state, witness actor versus witness record.
4. Constrain merge precedence and trust-root lifecycle semantics enough that the core protocol stops depending on unwritten policy.
5. Add a compact digest / summary semantic contract.
6. Reduce repeated doctrine once the canonical homes are decided.

## Closing Assessment

The repo is already stronger on framing than on protocol closure. That is not a moral failure. It is exactly what a design-first project looks like before the semantic backbone fully hardens.

The next step should not be more prose volume. The next step should be to decide where the protocol is allowed to stay open, where it must now become canonical, and which chapter owns each of those decisions.
