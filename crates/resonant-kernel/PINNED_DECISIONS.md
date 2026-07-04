# Pinned Decisions

The treatise in `docs/` deliberately leaves parts of the design open, and
audits its own gaps (`docs/SPEC_AUDIT.md`, `docs/OPEN_QUESTIONS.md`). This
crate is the *reference instantiation*: for every flagged gap it commits to
one concrete, documented choice. The docs stay a treatise; the pins live
here and as `PINNED` doc-comments on the owning modules.

| Pin | Gap it closes | Doc citation | Choice | Owner |
|---|---|---|---|---|
| P1 | State vocabulary not fully canonical | SPEC_AUDIT High #1; SEMANTICS.md "Canonical Membership States" | The nine lowercase SEMANTICS states are `BeliefState`, `TRANSITION_TABLE` is the executable form of the SEMANTICS transition table, merge output classes are a projection (`MergeResolution::project`), and revocation is an event (`BeliefEvent::Revoked`), never a state. | `belief/state.rs` |
| P2 | `permutation_rank` construction open | PERMUTATION_RANK.md; OPEN_QUESTIONS "hotspot risk" | Order by `(blake3::keyed_hash(K, candidate), candidate_id)` with `K = blake3("resonant/rank/v1" ‖ canonical(RankSeed))`. Per-candidate hashing keeps pools stable under growth; `round` in the seed rotates hotspots; `RankedSelection` records seed, pool, exclusions-with-reasons, and full order so `reconstruct` verifies it independently. | `rank.rs` |
| P3 | Rank must only tie-break | SEMANTICS.md merge contract | Cross-class dominance decisions consume `DominanceEvidence`-style count-free scores with no rank parameter; rank/input-order appears only in within-class representative selection and is *named* (`DecidedBy::InputOrder`) when it decides. | `merge/engine.rs` |
| P4 | Trust-root lifecycle too open | SPEC_AUDIT High #4; TRUST.md; OPEN_QUESTIONS "trust-root promotion" | `TrustRootStanding` per (root, scope): `Proposed → {Active | Probation} → Active ⇄ Narrowed → Suspended → Revoked` (terminal). Basis is permanent (`OperatorPolicy | IdentityLineage | EarnedHistory`); **earned standing has no shortcut past probation** and probation carries half effective weight; revocation requires attributable support. | `trust.rs` |
| P5 | Digest semantics under-specified | SPEC_AUDIT High #3; PRIMITIVES.md RepairDigest | `RepairDigest::of(&view)` is the only constructor; it must preserve scope, epoch, per-state counts, per-subject (state, epoch) summaries, live residue ids, and computed honesty flags. Comparison yields `NoAction | FetchDetail | HoldForRepair` — the merge engine consumes only full fragments, so a summary can never be merged as evidence. | `digest.rs` |
| P6 | Quarantine/hysteresis thresholds open | SEMANTICS.md quarantine section; TRUST.md hysteresis | `PolicyBundle` defaults: strengthen after ≥2 rounds held; quarantine release after ≥2 rounds *and* fresh corroboration (carried in the event by construction); stale after 2 epochs. Weakening always bypasses hysteresis — slow to strengthen, fast to weaken. | `policy.rs`, `belief/cell.rs` |
| P7 | Merge calculus inside the allowed family | SPEC_AUDIT Medium #5; OPEN_QUESTIONS "merge precedence" | One concrete calculus: admissibility gate → semantic classification (permissive/restrictive) → count-free cross-class dominance with the lab's margins → within-class scores where the capped informer may participate → residue mandatory on material conflict. Score tables and margins = the Deterministic Reunion Lab's constants (`MergePolicy::lab_compat`), so the JS lab and this engine are two renderings of one calculus — proven by the conformance sweep over every scenario × replay prefix × override. | `policy.rs`, `merge/engine.rs` |
| P8 | Trust weights numeric vs ordinal; trust ≠ confidence | SEMANTICS.md trust section | `TrustGrade(u8 0..=100)` ordinal with named bands; `Confidence` a separate four-variant enum with no arithmetic bridge. | `trust.rs` |
| P9 | Clocking model open | SEMANTICS.md epochs section | No wall clock anywhere: caller-supplied per-scope `Epoch(u64)` plus a kernel `Round(u64)`; freshness is epoch arithmetic against policy. | `epoch.rs` |
| P10 | Scope vs topology entangled | SPEC_AUDIT High #2 | The kernel has **no topology types**. Scope owns meaning (`ScopeId`, `ScopeAuthority` in the merge); topology enters only as caller-provided candidate pools with `ExclusionReason` annotations. Topology cannot influence claim meaning because the kernel has no vocabulary for it. | `scope.rs`, `rank.rs` |
| P11 | Residue growth vs usefulness | OPEN_QUESTIONS "residue growth" | Growth bounded by superseding same-tension entries — a visible replacement, never a silent TTL. `ResidueLedger` has no public remove; the only exits are `resolve` (with evidence) and supersession. | `residue.rs` |
| P12 | Revocation representation | OPEN_QUESTIONS "quarantine/revocation" (leaning) | One `RevocationEvent` with three typed targets (subject acceptance, witness standing, trust-root standing); outcome constrained to the four degraded states along canonical edges. | `belief/state.rs` |
| P13 | Witness quality/diversity vocabulary | (docs give none; lab.js implies it) | `Quality {Weak, Mixed, Strong}` × `Diversity {Laundered, SingleScope, Mixed, CrossScope}`; `Laundered` is first-class and scores below no-diversity, so the trust-laundering failure mode lives in types. | `evidence.rs` |
| P19 | Where a `WitnessSummary` comes from | (docs and lab both take summaries as given) | Derived from raw evidence by `EvidenceBook::summarize`: count = distinct corroborating witnesses; quality from observation modes (challenge-response/direct-contact strong, timeout/topology weak; mostly-strong with ≥2 witnesses → Strong); diversity from distinct vouch-lineage roots (≥3 → CrossScope, 2 → Mixed, 1 → SingleScope, and 1 root behind a loud (≥4) non-strong cluster → Laundered). Advancement gates on the derived summary, so laundered volume cannot buy standing. | `witnessing.rs` |

