# ChronOS Glossary and Invariants

This document is the short path into the project vocabulary. It defines the core terms and the invariant statements the rest of the docs assume.

For the long-form thesis, see [`CHRONOS_README.md`](/Users/henry/schedspec/docs/CHRONOS_README.md). For the full semantic contract, see [`SPEC.md`](/Users/henry/schedspec/docs/SPEC.md).

## Core Terms

- **ChronOS:** a proposed temporal operating system for stateful workflows
- **chrono flow:** the proposed language layer for writing durable, deterministic, time-aware workflows
- **flow:** a durable execution identity with a stable `flow_id`
- **history:** the append-only event record for a flow; the normative source of truth
- **event:** a recorded fact in flow history, including lifecycle steps, timer firings, effect outcomes, migration boundaries, and operator actions
- **projection:** derived state, cache, or index rebuilt from history
- **decision layer:** the deterministic part of workflow execution that reads history and produces next steps
- **effect boundary:** the explicit line where deterministic workflow logic requests interaction with the outside world
- **effect intent:** a recorded request to perform an external action
- **timer:** a durable temporal obligation such as a deadline, retry wakeup, or wait timeout
- **operator action:** a human intervention recorded as part of history
- **child flow:** a flow with its own stable identity and history linked to a parent flow
- **comparative replay:** replaying the same history under alternate code, rules, or versions and inspecting divergence
- **migration:** an explicit version transition for live flows, events, state, or projections

## Invariants

These are the project's shortest serious statements.

1. **History first.** History is normative; materialized state is derived.
2. **Identity across time.** A flow remains the same flow across waits, restarts, retries, replay, and migration.
3. **Replay as primitive.** Replay is part of normal execution, recovery, and explanation.
4. **Explicit nondeterminism boundary.** External effects and human actions must cross visible boundaries and re-enter history explicitly.
5. **Durable time.** Timers, waits, and deadlines must survive failure.
6. **Operator actions are auditable.** Human intervention is part of the model, not an out-of-band exception.
7. **Migration is normal life.** Long-lived workflows must expect version change and schema evolution.
8. **Observability is structural.** Operators should query flow structure, not excavate intent from logs.

## Fast Distinctions

- **History vs state:** history is authoritative; state is a projection over history
- **Decision vs effect:** decisions must replay; effects cross the boundary into the external world
- **Replay vs rerun:** replay explains or recovers from history; rerun would perform fresh live actions
- **Compensation vs undo:** compensation is a structured response to partial external progress, not a promise of perfect reversal
- **Child flow vs function call:** child flows have durable identity, lineage, and terminal outcomes
- **Operator action vs admin patch:** operator actions belong in history and are visible to replay and audit

## Questions ChronOS Should Answer Cleanly

- Why is this flow waiting?
- Which event changed its status?
- Which effect was intended, dispatched, and observed?
- Which child is blocking completion?
- Which operator changed the path?
- Which version interpreted this decision?
- Where would comparative replay first diverge?

Those questions are the reason the vocabulary matters. If the system cannot answer them structurally, it has probably hidden too much of the execution model.
