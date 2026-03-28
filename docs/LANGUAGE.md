# Chrono Flow Language

`chrono flow` is the proposed language layer for ChronOS. This document sketches its surface and the determinism model behind it.

Status:

- the semantic direction is relatively stable
- the exact syntax is aspirational
- examples below are illustrative, not final grammar

This document is about language shape, not the full runtime contract. For the system contract behind the language, see [`SPEC.md`](SPEC.md). For the shared vocabulary and invariants, see [`GLOSSARY.md`](GLOSSARY.md). For worked scenarios, see [`EXAMPLES.md`](EXAMPLES.md).

## Design Goals

`chrono flow` should make the durable parts of workflow programming explicit:

- state declarations
- event-driven transitions
- waits and deadlines
- effect intent
- child flow structure
- retries, cancellation, and compensation
- operator intervention
- version boundaries

It should also make the dangerous parts difficult to smuggle in:

- hidden I/O in decision logic
- implicit wall-clock reads
- untracked randomness
- mutable shared ambient state

## Notation Conventions

The examples in this document follow a few consistent conventions:

- `emit effect name(args)` records an effect intent
- `on effect_succeeded name(result)` and `on effect_failed name(error)` observe recorded effect outcomes
- `child flow_name(args) as handle` starts a child flow with an optional handle
- `await child handle completed` waits on a specific child handle
- `await all children completed` and `await quorum children ...` describe structured waits over the child set

These forms are still illustrative, but the docs should use them consistently enough that the semantic load stays clear.

## Semantic Model

A `chrono flow` program describes how a flow:

- initializes durable state
- reacts to events
- decides when to wait
- requests effects
- spawns children
- joins on child outcomes
- compensates partial external progress
- reaches a terminal state

The language is intended to compile into a deterministic decision layer over append-only history.

## Core Surface

An illustrative flow has these elements:

- `flow`: declares a flow type
- `state`: declares durable projected state
- `on`: handles a discrete event or lifecycle trigger
- `when`: reacts to a derived condition
- `await`: suspends until a durable condition is satisfied
- `emit effect`: requests an explicit external effect
- `child`: spawns a child workflow
- `retry`: declares retry policy
- `compensate`: declares compensating action

Example:

```chrono
flow payment_capture(order: OrderId, amount: Money) {
  state {
    authorization_id: option<AuthId> = none
    capture_id: option<CaptureId> = none
    attempt_count = 0
  }

  on start {
    emit effect authorize_payment(order, amount)
  }

  on effect_succeeded authorize_payment(auth_id) {
    authorization_id = some(auth_id)
    emit effect capture_payment(auth_id, amount)
  }

  on effect_failed capture_payment(error) {
    attempt_count = attempt_count + 1
    retry effect capture_payment in 30s up to 3 times
  }

  on retry_exhausted capture_payment {
    compensate void_authorization(authorization_id)
    complete failed
  }
}
```

## State Declarations

State declarations represent durable projected state, not ephemeral locals.

Example:

```chrono
state {
  status: RolloutStatus = pending
  approved: bool = false
  deployed_regions: set<Region> = {}
  failed_regions: set<Region> = {}
}
```

State should be reconstructable from history or from deterministic replay rules. The language may eventually distinguish:

- compact mutable projection fields
- derived fields
- indexed query views

That split is still aspirational.

## `on`

`on` handles discrete events or lifecycle triggers.

Illustrative forms:

```chrono
on start { ... }
on timeout { ... }
on operator.approved(by) { ... }
on effect_succeeded authorize_payment(result) { ... }
on child_failed deploy_region(region) { ... }
```

`on` should be read as "when this event enters history or becomes visible at the deterministic boundary, run this decision block."

## `when`

`when` describes a derived-condition transition that becomes enabled when predicates over state or history become true.

Example:

```chrono
when approved and size(deployed_regions) >= 2 {
  emit effect shift_traffic(service, version)
}
```

The precise interaction between `on` and `when` is not fully nailed down yet. The intended rule is that both must remain replay-deterministic and grounded in visible history plus deterministic projection state.

## `await`

`await` expresses durable suspension. It should never mean "block this process thread and hope nothing dies."

Illustrative forms:

```chrono
await operator.approve("release-manager")
await child region_west completed
await all children completed
await quorum children completed(status == ok) >= 2 within 20m
await effect authorize_payment settled
```

An `await` should compile into:

- durable wait registration
- timer registration if bounded by time
- eventual wakeup as history advances

## Control Flow Relationship

The intended relationship between the core constructs is:

- `on` reacts to an event or lifecycle trigger entering the deterministic boundary
- `when` reacts to a derived condition becoming true over deterministic state and history
- `await` suspends progress until a durable condition is satisfied and a wakeup event returns control to the flow
- `child` introduces durable concurrent structure rather than hidden background activity
- `retry`, `compensate`, and `cancel` describe explicit control paths that should remain visible in history

In other words:

