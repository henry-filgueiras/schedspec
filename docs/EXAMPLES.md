# Examples

These are Resonant Membership scenarios meant to make the abstractions concrete without pretending the protocol is fully implemented.

## What Problem This Section Solves

The protocol vocabulary can sound elegant while remaining too abstract. These scenarios exist to show what scoped belief, witness quality, deterministic reunion, and topology-aware repair look like in plausible operating conditions.

Each scenario is written to answer the same four questions the rest of the paper spine is trying to keep visible:

- what problem is the system solving?
- which invariants matter most?
- where can the protocol fail or become misleading?
- how would an operator understand what happened?

## 1. Service Mesh / Multi-Region Membership And Partition Healing

Imagine a service mesh deployed across `us-west` and `us-east`, with local zone scopes under each region and a higher-level global scope above them. Membership is used for routing, placement, and failure handling, so the system cannot afford to confuse local usefulness with global certainty.

### Situation

A subject in `us-west` is introduced by a regional trust root and quickly corroborated by local witnesses. The regional view converges enough for `us-west` traffic policy to treat the subject as usable. Global dissemination remains provisional because `us-east` has not yet observed the subject directly. A partition then isolates the two regions long enough for contradictory local beliefs to accumulate about one service instance: `us-west` sees it as healthy but load-shedding, while `us-east` never confirms the latest introduction and carries an older suspicion record.

### What The Protocol Needs To Do

The protocol must keep `us-west` locally useful without pretending that global convergence already exists. It must preserve provenance for the original introduction, record which witnesses were decisive in each region, choose deterministic reunion peers when the partition heals, and merge the competing regional views without erasing residue too early.

This is exactly where a flat membership list becomes misleading. The system does not have one truth and one lagging replica. It has multiple scope-local belief states that must be reconciled under explicit merge rules.

### Design Invariants

- scope-local confidence is allowed to be stronger than global confidence
- restored connectivity is not equivalent to healed reality
- deterministic reunion should be reconstructable after the fact
- residue should remain visible if both regions carry credible but conflicting witness histories

### Tradeoffs And Failure Modes

The main risk is premature global acceptance. If the first region to reconnect implicitly dominates, the system may flatten disagreement into convenience. Another risk is repair storm behavior: once connectivity returns, over-eager anti-entropy may cause both regions to re-fanout aggressively before the reunion path stabilizes. A third risk is that summary views collapse too much detail, making it look as though the system "just converged" when in fact it performed a trust-weighted compromise.

### Operator Interpretation

An operator should be able to ask:

- which region accepted the subject first?
- which witnesses in each region drove the local belief?
- which permutation-rank rendezvous peers were chosen during reunion?
- what residue remains after the initial merge?

If the system cannot answer those questions, it is hiding the most important part of the healing event.

## 2. Edge Swarm / Partial-Trust Cluster With Scoped Witness Behavior

Now imagine a large edge swarm with uneven node quality, intermittent reachability, and several trust tiers. Some nodes are operator-managed, some are field-managed, and some are only weakly trusted due to unstable history. The system still needs usable local membership, but it cannot let every new introduction become a cluster-wide rumor.

### Situation

A weakly trusted edge node introduces a nearby subject. Local witnesses in the same physical vicinity observe it, but their histories are correlated: they share the same radio environment, the same parent connectivity, and the same operational owner. Higher-trust regional nodes remain skeptical and restrict blast radius. The claim propagates upward only as a scoped digest through a parent-proxy pool, while the local scope keeps the subject alive in provisional state.

### What The Protocol Needs To Do

The protocol must allow local usefulness without granting global credibility too early. It must preserve the fact that the introduction came from a weakly trusted source, select witnesses in a way that is topology-aware and reproducible, and carry the claim upward without letting weak confidence masquerade as strong trust.

This is where witness quality and scope interact directly. The system is not deciding whether the subject "exists" in some universal sense. It is deciding where the claim may safely matter while the witness story is still incomplete.

### Design Invariants

- trust should influence blast radius, not only acceptance
- witness diversity matters in low-quality environments
- scoped fanout is preferable to global rumor spread
- quarantine and hysteresis should exist before global suppression or global belief

### Tradeoffs And Failure Modes

The obvious failure is weak introductions leaking too far too fast. A subtler one is witness-set correlation: if the system chooses all witnesses from one local failure domain, apparent corroboration may really be shared bias. Another risk is that parent-proxy pools amplify noise rather than filter it, especially if upward digests are treated as stronger evidence than they deserve. Without hysteresis, the subject may oscillate between local provisional status and quarantine every time partial connectivity shifts.

### Operator Interpretation

An operator should be able to ask:

- why did this claim remain local?
- which witnesses were considered credible enough to keep the claim alive?
- which parent-proxy pool relayed the digest upward?
- why did the system retain quarantine or residue instead of converging?

A design that cannot show this path will make edge behavior look arbitrary.

## 3. Bootstrapping A New Rack

A new rack-local subject is introduced by a zone trust root. Local witnesses corroborate reachability, scoped fanout stays within zone until the witness threshold is met, and a parent-proxy pool relays a digest upward. Global scope marks the rack as provisional until cross-zone anti-entropy confirms it. The point of the scenario is that bootstrap should not quietly jump from "introduced" to "globally accepted" just because the rack came online through an approved path.

## 4. Low-Trust Introduction Under Dispute

A weakly trusted node introduces a subject. The claim spreads only within a narrow local scope. A stronger witness disputes the introduction, quarantine prevents broad blast radius, and operator visibility shows the conflict as provenance plus residue rather than silent suppression. This scenario exists to make clear that non-convergence can be the honest protocol outcome.

## Conclusion

Across these scenarios, the same pattern keeps recurring: under partial observability, usable convergence depends on witness quality, scope, deterministic ordering, healing discipline, and explicit residue. If any one of those disappears behind generic gossip language, the protocol becomes much harder to trust.
