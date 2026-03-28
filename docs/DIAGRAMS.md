# Resonant Membership Diagrams

These diagrams are the canonical editable Mermaid sketches for the proposed system model. They are intentionally text-first so the conceptual structure stays reviewable, revisable, and consistent with the rest of the docs.

See also:

- [`MEMBERSHIP.md`](MEMBERSHIP.md), [`DISSEMINATION.md`](DISSEMINATION.md), and [`TRUST.md`](TRUST.md) for lifecycle and trust semantics
- [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md) and [`TOPOLOGY.md`](TOPOLOGY.md) for repair and structure
- [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) and [`ARBORITIONS.md`](ARBORITIONS.md) for the two most distinctive protocol primitives
- [`EXAMPLES.md`](EXAMPLES.md) for worked scenarios that exercise these diagrams under pressure

## What Problem This Section Solves

The prose in this repo is trying to describe state transitions, trust flow, deterministic selection, and healing structure without collapsing them into generic gossip intuition.

These diagrams exist to keep the semantics legible. They are not decoration. They are compact arguments about what the protocol is claiming matters.

## 1. Membership Lifecycle State Machine

This state machine shows membership as a belief process rather than a liveness table. A subject moves through introduction, witness formation, scoped acceptance, dispute, quarantine, and repair.

Design invariant:
Membership is a structured belief state, not a binary alive-or-dead fact.

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

## 2. Trust Pipeline

This diagram makes the trust story sequential. Observations do not become accepted fact immediately. They move through claim formation, witness corroboration, trust weighting, and state transition, with room for staleness and revocation.

Design invariant:
Trust and witness quality must shape belief formation before broad propagation occurs.

```mermaid
flowchart LR
    O["Observation"] --> C["Claim"]
    C --> W["Witnessed claim"]
    W --> D["Trust-weighted decision"]
    D --> A["Accepted fact in scope"]
    D --> Q["Quarantined or disputed state"]
    A --> S["Stale"]
    S --> A["Fresh corroboration restores confidence"]
    S --> R["Revoked or removed"]
```

## 3. Partition Healing And Deterministic Reunion

This diagram shows healing as an explicit protocol phase rather than as "rumor resumes." Recontact triggers summary exchange, deterministic rendezvous selection, merge, repair dissemination, and possibly visible unresolved conflict.

Design invariant:
Restored connectivity is not convergence; healing must preserve provenance and residue.

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

## 4. Topology Hierarchy With Parent-Proxy Pools

This diagram shows that scopes are hierarchical and that upward visibility should often pass through bounded parent-proxy pools rather than unconstrained mesh contact.

Design invariant:
Hierarchy and locality are protocol constraints, not transport afterthoughts.

```mermaid
flowchart TB
    G["Global scope"] --> GP1["Global parent-proxy pool"]
    GP1 --> R1["Region us-west"]
    GP1 --> R2["Region us-east"]
    R1 --> RP1["Regional parent-proxy pool"]
    R2 --> RP2["Regional parent-proxy pool"]
    RP1 --> Z1["Zone us-west-1a"]
    RP1 --> Z2["Zone us-west-1b"]
    RP2 --> Z3["Zone us-east-1a"]
    Z1 --> N1["Rack / local witness set"]
    Z2 --> N2["Rack / local witness set"]
    Z3 --> N3["Rack / local witness set"]
```

## 5. Permutation-Rank-Based Peer Selection

This diagram shows how one seeded ordering can drive multiple accountable choices: fanout, witness selection, rendezvous, and tie-breaking.

Design invariant:
Selection should be reproducible and auditable rather than dependent on host-local enumeration order.

```mermaid
flowchart TB
    A["Seed = epoch || scope || subject"] --> B["Candidate peer set"]
    B --> C["Deterministic permutation"]
    C --> D["Permutation rank order"]
    D --> E1["First k for accountable fanout"]
    D --> E2["First m for witness set"]
    D --> E3["Top rendezvous peers for reunion"]
    D --> E4["Deterministic tie-break path"]
    D --> E5["Audit trail for explanation"]
```

## 6. Arborition Overlay Forest

This diagram shows why the repo uses the term `arborition`: dissemination, witness, and repair do not always want the same tree, so the protocol should model a forest of related overlays instead of one flat graph.

Design invariant:
Propagation, witness gathering, and repair should be explicit overlay roles, not one undifferentiated fanout path.

```mermaid
flowchart TB
    A["Root scope / aggregation layer"] --> B1["Regional arborition A"]
    A --> B2["Regional arborition B"]
    B1 --> C1["Local witness subtree"]
    B1 --> C2["Dissemination subtree"]
    B1 --> C3["Repair subtree"]
    B2 --> D1["Local witness subtree"]
    B2 --> D2["Dissemination subtree"]
    B2 --> D3["Cross-scope repair subtree"]
    C3 --> E["Partition-healing rendezvous"]
    D3 --> E
```

## 7. Repair Subtree, Witness Subtree, And Upward Aggregation

This final diagram isolates the most operationally useful overlay distinction: one subtree gathers witnesses, one carries repair traffic, and one carries scoped summaries upward through parent-proxy pools.

Design invariant:
A flat fanout graph hides the difference between witness collection, repair traffic, and bounded upward visibility.

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
- the trust pipeline explains how an observation became a claim and then either converged, went stale, or was revoked
- the reunion and repair diagrams explain why healing followed one path rather than another
- the topology and arborition diagrams explain why propagation was scoped and structured rather than flat

If the running system cannot surface structures that roughly correspond to these diagrams, the observability story is weaker than the protocol story.
