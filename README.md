# ChronOS

ChronOS is a proposed temporal operating system for stateful workflows. The claim is not that workflows need better glue code; it is that once time, identity, replay, recovery, and external effects become first-class, workflow execution becomes an operating-systems problem. `chrono flow` is the corresponding language layer: a durable, deterministic, time-aware language for specifying long-lived flows whose history is normative and whose materialized state is derived.

## Three Theses

- **Workflow execution becomes an OS problem once time and recovery are first-class.**
- **History is normative; materialized state is a derived cache.**
- **Replay is not a debug trick. It is part of the execution model.**

## Why ChronOS Exists

Most workflow systems are comfortable while the flow is short-lived, mostly synchronous, and lightly audited. They become vague at the exact point the real problem starts:

- a workflow lives for hours, days, or months
- timers and deadlines matter
- humans intervene
- external APIs fail, retry, or change shape
- operators need to explain what happened
- schemas evolve while instances remain alive
- replay must recover meaning, not just reproduce a bug

ChronOS starts from a harder contract:

- history first
- stable identity across time
- replay as an execution primitive
- explicit nondeterminism boundaries
- migration as a normal condition of life

## Key Ideas

- **Append-only history is the source of truth.** Materialized state is a projection, cache, or index over durable events.
- **Deterministic decisions are separated from external effects.** A conforming runtime must be able to replay decision logic from history.
- **Flow IDs are durable identities.** A flow remains the same flow across retries, waits, restarts, migration, and operator intervention.
- **Timers are data, not sleep calls.** Deadlines, waits, and wakeups must survive process failure.
- **Operators act through audited events.** Manual approval, override, cancellation, and replay requests belong in history.
- **Observability is structural.** The system should explain a flow in terms of causality, state, effects, waits, children, and divergence, not force operators to reconstruct intent from logs.

## Anti-Goals

- not a toy workflow engine
- not a vague "agent framework"
- not hidden side effects wrapped in optimistic retries
- not mutable current state pretending history is optional
- not a finished implementation with claims this repo cannot support

## Example

Illustrative `chrono flow` sketch, not frozen syntax:

```chrono
flow rollout(service: ServiceId, target: Version) {
  state {
    desired = target
    approved = false
    deployed: set<Region> = {}
  }

  on start {
    emit effect create_change_ticket(service, desired)
    await operator.approve("release-manager")
    approved = true
  }

  when approved {
    for region in ["us-west", "us-east", "eu-central"] {
      child deploy_region(service, desired, region)
    }

    await quorum child deploy_region ok >= 2 within 20m
    emit effect shift_traffic(service, desired)
  }

  on child_failed deploy_region(region) {
    compensate shift_traffic(service, previous_version(service))
    emit effect page_oncall(service, region)
  }
}
```

## Repo Map

- [`README.md`](/Users/henry/schedspec/README.md): front page and navigation
- [`docs/CHRONOS_README.md`](/Users/henry/schedspec/docs/CHRONOS_README.md): long-form vision and anti-goals
- [`docs/GLOSSARY.md`](/Users/henry/schedspec/docs/GLOSSARY.md): core terms and invariants in one place
- [`docs/SPEC.md`](/Users/henry/schedspec/docs/SPEC.md): semantic contract for flows, events, replay, timers, effects, migration, and observability
- [`docs/LANGUAGE.md`](/Users/henry/schedspec/docs/LANGUAGE.md): `chrono flow` language sketch and determinism rules
- [`docs/EXAMPLES.md`](/Users/henry/schedspec/docs/EXAMPLES.md): worked examples across deployment, approvals, payments, incident response, and AI orchestration
- [`docs/GOOD_FIRST_DRAGONS.md`](/Users/henry/schedspec/docs/GOOD_FIRST_DRAGONS.md): contribution-sized hard problems that sharpen the model
- [`docs/ARCHITECTURE.md`](/Users/henry/schedspec/docs/ARCHITECTURE.md): proposed runtime components and implementation strategies
- [`docs/DIAGRAMS.md`](/Users/henry/schedspec/docs/DIAGRAMS.md): Mermaid diagrams for architecture, lifecycle, and lineage
- [`SAMEDIFF.md`](/Users/henry/schedspec/SAMEDIFF.md): adjacent thinking on replay-diff, comparative replay, and contrast as a first-class explanatory tool

## Current Status

This repository is currently design-first. The documents describe the intended model, invariants, and possible implementation directions; they should not be read as claims that a complete ChronOS runtime already exists.

## Start Here

Read [`docs/CHRONOS_README.md`](/Users/henry/schedspec/docs/CHRONOS_README.md) for the thesis, [`docs/GLOSSARY.md`](/Users/henry/schedspec/docs/GLOSSARY.md) for the core vocabulary and invariants, then [`docs/SPEC.md`](/Users/henry/schedspec/docs/SPEC.md) for the contract, followed by [`docs/LANGUAGE.md`](/Users/henry/schedspec/docs/LANGUAGE.md), [`docs/ARCHITECTURE.md`](/Users/henry/schedspec/docs/ARCHITECTURE.md), and [`docs/GOOD_FIRST_DRAGONS.md`](/Users/henry/schedspec/docs/GOOD_FIRST_DRAGONS.md).
