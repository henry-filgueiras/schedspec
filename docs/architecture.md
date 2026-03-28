# ChronOS Architecture

This document sketches a plausible ChronOS architecture. It is intentionally phrased in proposed terms rather than as a claim that the repository already contains a full implementation.

Its job is to explain the proposed runtime shape, not to imply a finished implementation.

For the semantic contract, see [`SPEC.md`](SPEC.md). For the shared vocabulary and invariants, see [`GLOSSARY.md`](GLOSSARY.md). For diagrams, see [`DIAGRAMS.md`](DIAGRAMS.md).

## Overview

A conforming ChronOS implementation would likely separate the system into these runtime concerns:

- flow log
- scheduler
- timer subsystem
- deterministic decision engine
- effect dispatcher
- projection subsystem
- replay engine
- migration and version manager
- operator console
- observability surface

The key architectural taste is the same throughout:

- keep history authoritative
- make nondeterminism explicit
- allow recovery through replay
- treat projections as rebuildable structure

## 1. Flow Log

The flow log is the durable append-only history store.

Responsibilities:

- persist flow events in order
- preserve stable flow identity and lineage metadata
- record version and migration boundaries
- make event streams available for replay and projection

Possible implementation strategies:

- per-flow event streams in a transactional store
- partitioned append-only log plus indexed flow views
- event-sourced storage over relational or LSM-backed primitives

The log is the system of record. If projections disagree with the log, the log wins.

## 2. Scheduler

The scheduler decides which flows or wakeups require deterministic execution next.

Responsibilities:

- receive new events and wakeups
- enqueue runnable flows
- dispatch replay or decision work
- coordinate with timer expirations and effect outcomes

This is where the temporal OS framing becomes operational. The scheduler is not merely handing out CPU slices; it is deciding which durable flow identities advance in response to time and history.

## 3. Deterministic Decision Engine

The deterministic decision engine evaluates `chrono flow` logic against history and deterministic state.

Responsibilities:

- rebuild or incrementally maintain decision state
- interpret event-driven workflow logic
- emit effect intents, child launches, waits, and terminal decisions
- guarantee replay-deterministic outcomes for the same history and version context

Possible strategies:

- interpreter over an IR
- compiled decision graph
- generated code from a typed workflow IR

This component should feel closer to a constrained runtime or state-machine evaluator than to an unconstrained general agent loop.

## 4. Durable Timer Subsystem

The timer subsystem manages durable waits, deadlines, and wakeups.

Responsibilities:

- register timers from workflow decisions
- persist timer metadata durably
- trigger wakeup events when timers fire
- support reconstruction after failure

Possible strategies:

- dedicated durable timer wheel or calendar queue
- database-backed deadline index
- hybrid approach with in-memory acceleration over durable state

The important property is semantic, not stylistic: timers must survive crashes and remain replay-compatible.

## 5. Effect Dispatcher

The effect dispatcher performs explicit external actions requested by workflow logic.

Responsibilities:

- receive effect intents
- dispatch to external systems
- track correlation and idempotency metadata
- record outcomes back into history

Effect dispatch should remain structurally separate from deterministic workflow evaluation. That separation is necessary for replay, audit, and controlled retries.

## 6. Projection Subsystem

The projection subsystem materializes query-friendly state from history.

Responsibilities:

- build status and search indices
- expose "what is this flow waiting on?" style views
- maintain parent-child lineage views
- support rebuild after projection loss

Possible strategies:

- streaming projection workers
- lazy on-demand projection
- relational materialized views backed by event ingestion

Projection is a cache over history, even if it becomes operationally important.

## 7. Replay Engine

The replay engine reconstructs decision state or compares alternate semantics over the same history.

Responsibilities:

- recovery replay for live restoration
- audit replay for explanation
- comparative replay for version or policy comparison
- divergence detection and reporting

Comparative replay is especially important. A strong ChronOS implementation should eventually answer:

- where two versions first diverge
- which invariants remain preserved
- how effect schedules or compensation paths change

That is the architectural point where ChronOS meets replay-diff and the `SameDiff` direction.

## 8. Version and Migration Manager

Long-lived flows will encounter semantic change. A version manager should make that explicit.

Responsibilities:

- track active flow version
- register migration adapters
- coordinate schema or state transforms
- preserve observable migration boundaries in history

Possible strategies:

- versioned IR and adapter library
- state transformation hooks attached to flow definitions
- projection readers capable of multi-version interpretation

## 9. Operator Console and Observability Surfaces

The operator console is the human control surface for live flows.

Responsibilities:

- inspect flow status, history, children, waits, and effects
- approve, deny, cancel, retry, or annotate
- trigger replay or comparative replay
- show migration boundaries and operator provenance

ChronOS observability should expose structure directly:

- current lifecycle state
- active waits and deadlines
- pending and failed effects
- child tree and join status
- compensation progress
- operator interventions
- replay divergence summaries

An operator console should be thought of as a view over structured workflow state, not a dashboard taped onto logs.

## Execution Shape

At a high level:

1. a flow event is appended to history
2. the scheduler marks the flow runnable
3. the deterministic engine replays or advances the flow
4. the engine emits new waits, child spawns, effect intents, or terminal decisions
5. timers and effect outcomes eventually append further events
6. projections and operator views update from the same history

## Failure and Recovery Shape

A conforming implementation should recover by leaning on durable history:

- flow process lost: replay from history
- projection lost: rebuild from history
- timer worker lost: reconstruct active timers from durable timer state and history
- operator action races with failure: serialize through history and replay

The architecture should prefer explainable recovery over clever hidden caches.

## Possible Implementation Strategies

Several implementation shapes appear plausible:

- **single-node reference runtime:** good for semantics and replay tooling
- **service-oriented control plane:** log, timer service, dispatcher, and projection workers separated
- **database-centric runtime:** relational durability and projection friendliness, potentially slower hot paths but simpler correctness
- **log-centric runtime:** stronger append semantics and event streaming, with projections layered on top

It is too early to claim one is obviously correct. The current docs aim to preserve semantic leverage regardless of storage or language choice.

## Architectural Non-Claims

This repo does not currently justify claims such as:

- a completed production runtime
- benchmark superiority
- microkernel implementation
- solved distributed consistency story

Those may become future work, but the present architecture docs should be read as a serious design proposal, not a post-hoc description of a finished system.
