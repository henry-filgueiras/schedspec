# schedspec

**A declarative scheduler policy language for runtime systems.**

Modern schedulers often entangle **mechanism** and **policy** until both become hard to reason about.  
`schedspec` explores a different split:

- express **scheduling policy declaratively** over live runtime state
- compile that policy into a **bounded, incremental runtime object**
- keep the actual **mechanism** in the host runtime or kernel

The goal is **not** to run Prolog in the kernel.  
The goal is to make scheduling policy easier to:

- design
- simulate
- explain
- profile
- verify
- evolve

...without giving up runtime discipline.

---

## The pitch

What if scheduling policy were a **legible artifact** instead of a pile of folklore?

What if the same policy description could be used to:

- replay traces offline
- compare fairness vs locality tradeoffs
- generate fast decision logic
- explain why thread X beat thread Y
- eventually target a constrained plugin/runtime interface

This project is an attempt to build that.

---

## Core idea

A policy author writes things like:

- which threads are eligible to run
- which placements are illegal
- which choices are preferred
- what fairness means
- how urgency trades off against locality
- what must be hard constraints vs soft preferences

That policy is compiled into a runtime object that receives hook events such as:

- `on_wake(thread)`
- `on_block(thread)`
- `on_tick(cpu, dt)`
- `on_affinity_change(thread)`
- `choose_next(cpu)`

The host runtime still owns the real mechanism:

- runqueues
- timers
- preemption
- context switching
- accounting
- locking
- cross-CPU coordination
- fallback safety

In other words:

 > **policy can be declarative; mechanism remains imperative.**

---

## Architecture sketch

```text
policy source
    ↓
policy IR
    ↓
compiler / lowering
    ↓
runtime policy object  <----- live state deltas / hook events
    ↓
dispatch choice / migration hints / explanation trace
    ↓
host scheduler mechanism
```

A more operational view:

```text
[declarative policy]
    relations
    constraints
    objectives
    priorities

        ↓ compile

[bounded runtime evaluator]
    incremental state
    specialized scoring
    legality filters
    bounded selection

        ↓ fed by

[host runtime / kernel]
    runnable set
    vruntime / debt / budget
    deadlines / affinity / topology
    wakeups / sleeps / ticks
```

---

## Why this is interesting

Schedulers are full of hidden law:

- fairness debt
- eligibility
- pinning
- anti-affinity
- locality
- latency sensitivity
- deadlines
- throttling
- class precedence

In production systems, those laws are often embedded in hand-entangled code and evolving heuristics.

\`schedspec\` asks whether those laws can become:

- explicit
- composable
- testable
- replayable
- comparable
- compilable

This is interesting at the intersection of:

- operating systems
- compilers
- logic / Datalog / constraints
- reactive systems
- eEPF-like runtime restrictions
- policy engines
- trace-driven performance analysis

---

## Non-goals

This project is **not** trying to:

- interpret a general-purpose logic language in a hot dispatch path
- replace scheduler mechanism with unrestricted plugins
- pretend exact global optimization is realistic on every tick
- erase the difference between relations, constraints, and decision variables
- hide concurrency and timing reality behind magical abstractions

This project is also **not** married to one implementation target.  
The same policy source might eventually support:

- offline simulation
- userspace prototypes
- generated C++/Rust evaluators
- restricted plugin/VM targets
- trace-analysis tooling

---

## Policy model

The language is currently aimed at a split between:

### 1. Relations
Finite facts and derived predicates:

- `runnable(Thread)`
- `affinity(Thread, Cpu)`
- `eligible(Thread, Cpu)`
- `must_run(Thread)`

### 2. Decision variables
Scheduler choices for the current dispatch epoch:

- `cpu_of[Thread] : option<Cpu>`
- or simpler — "pick next thread for CPU X" forms

### 3. Constraints
Hard legality conditions:

- pinning
- throttling
- one thread per CPU
- affinity correctness
- class invariants

### 4. Objectives
Soft preferences :

- urgency
- fairness rescue
- cache warmth
- locality
- anti-SMT interference
- load spreading / packing

### 5. Event-driven updates
Policy state evolves through a restricted hook surface:

- wake
- block
- tick
- migrate
- affinity change
- CPU becomes idle
- budget/deadline changes

---

## Example shape

This is illustrative pseudocode, not a frozen syntax.

```cpp
pred eligible(Thread, Cpu);

rule eligible(?t, ?c) :-
    runnable(?t),
    affinity(?t, ?c),
    not throttled(?t);

fdvar cpu_of[Thread] : option<Cpu>;

rule cpu_of[?t] = some(?c) :-
    eligible(?t, ?c);

require forall (<t in Thread, ?c in Cpu) {
    (cpu_of[?t] == some(?c)) => eligible(?t, ?c);
};

maximize urgency_score + locality_score - smt_penalty;
```

A more scheduler-flavored sketch:

```cpp
pred must_run(Thread);

rule must_run(?t) :-
    runnable(?t),
    class_of(?t) == "rt";

rule must_run(?t) :-
    runnable(?t),
    class_of(?t) == "deadline",
    deadline_slack_ns(?t) <= 0;

require all_different_except(cpu_of, none);

