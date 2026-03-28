# Resonant Membership Diagrams

These diagrams are canonical Mermaid sketches for the proposed system model. They are intentionally text-first so the conceptual structure stays easy to review and revise.

See also:

- [`MEMBERSHIP.md`](MEMBERSHIP.md), [`DISSEMINATION.md`](DISSEMINATION.md), and [`TRUST.md`](TRUST.md) for lifecycle and trust flow
- [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md) and [`TOPOLOGY.md`](TOPOLOGY.md) for repair and structure
- [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md), [`ARBORITIONS.md`](ARBORITIONS.md), and [`PRIMITIVES.md`](PRIMITIVES.md) for primitive semantics

## What Problem This Section Solves

The prose in this repo is trying to describe state transitions, trust flow, deterministic selection, and healing structure without collapsing them into generic gossip intuition.

These diagrams exist to keep the semantics legible. They are not decoration. They are compact arguments about what the protocol is claiming matters.

## Bootstrap Ladder

Illustrative sequence for introduction, witness, and scope-local decision:

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

State machine sketch for belief formation and repair:

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

High-level flow from introduction to weighted belief:

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

Repair path once previously separated scopes re-establish contact:

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

One possible hierarchy of scopes and local witness surfaces:

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

Minimal view of seeded deterministic ordering as a reusable primitive:

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

## Permutation-Rank Peer Selection

How one ordering can drive multiple accountable choices:

```mermaid
flowchart TB
    A["Candidate peers"] --> B["Apply seed"]
    B --> C["Permutation rank order"]
    C --> D1["First k for accountable fanout"]
    C --> D2["First m for witness set"]
    C --> D3["Top rendezvous pair for reunion"]
    C --> D4["Deterministic tie-break path"]
```

This diagram corresponds to [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md): one seeded ordering supports accountable fanout, witness selection, rendezvous, and arbitration without pretending those choices are arbitrary.

## Arborition Overlay Forest

Adaptive forest shape for dissemination and repair:

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

This diagram corresponds to [`ARBORITIONS.md`](ARBORITIONS.md): the overlay is a forest because dissemination, witness gathering, and repair do not always want the same tree.

## Repair, Witness, And Upward Aggregation Paths

Distinct but related paths for witnessing, repair, and summary propagation:

```mermaid
flowchart TB
    A["Local scope"] --> B1["Witness subtree"]
    A --> B2["Repair subtree"]
    B1 --> C["Scoped convergence summary"]
    B2 --> D["Anti-entropy / healing traffic"]
    C --> E["Upward aggregation path"]
    D --> E
    E --> F["Parent-proxy pool / higher scope"]
```

This diagram shows the interaction among witness subtrees, repair subtrees, and parent-proxy upward aggregation. It is the clearest minimal picture of why a flat fanout graph is the wrong mental model for the protocol.

## Diagram Conventions

Across the docs, the intended conventions are:

- membership is modeled as converging belief rather than a flat set
- witness and trust are explicit protocol stages
- deterministic ordering is treated as accountable selection
- topology is a first-class semantic constraint
- healing preserves residue when convergence is incomplete

## Operator Use

Operators should be able to read these diagrams as explanations of what the system ought to expose:

- the membership lifecycle diagram explains why a subject is provisional, disputed, quarantined, or accepted
- the trust pipeline explains how a claim moved from introduction to weighted belief
- the reunion and arborition diagrams explain why repair traffic followed one path rather than another

If the running system cannot surface structures that roughly correspond to these diagrams, the observability story is weaker than the protocol story.
