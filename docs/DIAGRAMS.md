# Resonant Membership Diagrams

These diagrams are canonical Mermaid sketches for the proposed system model. They are intentionally text-first so the conceptual structure stays easy to review and revise.

See [`MEMBERSHIP.md`](MEMBERSHIP.md), [`DISSEMINATION.md`](DISSEMINATION.md), [`TRUST.md`](TRUST.md), [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md), [`TOPOLOGY.md`](TOPOLOGY.md), and [`PRIMITIVES.md`](PRIMITIVES.md) for the surrounding text.

## Bootstrap Ladder

```mermaid
sequenceDiagram
    participant I as Introducer
    participant W1 as Witness A
    participant W2 as Witness B
    participant S as Scope Merger

    I->>W1: introduce subject
    I->>W2: introduce subject
    W1->>S: observation + trust weight
    W2->>S: corroboration or dispute
    S->>S: provisional decision
    S-->>I: accepted / quarantined / disputed
```

## Membership Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Unknown
    Unknown --> Introduced: introducer presents subject
    Introduced --> Witnessed: local witnesses observe or challenge
    Witnessed --> Provisional: corroboration threshold met in scope
    Provisional --> Accepted: wider scope converges
    Provisional --> Disputed: conflicting witness claims
    Disputed --> Quarantined: trust too weak or conflict too strong
    Accepted --> Suspected: freshness degrades or witnesses fail
    Suspected --> Accepted: stronger witness restores confidence
    Suspected --> Removed: removal converges
    Quarantined --> Provisional: repair and re-witness succeed
    Removed --> [*]
    Accepted --> [*]
```

## Trust Pipeline

```mermaid
flowchart LR
    A["Introduction"] --> B["Candidate witnesses selected"]
    B --> C["Local observations gathered"]
    C --> D["Trust weighting applied"]
    D --> E["Scope-local decision"]
    E --> F["Disseminate as provisional or accepted"]
    E --> G["Preserve dispute or residue"]
    G --> H["Escalate for repair or operator review"]
```

## Partition Healing / Deterministic Reunion

```mermaid
flowchart TB
    A["Partitioned scope A"] --> C["Recontact detected"]
    B["Partitioned scope B"] --> C
    C --> D["Exchange membership summaries"]
    D --> E["Select rendezvous peers by permutation rank"]
    E --> F["Merge witness histories and residue"]
    F --> G["Disseminate repair decisions"]
    F --> H["Preserve unresolved conflict"]
    G --> I["Converged healed view"]
    H --> J["Operator-visible disagreement"]
```

## Topology Hierarchy

```mermaid
flowchart TB
    G["Global scope"] --> R1["Region us-west"]
    G --> R2["Region us-east"]
    R1 --> Z1["Zone us-west-1a"]
    R1 --> Z2["Zone us-west-1b"]
    R2 --> Z3["Zone us-east-1a"]
    Z1 --> N1["Rack / local witness set"]
    Z2 --> N2["Rack / local witness set"]
    Z3 --> N3["Rack / local witness set"]
```

## Permutation Rank Selection

```mermaid
flowchart LR
    A["Seed = epoch || scope || subject"] --> B["Candidate peer set"]
    B --> C["Deterministic permutation"]
    C --> D["Ranked peer order"]
    D --> E["Fanout selection"]
    D --> F["Rendezvous selection"]
    D --> G["Tie-break order"]
    D --> H["Audit trail"]
```

## Arborition Overlay Forest

```mermaid
flowchart TB
    A["Root scope / aggregation layer"] --> B1["Regional arborition A"]
    A --> B2["Regional arborition B"]
    B1 --> C1["Local witness subtree"]
    B1 --> C2["Repair subtree"]
    B2 --> C3["Local witness subtree"]
    B2 --> C4["Cross-scope repair subtree"]
    C2 --> D["Partition-healing rendezvous"]
    C4 --> D
```

## Diagram Conventions

Across the docs, the intended conventions are:

- membership is modeled as converging belief rather than a flat set
- witness and trust are explicit protocol stages
- deterministic ordering is treated as accountable selection
- topology is a first-class semantic constraint
- healing preserves residue when convergence is incomplete
