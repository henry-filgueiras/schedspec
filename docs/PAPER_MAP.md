# Paper Map

This document maps the repository into an eventual paper or treatise structure.

Its purpose is editorial rather than semantic. It says what each chapter is for, what order it is best read in, and where adjacent archived material belongs without competing with the active project.

## Active Docs

Canonical active-doc inventory:

- **Front door:** [`../README.md`](../README.md)
- **Spine chapters:** [`MANIFESTO.md`](MANIFESTO.md), [`ABSTRACT.md`](ABSTRACT.md), [`PRIMITIVES.md`](PRIMITIVES.md), [`SEMANTICS.md`](SEMANTICS.md), [`MEMBERSHIP.md`](MEMBERSHIP.md), [`DISSEMINATION.md`](DISSEMINATION.md), [`TRUST.md`](TRUST.md), [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md), [`TOPOLOGY.md`](TOPOLOGY.md)
- **Interstitial primitive chapters:** [`GLOSSARY.md`](GLOSSARY.md), [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md), [`ARBORITIONS.md`](ARBORITIONS.md), [`MECHANICS.md`](MECHANICS.md)
- **Appendices / support docs:** [`THREAT_MODEL.md`](THREAT_MODEL.md), [`EVALUATION.md`](EVALUATION.md), [`CRITIQUE.md`](CRITIQUE.md), [`EXAMPLES.md`](EXAMPLES.md), [`DIAGRAMS.md`](DIAGRAMS.md), [`PAPER_MAP.md`](PAPER_MAP.md), [`EDITORIAL_GUIDE.md`](EDITORIAL_GUIDE.md), [`MAINTENANCE_CHECKLIST.md`](MAINTENANCE_CHECKLIST.md)

## Reading Order

The canonical reading spine is:

1. **Dangerous preface / manifesto**  
   [`MANIFESTO.md`](MANIFESTO.md)
2. **Abstract**  
   [`ABSTRACT.md`](ABSTRACT.md)
3. **Glossary companion**  
   [`GLOSSARY.md`](GLOSSARY.md)
4. **Primitives**  
   [`PRIMITIVES.md`](PRIMITIVES.md)
5. **Semantics**  
   [`SEMANTICS.md`](SEMANTICS.md)
6. **Permutation rank**  
   [`PERMUTATION_RANK.md`](PERMUTATION_RANK.md)
7. **Arboritions**  
   [`ARBORITIONS.md`](ARBORITIONS.md)
8. **Mechanics**  
   [`MECHANICS.md`](MECHANICS.md)
9. **Membership**  
   [`MEMBERSHIP.md`](MEMBERSHIP.md)
10. **Dissemination**  
   [`DISSEMINATION.md`](DISSEMINATION.md)
11. **Trust**  
   [`TRUST.md`](TRUST.md)
