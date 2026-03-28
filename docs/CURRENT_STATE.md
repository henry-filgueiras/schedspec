# Current State

This is the compact re-entry document for the active Resonant Membership project. It is meant to let a human recover the repo's current shape quickly without rereading the full treatise.

## Project Summary

Resonant Membership is a design-first systems project about how distributed systems construct, maintain, dispute, and repair a usable shared belief about membership under partial observability. Its center of gravity is not generic rumor spread. It is bootstrap, witness, trust, scoped dissemination, deterministic merge, partition healing, topology-aware structure, and operator-visible convergence discipline.

## Three Core Theses

- Membership is a problem of converging belief, not merely detecting liveness.
- Partial observability is normal, so trust, witness, and repair must be explicit.
- Deterministic ordering and topology matter because operators need accountable dissemination, not anonymous rumor.

## Canonical Primitive List

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

## Canonical Chapter Map

- Spine chapters: `MANIFESTO.md`, `ABSTRACT.md`, `PRIMITIVES.md`, `SEMANTICS.md`, `MEMBERSHIP.md`, `DISSEMINATION.md`, `TRUST.md`, `MERGE_AND_HEALING.md`, `TOPOLOGY.md`
- Interstitial primitive chapters: `GLOSSARY.md`, `PERMUTATION_RANK.md`, `ARBORITIONS.md`, `MECHANICS.md`
- Appendices / support docs: `THREAT_MODEL.md`, `EVALUATION.md`, `CRITIQUE.md`, `EXAMPLES.md`, `DIAGRAMS.md`, `PAPER_MAP.md`, `EDITORIAL_GUIDE.md`, `MAINTENANCE_CHECKLIST.md`

## What Is Currently Stable

- The active project identity and reading spine are now explicit.
- The canonical subject-state model is explicit: `unknown`, `introduced`, `witnessed`, `provisional`, `accepted`, `suspected`, `disputed`, `quarantined`, `removed`.
- `revocation` is defined as a transition event rather than a durable subject state.
- The boundary among claim body, observation, witness record, scoped belief state, and operator-visible status is now explicit.
- Witness terminology is normalized: witness as actor, observation as evidence, witness record as protocol object, witness history as collection, witness set as selected peers.
- Scope-vs-topology, merge precedence, trust-root lifecycle, and digest / summary semantics now have compact canonical boundary sections.
- The repo is explicit about being a systems design document set rather than a finished runtime.

## What Is Still Open Or Intentionally Under-Specified

- exact wire format and storage schema
- exact trust calculus
- exact candidate-set eligibility policy
- exact quarantine and hysteresis thresholds
- exact overlay adaptation algorithm
- exact digest encoding and proof attachment format
- exact merge calculus inside the allowed semantic precedence family

These are open by design. The repo now constrains the semantic space more tightly than before, but it does not pretend to have frozen one full implementation policy.

The maintained ledger for these unresolved issues lives in [`OPEN_QUESTIONS.md`](OPEN_QUESTIONS.md).

## Top 5 Unresolved Tensions

- how much residue is operationally useful before it becomes cognitive debt
- how much determinism is helpful before predictable rendezvous becomes a liability
- how much trust-root concentration is tolerable before the system drifts toward hidden authority
- how much topology adaptation is useful before overlays stop being stable debugging objects
- when summary compression is still honest and when it starts laundering disagreement into premature confidence

## Fast-Refresh Reading Order

1. `MANIFESTO.md`
2. `ABSTRACT.md`
3. `CURRENT_STATE.md`
4. `SEMANTICS.md`
5. `MEMBERSHIP.md`
6. `TRUST.md`
7. `MERGE_AND_HEALING.md`
8. `TOPOLOGY.md`
9. `EVALUATION.md`

Use `GLOSSARY.md`, `PERMUTATION_RANK.md`, `ARBORITIONS.md`, and `MECHANICS.md` as needed while refreshing those core chapters.

## Repo Status

This is a design-first repository, not a finished implementation. It describes intended semantics, primitives, invariants, and protocol shape. It should not be read as evidence that a complete runtime, network stack, or deployment-grade system already exists.
