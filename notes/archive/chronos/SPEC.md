# ChronOS Spec Notes

This is a compact archive of the core ChronOS semantic contract.

Key entities:

- **flow:** a durable execution identity with stable `flow_id`
- **event:** an append-only historical fact associated with a flow
- **history:** the authoritative ordered record of a flow's events
- **projection:** derived state or index materialized from history
- **timer:** a durable temporal obligation that may cause future wakeup or timeout
- **effect:** an explicit interaction with an external system or human operator
- **version:** the semantic identity of the flow definition used to interpret subsequent decisions

Core invariants:

1. history is normative
2. flow identity is durable
3. deterministic decisions replay
4. external effects are explicit
5. timers are durable
6. operator actions are first-class events
7. projections are rebuildable
8. migration is explicit

The strongest ChronOS claim was that replay is not merely for debugging and recovery. It is part of the execution model and part of explanation.