12. **Merge and healing**  
   [`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md)
13. **Topology**  
   [`TOPOLOGY.md`](TOPOLOGY.md)
14. **Threat model**  
   [`THREAT_MODEL.md`](THREAT_MODEL.md)
15. **Evaluation**  
   [`EVALUATION.md`](EVALUATION.md)
16. **Critique**  
   [`CRITIQUE.md`](CRITIQUE.md)
17. **Examples**  
   [`EXAMPLES.md`](EXAMPLES.md)
18. **Diagrams**  
   [`DIAGRAMS.md`](DIAGRAMS.md)

## Chapter Roles

### Dangerous Preface / Manifesto

[`MANIFESTO.md`](MANIFESTO.md) is the dangerous preface.

Its job is to say what problem the repo refuses to simplify away. It should stay sharp, memorable, and slightly dangerous. It is not the place for object taxonomy or evaluation detail.

### Abstract

[`ABSTRACT.md`](ABSTRACT.md) is the framing chapter.

Its job is to summarize the argument of the whole treatise in a paper-like way: problem setting, central claim, key invariants, and the role of the distinctive primitives. It should motivate the body without trying to replace it.

### Primitives

[`PRIMITIVES.md`](PRIMITIVES.md) is the vocabulary chapter.

Its job is to define compact shared terms and fast distinctions so that later chapters can reuse words without drift. It should stay concise and avoid becoming the full semantic contract.

### Glossary

[`GLOSSARY.md`](GLOSSARY.md) is an interstitial primitive chapter.

Its job is to give the reader a fast stable lookup surface for the load-bearing terms without replacing either `PRIMITIVES.md` or `SEMANTICS.md`.

### Semantics

[`SEMANTICS.md`](SEMANTICS.md) is the semantic backbone.

Its job is to define the protocol objects, decision surfaces, invariants, and skeletal decision flows that a conforming design would need to preserve. This is where the model becomes crisp.

### Permutation Rank

[`PERMUTATION_RANK.md`](PERMUTATION_RANK.md) is an interstitial primitive chapter.

Its job is to treat seeded accountable ordering as a first-class primitive rather than as a convenience trick hidden inside witness or repair code.

### Arboritions

[`ARBORITIONS.md`](ARBORITIONS.md) is an interstitial primitive chapter.

Its job is to define adaptive dissemination, witness, and repair forests as explicit protocol structure instead of implementation residue.

### Mechanics

[`MECHANICS.md`](MECHANICS.md) is an interstitial primitive chapter.

Its job is to turn the semantic model into algorithm-shaped procedures without pretending the repo already contains a complete runtime or wire protocol.

### Membership

[`MEMBERSHIP.md`](MEMBERSHIP.md) is the behavioral chapter for introduction, witness formation, scoped belief, and visible lifecycle state.

Its job is to explain how membership behaves under partial observability, not to re-explain the whole treatise.

### Dissemination

[`DISSEMINATION.md`](DISSEMINATION.md) is the propagation chapter.

Its job is to explain how claims move, when blast radius should stay bounded, and how scope and topology shape spread.

### Trust

[`TRUST.md`](TRUST.md) is the witness-quality and confidence chapter.

Its job is to explain why not all corroboration is equal and how that should affect propagation and acceptance.

### Merge And Healing

[`MERGE_AND_HEALING.md`](MERGE_AND_HEALING.md) is the reconciliation chapter.

Its job is to explain negotiated reality merge, residue, deterministic reunion, and repair.

### Topology

[`TOPOLOGY.md`](TOPOLOGY.md) is the structure chapter.

Its job is to explain locality, hierarchy, parent-proxy interaction, permutation rank in topology, and arboritions as overlay structure.

### Threat Model

[`THREAT_MODEL.md`](THREAT_MODEL.md) is an appendix / support chapter.

Its job is to say what hostile timing, stale witnesses, scope abuse, and adversarial behavior the design is trying to remain honest under.

### Evaluation

[`EVALUATION.md`](EVALUATION.md) is an appendix / support chapter.

Its job is to say how the design should be compared, stressed, costed, and potentially falsified.

### Critique

[`CRITIQUE.md`](CRITIQUE.md) is an appendix / support chapter.

Its job is to state the strongest fair objections to the project, distinguish superficial complaints from genuinely dangerous ones, and say where the whole proposal could still fail.

### Examples

[`EXAMPLES.md`](EXAMPLES.md) is an appendix / support chapter.

Its job is to force the primitives and chapters to interact in concrete operating conditions.

### Diagrams

[`DIAGRAMS.md`](DIAGRAMS.md) is an appendix / support chapter.

Its job is to keep the protocol shape legible in canonical editable Mermaid form.

## Public Surfaces

The most public-facing surfaces in the repo are:

- [`../README.md`](../README.md): front door and navigational overview
- [`MANIFESTO.md`](MANIFESTO.md): dangerous preface
- [`ABSTRACT.md`](ABSTRACT.md): framing summary

These should stay concise and should not absorb the full responsibility of later chapters.

## Archived Adjacent Lineages

The active project is Resonant Membership.

Archived adjacent lineages are preserved under [`../notes/archive`](../notes/archive):

- ChronOS / `chrono flow`: [`../notes/archive/chronos/README.md`](../notes/archive/chronos/README.md)
- SameDiff: [`../notes/archive/SAMEDIFF.md`](../notes/archive/SAMEDIFF.md)

These remain part of the repo because the framing is still strong and adjacent, not because they are co-equal active chapters in the current treatise.
