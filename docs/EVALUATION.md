# Evaluation

This document describes how Resonant Membership should be judged as a systems proposal.

It is not a results section. It does not claim benchmarks, deployments, or empirical wins that the repo cannot support. Its job is to say what evidence would count, which tradeoffs are being paid for, and what kinds of failure would seriously weaken the design.

See also:

- [`ABSTRACT.md`](ABSTRACT.md) for the framing argument
- [`SEMANTICS.md`](SEMANTICS.md) for the protocol objects and decision surfaces being evaluated
- [`THREAT_MODEL.md`](THREAT_MODEL.md) for the pressure the design is expected to survive
- [`EXAMPLES.md`](EXAMPLES.md) and [`DIAGRAMS.md`](DIAGRAMS.md) for concrete protocol situations

## What Problem This Section Solves

A design can be elegant and still be weak.

The rest of the repo argues that membership under partial observability should be treated as a protocol for coordinated belief rather than as a heartbeat table with rumor spread. That argument needs an evaluation layer. Without one, the design can remain rhetorically sharp while avoiding the harder question: what would success look like, what costs are being paid, and what evidence would show that the framing is not worth its complexity?

This section exists to keep the proposal falsifiable. It names the dimensions on which the design should be compared, stressed, and possibly rejected.

## Evaluation Dimensions

The design should be evaluated along several interacting axes:

- convergence quality under partial observability
- residue behavior under disagreement and healing
- trust and witness quality under uneven provenance
- topology sensitivity under hierarchy, locality, and partition
- operator explainability under stress
- cost in traffic, state, and cognitive burden

Success is not perfect certainty. Success is a system that remains locally useful, structurally honest, and operationally explainable when certainty is unavailable.

Failure is not merely slow propagation. Failure is a design that carries extra semantic machinery but still cannot control blast radius, cannot explain why a view converged, cannot heal partitions cleanly, or cannot preserve residue in a way operators can reason about.

## Protocol-Level Metrics

At the protocol level, a conforming evaluation would care about metrics such as:

- **convergence lag:** how long scopes take to reach stable useful belief after introductions, suspicions, revocations, or healed partitions
- **belief stability:** how often subjects oscillate between provisional, accepted, disputed, and quarantined states
- **residue volume:** how many unresolved disagreements remain visible after ordinary convergence or repair rounds
- **residue persistence:** how long residue remains before it is either resolved or explicitly accepted as durable uncertainty
- **scope-local usefulness:** whether local scopes reach useful decisions before global certainty exists
- **global overreach:** how often weak local evidence is allowed to spread farther than its confidence should justify

These metrics matter because the protocol is explicitly paying for more structure than a flat membership list. If that structure does not improve scoped usefulness, explainability, or convergence discipline, it is not earning its keep.

## Topology-Sensitive Metrics

Resonant Membership claims topology is part of the protocol rather than a transport detail. That claim should be evaluated directly.

Useful metrics include:

- **fanout locality efficiency:** whether dissemination remains near the scopes where information is most relevant
- **parent-proxy concentration:** whether bounded parent-proxy pools become overloaded or too central
- **rendezvous hotspot risk:** whether permutation-rank-based reunion repeatedly selects the same narrow peer population
- **arborition churn:** how often witness, dissemination, or repair overlays have to be rebuilt under normal cluster motion
- **overlay instability:** whether arboritions change so rapidly that they cease to be inspectable or operationally meaningful
- **repair traffic path stretch:** how inefficient repair routes become relative to the topology they are supposed to respect

If topology-aware structure produces only opaque complexity or unstable overlays, the design claim weakens substantially.

## Trust And Witness Quality Metrics

The proposal treats trust and witness quality as first-class rather than hidden policy. That means they need visible evaluation criteria.

Important metrics include:

- **witness diversity quality:** whether corroboration comes from genuinely different failure domains, operators, or observation positions
- **trust concentration:** how much the system depends on a narrow set of high-weight witnesses
- **confidence calibration:** whether accepted claims tend to have witness histories that match their published confidence
- **blast radius control:** whether low-quality or weakly trusted claims remain scoped rather than being amplified globally
- **quarantine accuracy:** whether quarantine captures genuinely suspect claims without swallowing too many legitimate introductions
- **quarantine overreach:** whether cautious trust policy unnecessarily delays useful local convergence

The interesting question is not whether a witness has a number attached to it. The question is whether trust actually shapes dissemination and acceptance in ways that improve protocol honesty.

## Merge And Healing Metrics

Partition healing is one of the sharpest claims in the repo, so it deserves its own evaluation posture.

Metrics and dimensions here include:

