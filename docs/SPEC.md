# ChronOS Specification

This document describes the semantic contract ChronOS is intended to satisfy. It is a proposal, not a claim of completed implementation.

ChronOS defines a runtime model for durable, time-aware workflows. The central entities are:

- **flow:** a durable execution identity with stable `flow_id`
- **event:** an append-only historical fact associated with a flow
- **history:** the authoritative ordered record of a flow's events
- **projection:** derived state or index materialized from history
- **timer:** a durable temporal obligation that may cause future wakeup or timeout
- **effect:** an explicit interaction with an external system or human operator
- **version:** the semantic identity of the flow definition used to interpret subsequent decisions

For the surrounding vision, see [`CHRONOS_README.md`](/Users/henry/schedspec/docs/CHRONOS_README.md). For the shared vocabulary and invariants, see [`GLOSSARY.md`](/Users/henry/schedspec/docs/GLOSSARY.md). For the language sketch, see [`LANGUAGE.md`](/Users/henry/schedspec/docs/LANGUAGE.md).

## Semantic Invariants

A conforming implementation should preserve these invariants:

1. **History is normative.** Materialized state is not authoritative over recorded history.
2. **Flow identity is durable.** A flow keeps the same identity across waits, restarts, replay, and migration.
3. **Deterministic decisions replay.** Given the same relevant history and version context, the decision layer must produce the same decision results.
4. **External effects are explicit.** The system must not rely on hidden side effects for correctness.
5. **Timers are durable.** Waits, deadlines, and wakeups must survive process failure.
6. **Operator actions are events.** Human interventions must enter history as first-class facts.
7. **Projection is rebuildable.** Losing a projection must not lose truth.
8. **Migration is explicit.** Version change for live flows must be observable and rule-governed.

## Flow Lifecycle

Each flow has:

- a stable `flow_id`
- a `flow_type`
- a current semantic `version`
- an append-only event stream
- zero or more child flows
- zero or more active waits, deadlines, or timers
- zero or more projections

An implementation may represent lifecycle states differently, but the semantic model should distinguish at least:

- `created`
- `running`
- `waiting`
- `blocked_on_children`
- `blocked_on_operator`
- `completed`
- `failed`
- `compensating`
- `cancelled`
- `terminated`

These are not merely UI labels; they are projections over history.

## Event Semantics

Events are append-only records associated with a flow and, where applicable, a parent-child lineage context.

Suggested event families:

- **lifecycle events:** flow created, started, resumed, completed, failed, cancelled
- **decision events:** branch chosen, retry scheduled, child spawned, join satisfied
- **timer events:** timer scheduled, timer fired, deadline exceeded
- **effect events:** effect requested, dispatched, acknowledged, succeeded, failed, compensated
- **operator events:** approved, denied, retried, force-advanced, cancelled, annotated
- **migration events:** flow version advanced, schema transformed, replay under alternate semantics requested

The exact event schema is still open, but the separation between intent, dispatch, and outcome should be maintained.

## State and Projection Semantics

ChronOS distinguishes:

- **history:** authoritative record
- **decision state:** deterministic state reconstructed or incrementally maintained from history
- **operator projection:** query-oriented materialization for dashboards, search, and status views

Derived state must be understood as a projection over history. A conforming implementation would allow projections to be rebuilt from history and version-aware rules.

Implications:

- "current status" is a projection
- "pending approvals" is a projection
- "children still running" is a projection
- "latest effect outcome" is a projection

Projection lag may exist in an implementation, but projection lag must not redefine truth.

## Deterministic Decision Boundary

ChronOS requires a deterministic decision layer. This layer:

- consumes history and derived deterministic state
- evaluates workflow logic
- produces new decision outputs such as waits, child launches, effect intents, or completion

This layer must exclude hidden nondeterminism such as:

- ambient wall-clock reads not derived from durable time events
- unrecorded random values
- direct network results consulted as if they were pure inputs
- mutable global process state not represented in history

A conforming runtime may provide controlled deterministic inputs such as:

- logical time derived from fired timers
- recorded operator actions
- recorded effect outcomes
- explicit version and migration context

## Effect Semantics

Effects are explicit boundary crossings from deterministic workflow logic into the external world.

The system should distinguish at least:

- effect intent was created
- effect dispatch was attempted
- effect outcome was observed
- effect result was accepted into history

This separation matters for replay, audit, and failure handling.

A conforming implementation should support:

- idempotency strategy per effect class
- correlation IDs or durable effect IDs
- retry policy that is itself observable
- compensation hooks where inverse or corrective action exists

