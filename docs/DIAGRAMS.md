# ChronOS Diagrams

These diagrams are text-first and intended to stay easy to review and diff. They are conceptual diagrams for the proposed system model.

See [`ARCHITECTURE.md`](/Users/henry/schedspec/docs/ARCHITECTURE.md) for the surrounding architectural narrative, [`SPEC.md`](/Users/henry/schedspec/docs/SPEC.md) for the semantic contract, and [`GLOSSARY.md`](/Users/henry/schedspec/docs/GLOSSARY.md) for the shared vocabulary.

## Layered Architecture

```mermaid
flowchart TB
    A["chrono flow source"] --> B["typed workflow IR"]
    B --> C["deterministic decision engine"]

    subgraph Runtime["ChronOS runtime"]
        C --> D["append-only flow log"]
        C --> E["durable timer subsystem"]
        C --> F["effect intent stream"]
        C --> G["child flow manager"]
    end

    D --> H["projection subsystem"]
    D --> I["replay engine"]
    D --> J["migration/version manager"]

    F --> K["effect dispatcher"]
    K --> L["external systems / humans"]
    L --> M["effect outcomes / operator actions"]
    M --> D

    E --> N["timer fired events"]
    N --> D

    H --> O["operator console"]
    I --> O
    J --> O
```

Reading notes:

- history remains central
- timers and effects feed history, not hidden side channels
- replay, migration, and operator visibility all anchor on the same durable log

## Workflow Lifecycle with Compensation Paths

```mermaid
stateDiagram-v2
    [*] --> Created
    Created --> Running: start
    Running --> Waiting: await timer/operator/effect/child
    Waiting --> Running: wakeup event
    Running --> EffectsPending: emit effect intent
    EffectsPending --> Running: effect succeeded
    EffectsPending --> Retrying: effect failed, retry policy applies
    Retrying --> Waiting: durable retry timer scheduled
    Retrying --> Compensating: retries exhausted or downstream failure
    Running --> BlockedOnChildren: child flows started
    BlockedOnChildren --> Running: join or quorum satisfied
    Running --> Completed: terminal success
    Running --> Failed: terminal failure
    Failed --> Compensating: compensation policy exists
    Compensating --> Cancelled: compensation ended by operator cancellation
    Compensating --> Completed: compensation restores acceptable terminal state
    Compensating --> Failed: compensation itself fails
    Running --> Cancelled: operator cancel or policy cancel
    Completed --> [*]
    Failed --> [*]
    Cancelled --> [*]
```

Reading notes:

- waiting is durable
- retries are explicit lifecycle steps
- compensation is a normal path, not an exception hidden in logs

## Lineage and Intellectual Roots

```mermaid
flowchart TD
    A["kernel / scheduler thinking"] --> X["ChronOS"]
    B["append-only logs + schema evolution"] --> X
    C["daemon / agent orchestration over durable history"] --> X
    D["SameDiff / replay-diff / branch comparison"] --> X

    X --> Y["temporal operating system for stateful workflows"]
    X --> Z["chrono flow: deterministic, time-aware workflow language"]

    A --> A1["admission, scheduling, lifecycle, recovery"]
    B --> B1["history first, projections as caches, migration"]
    C --> C1["long-lived orchestration, operator control, durable identity"]
    D --> D1["comparative replay, divergence explanation, contrast as structure"]
```

Reading notes:

- ChronOS is not reducible to one tradition
- the project lives at the intersection of temporal scheduling, durable history, orchestration, and replay-diff

## Diagram Conventions

Across the docs, the intended conventions are:

- **history is central** rather than peripheral
- **effects are explicit boundary crossings**
- **timers and operator actions re-enter through history**
- **child flows are lineage, not hidden subroutines**
- **replay is explanatory infrastructure**
