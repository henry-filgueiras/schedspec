# Good First Dragons

These are not "easy issues." They are the smallest hard problems that force ChronOS to become precise in the right places.

For the surrounding thesis, see [`CHRONOS_README.md`](/Users/henry/schedspec/docs/CHRONOS_README.md). For the semantic contract, see [`SPEC.md`](/Users/henry/schedspec/docs/SPEC.md). For the shared vocabulary, see [`GLOSSARY.md`](/Users/henry/schedspec/docs/GLOSSARY.md).

## 1. Event Envelope

Define the smallest useful event envelope for:

- flow lifecycle events
- timer scheduling and timer firing
- effect intent, dispatch, and outcome
- operator actions
- migration boundaries

Success looks like:

- intent, dispatch, and outcome are impossible to confuse
- replay can identify the first divergence point cleanly
- operator actions are first-class without special-case hacks

## 2. Deterministic Decision IR

Define a compact intermediate representation for the decision layer.

It should be able to express:

- event-triggered transitions
- durable waits
- effect intents
- child flow creation
- retry and compensation rules

Success looks like:

- the IR is replayable from history
- the IR does not smuggle in arbitrary side effects
- versioning and migration can target the IR rather than ad hoc runtime code

## 3. Durable Timer Model

Specify the timer subsystem contract precisely enough to test.

Questions to answer:

- what is recorded when a timer is created?
- what is recorded when it fires?
- how are duplicate firings prevented or tolerated?
- how do retries and deadlines compose?

Success looks like:

- crash recovery can reconstruct pending timers
- timer events behave deterministically under replay
- timeout behavior is visible in history

## 4. Child Flow Tree Semantics

Make parent-child execution precise.

Questions to answer:

- what lineage metadata is required?
- what can a parent wait on?
- how do joins and quorum waits observe child state?
- how does cancellation propagate?

Success looks like:

- a flow tree is auditable without log archaeology
- parent logic can reason over child terminal states deterministically
- late child completion after quorum is well-defined

## 5. Comparative Replay Output

Define the first useful output format for replay-diff.

Questions to answer:

- what counts as the first divergence?
- which invariants should be reported alongside divergence?
- how are hypothetical effects represented?
- how should migration boundaries appear in the comparison?

Success looks like:

- an operator can tell not just that two runs differ, but how
- the result is useful for version review and migration planning
- hypothetical branches are clearly marked

## 6. Projection Contract

Specify what a projection may assume and what it must never assume.

Questions to answer:

- which fields are caches only?
- which views are required for operators?
- how is projection lag represented?
- how are projections rebuilt across migration?

Success looks like:

- losing a projection never loses truth
- operator surfaces can explain current status structurally
- rebuild semantics are boring and reliable

## 7. Operator Action Model

Treat operator intervention as a proper part of the execution model.

Questions to answer:

- which operator actions are primitive?
- what provenance must be recorded?
- how do approvals, retries, force-advance, and cancellation interact with replay?
- how should authorization be represented without hiding the action itself?

Success looks like:

- operator actions are auditable and replay-visible
- permission checks do not erase history
- a human changing the flow path becomes legible structure

## 8. Migration Boundary Semantics

Define how live flows cross semantic versions.

Questions to answer:

- when does a version boundary become effective?
- how are old events viewed under new semantics?
- what is migrated eagerly versus lazily?
- what must be visible to operators?

Success looks like:

- migration is explicit, not ambient drift
- replay can explain which version interpreted each decision
- cross-version live flows remain understandable

## 9. Minimal Reference Runtime

Build the narrowest implementation that tests the model honestly.

Suggested scope:

- single-node only
- append-only flow log
- durable timer queue
- deterministic evaluator
- explicit effect adapter with stubbed external systems
- projection and replay CLI

Success looks like:

- the semantics survive contact with real persistence and failure
- the implementation is small enough to reason about
- docs can start pointing at executable artifacts instead of pure proposal

## 10. Example Suite with Expected Histories

Turn the example workflows into executable semantic fixtures.

Suggested starting set:

- deployment rollout
- approval workflow
- payment authorization plus compensation
- incident escalation
- AI tool orchestration

Success looks like:

- each example has an expected event history
- replay produces stable derived state
- comparative replay has something concrete to diff

## Contribution Bias

Strong contributions here usually do one of three things:

- sharpen an invariant
- expose a hidden ambiguity
- turn an essay claim into a testable contract

The best "first dragon" is often a one-page demolition of an unclear assumption followed by a cleaner replacement.
