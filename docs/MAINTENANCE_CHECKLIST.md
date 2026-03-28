# Maintenance Checklist

Use this checklist before accepting a docs change in the active Resonant Membership set.

## Pre-Acceptance Checks

- Does the edit fit an existing chapter role?
- If it changes reading order or active-doc inventory, were `README.md` and `PAPER_MAP.md` updated together?
- If it changes doc classification, do `README.md`, `PAPER_MAP.md`, `EDITORIAL_GUIDE.md`, and `AGENTS.md` all agree on `spine chapter`, `interstitial primitive chapter`, or `appendix / support doc`?
- If it adds a new major concept, was the existing docs set checked first for a better home?

## Terminology Checks

- Are canonical terms used with canonical spellings?
- Does the change preserve the canonical subject states?
- Did `witness`, `witness record`, `witness set`, and `observation` stay distinct?
- Did `scope` and `topology` stay distinct?
- If a new term appears, is it actually needed?

## Cross-Link Checks

- Does the doc link to its semantic home?
- Does it link to adjacent chapters when those boundaries matter?
- If a coined term or alias appears, is the canonical source linked?
- Does the reading spine still match the canonical order in `README.md`, `PAPER_MAP.md`, and `EDITORIAL_GUIDE.md` exactly?

## Duplicate Thesis Checks

- Did this change restate repo-wide thesis lines where a short cross-link would do?
- Did a local chapter start redoing the manifesto, abstract, or semantics chapter?
- If repeated doctrine was added, is it local consequence or just duplication?

## Ambiguity Checks

- Could a reader confuse claim content with belief state?
- Could a reader confuse policy choice with semantic requirement?
- Could a reader confuse transition events with durable states?
- Could a reader mistake an example for normative protocol text?

## Scope-Control Checks

- Did this change add conceptual scope accidentally?
- Did it introduce a new subsystem, guarantee, or attack model not demanded by the current docs?
- Did it broaden the project beyond Resonant Membership into archived lineages without saying so?

## Policy vs Semantics Checks

- Are `must`, `should`, and `may` used correctly?
- If a section is deployment-shaped, does it say so?
- If a section is semantically load-bearing, does it say so?
- Did topology, trust policy, or implementation detail silently become semantic meaning?

## Proposal vs Implementation Checks

- Does the wording stay proposal-first unless implementation evidence exists?
- Are scenarios, diagrams, and mechanics still clearly non-runtime artifacts?
- Did any sentence accidentally imply finished implementation status?

## Final Gate

- Is the edit shorter, clearer, or more canonical than what it replaced?
- Does it lower future maintenance cost instead of adding ceremony?
- Would another editor know exactly where to update the same concept next time?
