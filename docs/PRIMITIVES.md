# Primitives and Glossary

This document defines the shared vocabulary used across the Resonant Membership docs.

See also:

- [`ABSTRACT.md`](ABSTRACT.md) for the long-form framing
- [`GLOSSARY.md`](GLOSSARY.md) for the compact terminology index
- [`MEMBERSHIP.md`](MEMBERSHIP.md) for lifecycle and protocol behavior
- [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) and [`ARBORITIONS.md`](ARBORITIONS.md) for the distinctive protocol primitives

## What Problem This Section Solves

Systems papers tend to fail early when their terms drift.

This section exists to keep the rest of the repo from quietly changing the meaning of words like `claim`, `witness`, `scope`, `confidence`, or `residue` from page to page.

The goal is not merely definitional hygiene. A membership design built on partial observability depends on sharp vocabulary because the same event may mean different things to different scopes. If the terms blur, the semantics blur with them.

## Core Terms

The definitions below are intentionally compact. They are the small vocabulary the rest of the repo should keep reusing without drift.

- **membership view:** a node's current structured belief about who belongs, in what state, under what confidence
- **subject:** the node, service, endpoint, or identity about which a claim is made
- **claim:** a statement about membership state, identity, trust, scope, or reachability
- **observation:** locally derived evidence, such as direct contact, timeout, challenge-response, or topology evidence
- **witness:** an observer whose corroboration or dispute contributes to belief formation
- **introducer:** a node or authority that first presents a subject to a scope
- **scope:** the bounded audience or jurisdiction for a claim, such as rack, zone, service, region, or trust domain
- **trust weight:** the protocol-relevant credibility attached to an introducer or witness
- **residue:** visible unresolved disagreement that should not be flattened into false certainty
- **merge rule:** the rule by which competing claims and observations are reconciled
- **healing:** explicit reconciliation after partition, omission, or divergent evolution
- **permutation rank:** seeded deterministic peer ordering for accountable fanout, rendezvous, tie-breaking, and auditability
- **arborition:** adaptive topology-aware dissemination, witness, and repair trees or overlay forests

## Primitive Operations

Resonant Membership repeatedly performs a small set of operations:

- introduce a subject into a scope
- witness or dispute a claim
- disseminate a claim along a scoped path
- merge competing views
- quarantine unresolved disagreement as residue
- repair divergence after connectivity or trust changes

These operations are more central than any particular transport.

Two especially distinctive primitives in this repo are important enough to stand on their own:

- [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md)
- [`ARBORITIONS.md`](ARBORITIONS.md)

They are not decorative terminology. They are load-bearing parts of the protocol story:

- **permutation rank** makes selection decisions reproducible enough to audit
- **arboritions** make propagation and repair structure explicit enough to inspect

## Design Invariants

The design aims to preserve these invariants:

1. **Partial observability is normal.** No node should assume it sees the whole system directly.
2. **Claims are not self-justifying.** Introduction and witness matter.
3. **Deterministic ordering is accountable.** Selection decisions should be reconstructable.
4. **Scope is part of meaning.** A claim accepted in one scope may remain tentative in another.
5. **Merge must preserve disagreement when needed.** Convergence should not require erasing evidence of conflict.
6. **Healing is explicit.** Partition repair should be modeled, not hand-waved.
7. **Topology matters.** Locality and hierarchy shape efficient and credible dissemination.
8. **Operator observability is structural.** The system should explain who believed what, why, and through which path.

## Fast Distinctions

- **observation vs claim:** an observation is local evidence; a claim is transmissible protocol content
- **introducer vs witness:** an introducer presents a subject; a witness corroborates or disputes it
- **confidence vs trust weight:** confidence is about a specific belief state; trust weight is about the credibility of a source
- **scope vs topology:** scope controls where a claim is relevant; topology shapes how it should travel
- **merge vs healing:** merge reconciles views at contact; healing is the broader repair process after divergence
- **fanout vs permutation rank:** fanout is count; permutation rank is accountable ordering
- **tree vs arborition:** a tree is static structure; an arborition is an adaptive overlay forest for dissemination, witness, and repair

## Tradeoffs And Failure Modes

Vocabulary does not remove ambiguity by itself.

The failure mode this section tries to prevent is quieter: multiple components behaving coherently locally while disagreeing about the meaning of key terms globally. Once that happens, even correct-looking convergence becomes hard to reason about.

Another danger is compression in the wrong place. If the system uses the word `membership` to hide distinctions among introduction, witness, acceptance, dispute, quarantine, and residue, then the paper may look clean while the protocol itself remains under-specified.

## Operator Interpretation

A serious system should answer questions like:

- who introduced this subject?
- which witnesses corroborated or disputed it?
- why did this claim propagate to this scope?
- why was this rendezvous set selected?
- which merge rule resolved the conflict?
- where does residue remain?
- which arborition path is carrying repair traffic?

If those questions cannot be answered structurally, the protocol is hiding too much of its own behavior.
