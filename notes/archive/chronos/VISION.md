# ChronOS Vision

ChronOS is a proposed temporal operating system for stateful workflows, and `chrono flow` is its language layer.

The core claim is deliberately sharp: once workflow execution must preserve identity across time, survive failure, replay deterministically, expose operator intervention, and recover from evolving schemas and dependencies, the problem stops looking like "background jobs plus retries" and starts looking like an operating system for durable process-like entities.

ChronOS is therefore framed less like a task runner and more like a temporal kernel:

- flows have stable identity
- history is normative
- replay is primitive
- effects cross an explicit boundary
- time is durable
- operators are part of the model
- migration is ordinary life, not an afterthought

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

## Replay As Explanation

Replay is often treated as a debugging trick. ChronOS treats it as a primitive.

Replay serves at least four jobs:

- recovery
- audit
- comparison
- migration

That comparative angle is one of the places ChronOS touched the same general terrain as the archived [`../SAMEDIFF.md`](../SAMEDIFF.md) note.