- `on` and `when` decide
- `await` yields
- `child` branches durable work
- retries, compensation, and cancellation shape failure behavior explicitly

## Timers and Deadlines

Time must be explicit and durable.

Illustrative forms:

```chrono
await operator.approve("manager") within 48h
retry in 30s up to 5 times
deadline 20m
```

A flow should not consult ambient wall-clock time directly inside decision logic. Timer firing and deadline expiry should arrive through durable scheduler events.

## Effects

Effects are explicit boundary crossings.

Illustrative form:

```chrono
emit effect send_invoice(customer, amount)
```

Important intended semantics:

- effect intent is explicit in history
- dispatch may be retried under policy
- effect outcome re-enters the flow as a recorded fact
- replay must not silently repeat live effects

Potential future syntax may allow effect policies to be attached directly:

```chrono
emit effect authorize(order, amount) idempotent by order
```

That surface is aspirational; the semantic requirement is not.

## Child Flows

Child workflows are native, not a naming convention.

Illustrative forms:

```chrono
child deploy_region(service, version, "us-west") as region_west
child invoice_customer(customer, amount) as invoice
await child region_west completed
await child invoice completed
await all children completed
```

Child flows should have:

- their own stable IDs
- explicit parent linkage
- auditable terminal outcomes
- clear cancellation and compensation interaction

## Joins and Quorum Waits

ChronOS should support multiple join patterns.

Examples:

```chrono
await all children completed
await any child completed
await quorum children completed(status == ok) >= 2 within 20m
await join approvals where approved >= 3 and denied == 0
```

The exact syntax is still open. The semantics should state clearly:

- what counts toward quorum
- what happens on timeout
- how late child completions are handled

## Retries

Retries are part of the workflow contract, not hidden middleware behavior.

Illustrative forms:

```chrono
retry in 30s
retry in exponential(30s, factor: 2, max: 30m) up to 6 times
retry effect capture_payment unless error.code in ["DECLINED", "INVALID"]
```

Retry policy should be visible in replay and operator tooling.

## Compensation

Compensation addresses partial external progress.

Illustrative forms:

```chrono
compensate authorize(order, amount)
compensate shift_traffic(service, previous_version)
```

ChronOS should support both:

- direct compensating effects
- compensating child flows

Compensation is not a promise of perfect undo. It is a structured way to respond to partial commitment.

## Cancellation

Cancellation should be explicit and observable.

Illustrative forms:

```chrono
on operator.cancelled {
  cancel children
  compensate active_effects
  complete cancelled
}
```

Open question:

- whether cancellation is modeled as a built-in control action, a library pattern, or both

## What Must Remain Deterministic

The language is intended to forbid or tightly control constructs that would break replay.

Disallowed or restricted in decision logic:

- direct socket or HTTP calls
- ambient wall-clock reads
- uncontrolled randomness
- nondeterministic iteration over unstable collections
- reads from mutable global process state

Allowed only via explicit boundaries:

- effect outcomes recorded in history
- operator actions recorded in history
- timer firings recorded in history
- version and migration metadata

The practical test is simple: if a branch decision cannot be reconstructed from durable history plus explicit version context, it does not belong in the deterministic layer.

## Versioning and Migration in the Language

Long-lived flows need version-aware semantics.

Possible future language hooks:

```chrono
version 3

migrate from 2 {
  state.approved = legacy.manual_ok
}
```

This area is aspirational, but the underlying requirement is firm: a live flow must not silently change meaning without an observable migration boundary.

## Examples

### Deployment rollout

```chrono
flow deploy_region(service: ServiceId, version: Version, region: Region) {
  on start {
    emit effect deploy(service, version, region)
    await effect deploy settled within 15m
  }

  on effect_succeeded deploy(result) {
    complete ok
  }

  on timeout {
    emit effect page_oncall(service, region)
    complete failed
  }
}
```

### Human approval

```chrono
flow approval(doc: DocId) {
  state {
    approved_by: option<UserId> = none
  }

  on start {
    await operator.approve("reviewer") within 24h
  }

  on operator.approved(by) {
    approved_by = some(by)
    complete approved
  }

  on timeout {
    emit effect escalate(doc)
  }
}
```

### AI tool orchestration

```chrono
flow investigate_incident(incident: IncidentId) {
  on start {
    child gather_logs(incident) as logs
    child classify_symptoms(incident) as triage
    child draft_runbook(incident) as runbook
    await all children completed
  }

  on all_children_completed {
    await operator.approve("incident-commander")
    emit effect publish_summary(incident)
  }
}
```

## What Is Nailed Down vs Aspirational

Relatively nailed down:

- history-first execution model
- deterministic decision boundary
- explicit effects
- durable timers and waits
- child flows as first-class structure
- migration and replay as core requirements

Still aspirational:

- concrete parser and grammar
- final syntax for joins, retries, and compensation
- type system details
- module/import structure
- standard library surface

The goal of the current language sketch is to make the semantic load visible early, not to pretend the syntax is settled.