- **deterministic reunion quality:** whether independently participating peers choose compatible rendezvous sets often enough for reunion to remain bounded and legible
- **merge reproducibility:** whether the same merge inputs produce the same merge outcome under equivalent policy
- **repair completion lag:** how long it takes for repaired scopes to move from competing local views to a stable post-healing state
- **residue after healing:** how much unresolved disagreement remains after reunion, and whether that residue is informative rather than noisy
- **repair traffic cost:** how much bandwidth and coordination load healing rounds impose
- **healing oscillation risk:** whether repeated reunion and re-repair create cycles instead of stability

This section should be read with one blunt question in mind: does the merge-and-healing machinery actually outperform naive rumor resumption in situations where partitions matter?

## Operator-Observability Metrics

Resonant Membership makes a strong claim that operator visibility is part of correctness. That claim can also be evaluated.

Relevant dimensions include:

- **decision explainability:** whether operators can reconstruct why a subject became accepted, disputed, quarantined, or repaired
- **witness-path visibility:** whether operators can inspect which witnesses mattered and why
- **residue intelligibility:** whether unresolved disagreement is understandable enough to support action
- **scope traceability:** whether operators can see how a claim moved across scopes and where blast radius was widened or constrained
- **repair narrative quality:** whether a healed partition can be explained as a sequence of deterministic reunion, merge, and dissemination choices
- **operator override legibility:** whether human intervention is distinguishable from protocol convergence

The core criterion is not whether the system emits a lot of logs. It is whether an operator can form a trustworthy explanation without doing archaeology across unrelated artifacts.

## Attack And Abuse Stress Cases

Evaluation should include adversarial or abusive cases, even if the design does not claim Byzantine closure.

Important stress cases include:

- stale but previously high-weight witnesses continuing to influence acceptance
- low-quality witnesses laundering claims upward through weak scopes
- abuse of deterministic selection to predict or overload rendezvous targets
- arborition churn induced by targeted topology instability
- scoped equivocation where a claim is presented differently in neighboring scopes
- repair suppression that delays healing while leaving the system superficially calm

The design does not need to prevent all of these perfectly. It does need to surface them well enough that the tradeoffs remain visible and the failure is not mistaken for healthy convergence.

## Cost And Complexity Tradeoffs

This proposal is paying for structure.

Compared with heartbeat-only or flat-gossip models, it carries more semantic fields, more decision stages, more preserved residue, and more topology-aware machinery. Those costs show up as:

- additional protocol state
- higher reasoning burden for implementers
- more operator-facing concepts
- more careful policy tuning around trust, scope, and healing
- potential concentration risks around parent-proxy pools or deterministic rendezvous choices

The proposal is only worthwhile if those costs buy meaningful gains in scoped usefulness, blast-radius control, explainability, and post-partition honesty.

## Comparison Posture

The intended comparison is not against an imaginary strawman.

Simpler heartbeat-only or flat-gossip membership models often work well when trust is mostly uniform, topology is forgiving, and operators mainly need an approximate liveness picture. Resonant Membership is aimed at a harder operating regime where scope, witness quality, merge semantics, and repair structure materially change what the system can justify.

The comparison claim is therefore modest but pointed: this design family should be preferred only when the harder regime is real enough that added structure improves disciplined convergence more than it harms cost and operability. The repo does not claim universal superiority or research-literature novelty by fiat.

## What Would Falsify Or Seriously Weaken The Design

Several kinds of evidence would count strongly against the design:

- the added semantic structure does not materially improve convergence discipline over simpler membership models
- permutation rank repeatedly creates unfair or unstable hotspot behavior that outweighs its auditability benefits
- arboritions churn so rapidly that topology-aware overlays stop being meaningful operational objects
- trust weighting does not improve blast-radius control or witness quality enough to justify the added complexity
- residue grows but does not help operators make better decisions
- deterministic reunion and repair machinery impose substantial cost without noticeably improving post-partition correctness or explainability
- operators still need log archaeology to answer why the system believed one view over another

Any of those outcomes would not merely suggest tuning work. They would challenge the central claim that this protocol family is buying the right kind of structure.

## Open Questions

The design still carries real unresolved tensions:

- how much residue is operationally healthy before it becomes cognitive debt?
- how much determinism is useful before predictable rendezvous becomes a liability?
- how stable do arboritions need to be before they are meaningful debugging objects?
- how much trust concentration is tolerable before high-weight witnesses become brittle system dependencies?
- how should quarantine thresholds adapt across scopes with very different costs of false acceptance and false suspicion?
- when is local usefulness worth preserving even if global certainty remains weak for a long time?

These are not footnotes. They are the pressure points most likely to decide whether the design becomes a serious protocol family or an over-structured argument.