maximize lexicographic(
    sum (?t in Thread where must_run(?t)) {
        bool_to_int(cpu_of[?t] != none)
    },
    urgency_score + locality_score - smt_penalty,
    fair_rescue_score
);
```

The intended compilation model is not "run a theorem prover forever."  
It is more like:

- precompute legality structure
- maintain incremental derived state
- evaluate a bounded decision problem
- return a candidate quickly
- fall back safely if needed

---

## Design principles

### Policy / mechanism separation
The policy describes what counts as a good decision.  
The host owns the actual scheduling machinery.

### Bounded runtime behavior
Anything emitted from policy compilation must live within strict runtime and memory budgets.

### Incremental state maintenance
Hook events should update a compact derived model, not trigger global recomputation.

### Explainable decisions
A policy engine should be able to answer:

- why was this thread eligible?
- why did it beat that thread?
- which hard constraints ruled others out?
- which objective terms dominated?

### Multiple backends
The same policy source should be useful for:

- simulation
- trace replay
- decision codegen
- restricted runtime targets

### Seriousness over magic
If the model breaks under cross-CPU races, timer reality, or load, that matters.

---

## What exists today

Right now, this project is in the **design / architecture phase**.

Current focus:

- language shape
- semantic model
- policy IR
- hook/event model
- compilation strategy
- minimal runtime object ABI
- trace-driven prototype direction

If you are here early, that is the opportunity:
the architecture is still movable.

---

## Open quests

### Quest 0001 — Policy IR
Define the smallest useful internal representation for:

- finite relations
- decision variables
- hard constraints
- soft objectives
- event-triggered updates

### Quest 0002 — Event / delta semantics
Specify the host-to-policy hook model and what derived state may be maintained incrementally.

### Quest 0003 — Dispatch epoch model
Decide what the core selection problem is:

- single-CPU next-thread pick
- multi-CPU dispatch epoch
- short-horizon bounded lookahead

### Quest 0004 — CFS-shaped prototype
Build a trace-driven prototype of a fair-scheduling policy using this model.

### Quest 0005 — RT / deadline case studies
Express at least one RT-like and one deadline-like policy in the same language family.

### Quest 0006 — Runtime policy object ABI
Design a bounded interface for compiled policy objects:

- allowed hooks
- allowed outputs
- memory/runtime limits
- fallback behavior
- explanation hooks

### Quest 0007 — Simulator + visualizer
Replay workloads and show:

- runnable state
- chosen dispatches
- rejected candidates
- objective contributions
- divergence between policies

### Quest 0008 — Safety envelope
Define static checks and runtime guardrails for policy objects.

---

## Contribution types that would help

This project benefits from more than code.

### Architecture critiques
Tell us where the model breaks against real scheduler behavior.

### Lowering strategies
How would you compile declarative policy into fast incremental decision logic?

### Runtime safety proposals
What must be true for a bounded policy object to be trustworthy?

### Example policies
CFS-like, RT-like, deadline-like, affinity-heavy, NUMA-heavy, or weird experimental policies.

### Trace tooling
Synthetic workloads, replay harnesses, comparison tools, visualization.

### Language design
Surface syntax is still fluid. Semantic clarity matters more than syntax bikeshedding, but both are welcome.

---

## Suggested repo shape

```text
/docs
  architecture.md
  semantics.md
  event-model.md
  quests/

/examples
  fair/
  rt/
  deadline/
  toy-traces/

/sim
  replay/
  workloads/
  visualizer/

/compiler
  parser/
  ir/
  lowering/

/runtime
  abi/
  evaluator/
  fallback/

/notes
  design-log/
```

---

## FAQ

### Is this trying to replace a production kernel scheduler?
No. The first goal is to make the design space more legible and executable.

### Is this just Prolog for kernels?
No. The interesting target is a compiled, bounded runtime artifact over live state.

### Is this a solver in the hot path?
Not necessarily. A compiled policy might lower to:
- fast scoring code
- table lookups
- bounded local search
- greedy selection with legality filters
- or a hybrid approach

### Why not just write heuristics directly?
Because direct heuristics are easy to accrete and hard to compare, explain, replay, and evolve.

### Why "policy object" instead of "scheduler plugin"?
Because unrestricted plugins collapse the safety boundary. The runtime target should be constrained.

### Is the syntax final?
Not even close. Semantics first. Semantic clarity matters more than syntax bikeshedding, but both are welcome.

---

## Contribution types that would help

Open an issue if you have:

- a counterexample
- a policy sketch
- a lowering idea
- a runtime ABI proposal
- a simulator idea
- a trace format
- a case study worth encoding

Suggested labels:
- `quest`
- `good first quest`
- `language`
- `semantics`
- `compiler`
- `runtime`
- `simulation`
- `visualization`
- `scheduler-policy`
- `safety`
- `trace-replay`

A good first contribution is not necessarily small.  
A really good issue here might be a one-page demolition of a bad assumption.

---

## Status

**Exploratory. Architecture-first. Looking for sharp collaborators.**

If this idea grabs you, open an issue with one of:

- "here is where this breaks"
- "here is how I'd lower it"
- "here is a better IR"
- "here is a trace format"
- "here is a minimal hook ABI"
- "here is a scheduler policy example worth encoding"

---

## License

TBD.

---

## One-sentence summary

> `schedspec` explores whether scheduling policy can be authored declaratively and compiled into bounded runtime decision objects over live system state.