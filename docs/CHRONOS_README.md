# ChronOS

ChronOS is a proposed temporal operating system for stateful workflows, and `chrono flow` is its language layer.

The core claim is deliberately sharp: once workflow execution must preserve identity across time, survive failure, replay deterministically, expose operator intervention, and recover from evolving schemas and dependencies, the problem stops looking like "background jobs plus retries" and starts looking like an operating system for durable process-like entities.

This document is the long-form "why" of the project. The semantic contract lives in [`SPEC.md`](SPEC.md); the runtime shape lives in [`ARCHITECTURE.md`](ARCHITECTURE.md).

For the formal contract, see [`SPEC.md`](SPEC.md). For the shared vocabulary and invariants, see [`GLOSSARY.md`](GLOSSARY.md). For the language sketch, see [`LANGUAGE.md`](LANGUAGE.md). For runtime shape, see [`ARCHITECTURE.md`](ARCHITECTURE.md). For contributor-scale hard problems, see [`GOOD_FIRST_DRAGONS.md`](GOOD_FIRST_DRAGONS.md).

## Thesis

Most systems called "workflow engines" quietly depend on accidental liveness:

- in-memory state
- ad hoc retries
- implicit timers
- hidden side effects
- opaque operator intervention
- weak notions of replay

That is tolerable only while flows are short-lived and the blast radius is small.

ChronOS assumes the opposite setting:

- flows may live for months
- external systems may fail, reorder, or evolve
- humans may intervene repeatedly
- the execution model must be inspectable after the fact
- a new code version may inherit live flows started under old semantics

In that world:

- append-only history is safer than mutable in-memory truth
- projection is safer than hand-maintained "current state"
- deterministic decision logic is safer than ambient side effects
- explicit effect intent is safer than hidden I/O
- durable timers are safer than sleeps
- migration rules are safer than pretending versions do not matter

## Why "Temporal OS"

The operating-system analogy is not decorative.

ChronOS treats workflows as durable, stateful processes with a temporal execution contract. A conforming implementation would need to provide:

- identity allocation and lookup
- append-only durable history
- scheduling of waits, timers, and resumptions
- deterministic replay of decision logic
- controlled dispatch of external effects
- projection and indexing of derived state
- structured operator surfaces
- versioning and migration of live flows

This resembles OS work because the hard problems are familiar in shape:

- admission and lifecycle control
- event delivery
- scheduling under time
- isolation across effect boundaries
- crash recovery
- observability and operator tooling
- compatibility across version change

The difference is that the unit of execution is not a machine thread but a durable flow whose normative truth spans time.

## Chrono Flow

`chrono flow` is the proposed language for describing those durable flows.

Its intended character is:

- deterministic on replay
- explicit about waits, deadlines, and effects
- comfortable with long-lived state
- structured around event history rather than opaque call stacks
- able to express child workflows, joins, quorum waits, retries, cancellation, and compensation

The language is not yet frozen. See [`LANGUAGE.md`](LANGUAGE.md) for the current sketch and for a clear separation between nailed-down semantic commitments and aspirational syntax.

## History First

ChronOS keeps the strongest possible stance on history:

- the append-only event log is authoritative
- materialized state is derived
- replay reconstructs decisions from history
- explanation is expressed in terms of history, not inferred from ambient logs

This matters because current state alone is not enough to answer the questions operators actually ask:

- why is this flow waiting?
- why did this branch run?
- which effect was attempted, and under what intent?
- what changed between version `v3` and `v4`?
- what would have happened under the old retry policy?
- which operator overrode the normal path, and when?

Those are history questions.

## Identity Across Time

A flow is not "the latest row in a table" and not "the currently running coroutine." It is a durable identity that persists across:

- restarts
- waiting periods
- child execution
- retries
- migration
- operator intervention
- projection rebuilds

Stable flow IDs make lineage possible. Parent-child flow trees become auditable structures rather than best-effort conventions. See the child flow and lineage semantics in [`SPEC.md`](SPEC.md) and the lineage diagram in [`DIAGRAMS.md`](DIAGRAMS.md).

## Replay as Explanation

Replay is often treated as a debugging trick. ChronOS treats it as a primitive.

Replay serves at least four jobs:

- **recovery:** rebuild a decision state after process loss
- **audit:** explain why a flow reached its current condition
- **comparison:** evaluate the consequences of a rule, version, or policy change
- **migration:** re-interpret history under a new model with explicit version boundaries

The adjacent `SameDiff` material in [`../SAMEDIFF.md`](../SAMEDIFF.md) is relevant here. ChronOS should not merely replay a flow; it should eventually support comparative replay and replay-diff as first-class operator tools.

## Explicit Nondeterminism Boundary

ChronOS draws a hard line between deterministic decisions and external effects.

Inside the deterministic layer:

- history is read
- projections are consulted
- control decisions are produced
- next intents are derived

Across the effect boundary:

- network calls happen
- emails are sent
- tickets are created
- machines are provisioned
- operators approve or override

The effect boundary must be explicit because replay cannot safely reproduce hidden I/O. A conforming implementation would record effect intent, effect dispatch, and effect outcome as separate observable facts.

## Operator Model

Operators are not an embarrassing escape hatch. They are part of the system model.

ChronOS should treat operator actions as first-class, auditable events:

- approve
- deny
- cancel
- retry
- force-advance
- force-compensate
- annotate
- trigger replay or comparative replay

This is both a correctness requirement and an observability requirement. If a human changed the path of a live flow, that action belongs in history.

## Architecture Taste

ChronOS should feel boring in the right places:

- append-only logs rather than hidden mutable truth
- durable timers rather than process-local sleeps
- explicit versions rather than ambient semantic drift
- projections as caches that can be rebuilt
- clear lifecycle semantics rather than "best effort" recovery

And ambitious in the right places:

- deterministic replay as a foundation
- comparative replay as an explanatory surface
- parent-child flow trees as a native structure
- migration as a normal, observable operation
- observability based on structure, not log archaeology

For one proposed component breakdown, see [`ARCHITECTURE.md`](ARCHITECTURE.md).

## Anti-Goals

ChronOS is not trying to be:

- a vague "agent framework"
- a thin wrapper around queues and retries
- a magic side-effect runner that hides nondeterminism
- an event-sourced system that ignores migration
- a workflow DSL that only works if nothing changes while a flow is alive
- a benchmarking claim without a runtime to support it

ChronOS may critique weakly specified agent systems, but the criticism is narrow and technical: many so-called agents are simply long-lived workflows with poor auditability, weak replay semantics, and implicit effect boundaries.

## Example Shapes

### Human approval with auditable intervention

```chrono
flow access_request(user: UserId, role: Role) {
  state {
    approved = false
    approver: option<UserId> = none
  }

  on start {
    emit effect notify_manager(user, role)
    await operator.approve("manager") within 48h
  }

  on operator.approved(by) {
    approved = true
    approver = some(by)
    emit effect grant_role(user, role)
  }

  on timeout {
    emit effect escalate_access_request(user, role)
  }
}
```

### Comparative replay as an operator question

```text
Replay flow F-18421 under:
- original semantics at version v3
- proposed retry policy at version v4

Compare:
- effect attempts
- completion time
- timeout paths
- compensation triggers
- operator interventions
```

That comparison is not yet implemented here, but it is central to the project thesis.

## Good First Dragons

See [`GOOD_FIRST_DRAGONS.md`](GOOD_FIRST_DRAGONS.md) for a contributor-facing version of this list with concrete success criteria.

## Current Status

This repo is still in the design and documentation phase. The point of the current material is to make the semantic commitments legible before any implementation calcifies the wrong abstractions.