## Pins forced by property testing

These came out of proptest counterexamples during implementation — places
where lab.js parity alone would have left a dishonest edge. They have been
folded back into the treatise: the canonical statements live in
`docs/SEMANTICS.md` under "Refinements From The Reference Kernel", with the
residue-identity finding also tracked (as resolved) in
`docs/OPEN_QUESTIONS.md`.

| Pin | Counterexample | Choice |
|---|---|---|
| P14 | A merge projecting `introduced` was refused wholesale | A merge may *carry* an introduction into a scope with no live belief (`unknown`/`removed` → `introduced`) but never re-introduce over live belief. |
| P15 | A one-sided `disputed` fragment carried forward as "stable, no residue" | A dispute is never stable: one-sided provisional *or disputed* evidence stays provisional with residue. |
| P16 | Two islands both `disputed` "converged cleanly" to a stable dispute with no residue | Agreement that a conflict exists is not its resolution: matching disputes stay provisional and leave residue. |
| P17 | The unexercised fallback branch let count flip cross-class outcomes and broke side-symmetry on ties | Fallback compares count-free dominance only; exact ties resolve by content (the more conservative state wins), not input order. |
| P18 | Two replicas performing the same reunion at different local rounds minted different residue ids and never converged | The rendezvous round is part of the agreed reunion input (`Input::ReunionRequested { round, .. }`), not local state. Necessary but not sufficient — see P20. |
| P20 | Cascaded pairwise reunions (A⟷C reunites before B⟷C) observe the same conflict through different tension shapes and minted different residue ids — permanent digest divergence over the real wire | Residue identity is content-addressed on the belief key alone (scope + subject): identity names *the conflict about this subject here*; tension sides, detail, and birth coordinates are content, not identity. Re-minting supersedes content in place and preserves an existing override-responsibility mark. Companion rule: a later stable, conflict-free convergence on a subject *resolves* its live residue into the visible resolved record (evidence: the merge input digest) — residue tracks live disagreement, history is never erased. Reunion is also gated to material divergence (class conflict or live disagreement), so strengthening drift never mints scars. |

## Documented divergences from lab.js

All confined to inputs the scenario corpus never produces; on the corpus the
kernel and the lab agree everywhere (see `resonant scenario verify`):

- both-sides-laundered and both-sides-restrictive keep lab.js's
  side-A-first asymmetry in the *oracle* (string-exact port) while the
  kernel handles them via the same branches; both are excluded from the
  side-symmetry property and marked in code;
- the lab's fallback branch used the flat count-inclusive score; the
  kernel's is count-free (P17);
- the lab reads the replayed step count from page state; the Rust port
  takes it as a parameter;
- the lab's naive-comparison null-dereference is unrepresentable under
  `Option`.
