# Examples

These are Resonant Membership scenarios meant to make the abstractions concrete without pretending the protocol is fully implemented.

## 1. Bootstrapping A New Rack

- a new rack-local subject is introduced by a zone trust root
- local witnesses corroborate reachability
- scoped fanout stays within zone until the local witness threshold is met
- parent-proxy pool relays a digest upward
- global scope marks the rack as provisional until cross-zone anti-entropy confirms it

## 2. Partitioned Region Reunion

- region `west` partitions from `east`
- both sides continue local convergence
- conflicting claims about one subject accumulate
- on recontact, deterministic reunion picks rendezvous peers by permutation rank
- residue remains visible until trust-weighted merge resolves the conflict

## 3. Low-Trust Introduction

- a weakly trusted node introduces a subject
- the claim spreads only within a narrow local scope
- a stronger witness disputes the introduction
- quarantine prevents broad blast radius
- operator visibility shows the conflict as provenance plus residue, not silent suppression

## 4. Arborition Repair Path

- a zone-level arborition carries witness summaries upward
- a subtree goes stale after repeated omission
- anti-entropy is scheduled along a repair arborition rather than full rebroadcast
- operators inspect the repair path and the blocked parent-proxy pool