ChronOS does not assume that every effect is perfectly reversible. Compensation is a workflow construct for handling partial external progress, not a promise of perfect undo.

## Timer, Deadline, and Wait Semantics

Timers are durable obligations. A timer or wait must survive process loss and should be represented in history and scheduler state.

ChronOS should support:

- absolute deadlines
- relative timers
- waits on child completion
- waits on operator actions
- waits on quorum conditions
- waits on effect outcomes

When a timer fires, that firing is an event. The workflow should not learn about time by reading the machine clock directly.

## Replay Model

Replay is a primitive operation that re-evaluates deterministic decisions from history.

Replay modes may include:

- **recovery replay:** rebuild live state after process loss
- **audit replay:** explain the path that produced current state
- **comparative replay:** run the same history under alternate code, rules, or versions

Replay must preserve the explicit effect boundary. During replay, a conforming implementation should not silently re-emit external effects as if they were fresh live actions. Instead it should:

- read recorded effect outcomes where appropriate
- mark hypothetical effect branches as hypothetical during comparative replay
- surface divergence explicitly

## Comparative Replay

Comparative replay is a first-class ChronOS idea, even if initially implemented in a narrow form.

Given the same flow history:

- replay under version `v_old`
- replay under version `v_new`
- compare decision points, waits, effect intents, and terminal outcomes

The output should identify:

- first divergence point
- preserved invariants
- changed decisions
- changed effect schedule
- changed compensation behavior

This is where the project intersects with replay-diff and the `SameDiff` framing in [`SAMEDIFF.md`](/Users/henry/schedspec/SAMEDIFF.md).

## Versioning and Migration

Long-lived workflows outlive code versions. ChronOS therefore treats migration as a normal condition.

A version model should include:

- flow definition version
- event schema version
- projection schema version
- migration rules or adapters

Migration may involve:

- changing state shape
- changing effect policy
- changing timer semantics
- changing join or quorum semantics
- changing child flow contracts

A conforming implementation would make migration explicit in history. It should be possible to determine:

- which version interpreted each decision
- when migration occurred
- whether replay crossed a migration boundary
- which adapters transformed state or event views

## Child Flow Semantics

Flows may spawn child flows. Child flows have their own stable identities and histories.

Required semantics:

- parent-child relationship is explicit and durable
- parent may wait on one child, all children, or a quorum
- child terminal states are visible to parent logic through history
- cancellation may propagate downward by policy
- compensation may spawn compensating children or trigger compensating effects

Parent-child execution should form an auditable flow tree, not an opaque pile of callbacks.

## Concurrency Model

ChronOS is expected to orchestrate many flows concurrently. Within a single flow, a language may express concurrent child activity or waits on multiple conditions.

The semantic contract should remain clear about:

- what ordering is guaranteed within a flow's event history
- how simultaneous external completions are serialized into history
- how joins and quorum waits observe child state
- how cancellation races are resolved
- how retries interact with parent and sibling state

An implementation may choose a particular serialization strategy, but it must make replay behavior well-defined.

## Failure Model

A conforming implementation should assume failures such as:

- process crash
- node loss
- projection loss
- timer scheduler interruption
- effect dispatcher retry storms
- partial external effect completion
- operator action during failure recovery

The semantic model should ensure:

- history remains authoritative after recovery
- timers can be reconstructed or durably resumed
- decision state can be replayed
- projections can be rebuilt
- partial effect progress is visible rather than hidden

## Operator Actions

Operators act through explicit commands that become audited events.

Examples:

- approve or deny a wait
- cancel a flow
- retry a failed effect
- override a branch
- force compensation
- annotate a flow with investigative notes
- request replay or comparative replay

Operator authority should be policy-aware. A conforming implementation may define role-based constraints, but whatever is allowed should remain visible in history.

## Observability Requirements

ChronOS rejects observability-by-log-archaeology as the primary interface.

A conforming implementation should expose structured answers to questions like:

- What is this flow waiting on?
- Which event last changed its status?
- Which child flows are blocking progress?
- Which effects are pending, failed, or compensated?
- Which operator actions altered its path?
- Which version and migration boundary apply?
- Where would comparative replay first diverge?

Logs may still exist, but logs are secondary artifacts. Structure comes first.

## Open Points

The following remain intentionally open:

- the exact event envelope and ordering metadata
- the final surface syntax of `chrono flow`
- the precise comparative replay output format
- the migration adapter API
- the storage and scheduling architecture of a first implementation

The aim of this specification is to fix the semantic center of gravity before implementation details harden around accidental constraints.
