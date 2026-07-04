//! The reunion engine: gate, classify, compare, preserve.
//!
//! PINNED (P7): the calculus below is one concrete instantiation of the
//! docs' allowed precedence family, arranged so the JS lab and this
//! engine produce the same outcomes on the scenario corpus. The structural
//! difference from lab.js: every *cross-class* comparison here consumes
//! count-free `DominanceEvidence` scores, so the capped informer can shift
//! a within-class choice but can never flip acceptance into removal —
//! lab.js folds the capped count into one flat score and relies on the cap;
//! this engine makes the boundary a type, and the conformance suite proves
//! the outcomes still agree on the whole corpus at every replay prefix.

use crate::belief::state::BeliefState;
use crate::belief::view::MembershipView;
use crate::belief::{BeliefEvent, Transition, TransitionError};
use crate::epoch::{Epoch, Round};
use crate::evidence::{Diversity, Quality};
use crate::id::{write_str, CanonicalBytes, SubjectId};
use crate::merge::{
    Admissibility, BeliefFragment, DecidedBy, InadmissibleReason, MergeResolution, MergeSource,
    MergeStability, MergeTrace, OverallOutcome, ReunionDigest, RuleId, SubjectMergeOutcome,
};
use crate::operator::OperatorOverride;
use crate::policy::{MergePolicy, PolicyBundle};
use crate::residue::{BeliefKey, ConflictTension, Residue, TensionSide};
use crate::scope::ScopeId;
use crate::trust::TrustGrade;
use crate::util::NonEmpty;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// One party to a reunion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeSide {
    pub label: String,
    pub epoch: Epoch,
    pub fragments: BTreeMap<SubjectId, BeliefFragment>,
}

/// Where and when a reunion happens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReunionContext {
    pub scope: ScopeId,
    pub epoch: Epoch,
    pub round: Round,
}

/// The whole-view reunion result. `#[must_use]`: the only consumption path
/// is `apply_to`, which routes projections through the state machine and
/// residue into the ledger.
#[must_use = "a reunion outcome carries residue; apply it to a view or account for it explicitly"]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReunionOutcome {
    pub context: ReunionContext,
    pub input_digest: [u8; 32],
    pub outcomes: Vec<SubjectMergeOutcome>,
    pub digest: ReunionDigest,
}

fn admissibility(fragment: Option<&BeliefFragment>) -> Admissibility {
    match fragment {
        None => Admissibility::Inadmissible(InadmissibleReason::NoBelief),
        Some(f) if f.state == BeliefState::Unknown => {
            Admissibility::Inadmissible(InadmissibleReason::NoBelief)
        }
        Some(f) if f.trust.get() == 0 => Admissibility::Inadmissible(InadmissibleReason::NoTrust),
        Some(_) => Admissibility::Admissible,
    }
}

fn is_restrictive(state: BeliefState) -> bool {
    matches!(state, BeliefState::Removed | BeliefState::Quarantined)
}

fn is_permissive(state: BeliefState) -> bool {
    matches!(state, BeliefState::Accepted | BeliefState::Provisional)
}

/// The count-free "may dominate" score: trust + corroboration quality and
/// diversity + scope authority. No witness count, no rank, no arrival
/// order — those inputs do not exist at this call site.
fn dominance_score(policy: &MergePolicy, f: &BeliefFragment) -> i64 {
    i64::from(f.trust.get())
        + policy.quality_score(f.witness.quality)
        + policy.diversity_score(f.witness.diversity)
        + policy.authority_score(f.authority)
}

fn informer_score(policy: &MergePolicy, f: &BeliefFragment) -> i64 {
    policy.informer_score(f.witness.count)
}

/// Within-class strength: dominance plus the capped informer. Only ever
/// consulted when both sides are in the same semantic class.
fn within_class_score(policy: &MergePolicy, f: &BeliefFragment) -> i64 {
    dominance_score(policy, f) + informer_score(policy, f)
}

/// Attribute a within-class choice to the tier that actually decided it.
fn attribute(policy: &MergePolicy, winner: &BeliefFragment, loser: &BeliefFragment) -> DecidedBy {
    let d_trust = i64::from(winner.trust.get()) - i64::from(loser.trust.get());
    let d_corr = (policy.quality_score(winner.witness.quality)
        + policy.diversity_score(winner.witness.diversity))
        - (policy.quality_score(loser.witness.quality)
            + policy.diversity_score(loser.witness.diversity));
    let d_auth = policy.authority_score(winner.authority) - policy.authority_score(loser.authority);
    let d_dominance = d_trust + d_corr + d_auth;
    let d_informer = informer_score(policy, winner) - informer_score(policy, loser);
    if d_dominance == 0 && d_informer == 0 {
        DecidedBy::InputOrder
    } else if d_dominance.abs() >= d_informer.abs() {
        if d_corr.abs() > d_trust.abs() + d_auth.abs() {
            DecidedBy::Corroboration
        } else {
            DecidedBy::TrustAndAuthority
        }
    } else {
        DecidedBy::Informer
    }
}

fn side_name(source: MergeSource) -> &'static str {
    match source {
        MergeSource::SideA => "side_a",
        MergeSource::SideB => "side_b",
        MergeSource::Both => "both",
        MergeSource::Override => "override",
        MergeSource::None => "none",
    }
}

struct SubjectContext<'a> {
    ctx: &'a ReunionContext,
    subject: &'a SubjectId,
}

impl SubjectContext<'_> {
    fn residue(
        &self,
        detail: impl Into<String>,
        sides: Vec<(MergeSource, &BeliefFragment)>,
    ) -> Residue {
        Residue::new(
            BeliefKey {
                scope: self.ctx.scope.clone(),
                subject: self.subject.clone(),
            },
            ConflictTension {
                sides: sides
                    .into_iter()
                    .map(|(source, f)| TensionSide {
                        source: side_name(source).to_string(),
                        state: f.state,
                        epoch: f.epoch,
                    })
                    .collect(),
                detail: detail.into(),
            },
            (self.ctx.epoch, self.ctx.round),
        )
    }
}

#[allow(clippy::too_many_lines)]
fn merge_subject(
    policy: &MergePolicy,
    ctx: &ReunionContext,
    subject: &SubjectId,
    a: Option<&BeliefFragment>,
    b: Option<&BeliefFragment>,
) -> SubjectMergeOutcome {
    let sc = SubjectContext { ctx, subject };
    let adm_a = admissibility(a);
    let adm_b = admissibility(b);
    let mut gate_drops = Vec::new();
    if let Admissibility::Inadmissible(reason) = &adm_a {
        gate_drops.push((MergeSource::SideA, reason.clone()));
    }
    if let Admissibility::Inadmissible(reason) = &adm_b {
        gate_drops.push((MergeSource::SideB, reason.clone()));
    }
    let has_a = adm_a == Admissibility::Admissible;
    let has_b = adm_b == Admissibility::Admissible;

    let base_epoch = a
        .map_or(Epoch(0), |f| f.epoch)
        .max(b.map_or(Epoch(0), |f| f.epoch));
    let base_trust = TrustGrade::new(
        a.map_or(0, |f| f.trust.get())
            .max(b.map_or(0, |f| f.trust.get())),
    );

    let trace = |rule: RuleId, decided_by: DecidedBy, notes: Vec<String>| MergeTrace {
        rule,
        decided_by,
        gate_drops: gate_drops.clone(),
        dominance: [
            has_a.then(|| dominance_score(policy, a.unwrap())),
            has_b.then(|| dominance_score(policy, b.unwrap())),
        ],
        informer: [
            has_a.then(|| informer_score(policy, a.unwrap())),
            has_b.then(|| informer_score(policy, b.unwrap())),
        ],
        notes,
    };
    let outcome = |resolution: MergeResolution,
                   trace: MergeTrace,
                   source: MergeSource,
                   epoch: Epoch,
                   trust: TrustGrade| SubjectMergeOutcome {
        subject: subject.clone(),
        resolution,
        trace,
        source,
        epoch,
        trust,
    };

    // Tier 0: the gate.
    if !has_a && !has_b {
        return outcome(
            MergeResolution::Converged {
                state: BeliefState::Unknown,
                decided_by: DecidedBy::AdmissibilityGate,
                residue: vec![],
            },
            trace(
                RuleId::NoAdmissibleInput,
                DecidedBy::AdmissibilityGate,
                vec![],
            ),
            MergeSource::None,
            base_epoch,
            base_trust,
        );
    }

    if has_a != has_b {
        let (entry, source) = if has_a {
            (a.unwrap(), MergeSource::SideA)
        } else {
            (b.unwrap(), MergeSource::SideB)
        };
        // A provisional or disputed state is never carried forward as
        // stable, however strong the single path is — one side alone
        // cannot upgrade tentativeness, and a dispute is never stable.
        let strong = within_class_score(policy, entry) >= policy.one_sided_stable
            && entry.state != BeliefState::Provisional
            && entry.state != BeliefState::Disputed;
        let resolution = if strong {
            MergeResolution::Converged {
                state: entry.state,
                decided_by: DecidedBy::AdmissibilityGate,
                residue: vec![],
            }
        } else {
            MergeResolution::ProvisionalConverged {
                state: entry.state,
                decided_by: DecidedBy::AdmissibilityGate,
                residue: vec![sc.residue(
                    "Only one side had enough evidence to speak; reunion preserves the asymmetry.",
                    vec![(source, entry)],
                )],
            }
        };
        return outcome(
            resolution,
            trace(RuleId::SingleSided, DecidedBy::AdmissibilityGate, vec![]),
            source,
            entry.epoch,
            entry.trust,
        );
    }

    let a = a.unwrap();
    let b = b.unwrap();

    // Semantic agreement needs no dominance at all.
    if a.state == b.state {
        let detail_by = attribute(policy, a, b);
        let stronger = if within_class_score(policy, a) >= within_class_score(policy, b) {
            MergeSource::SideA
        } else {
            MergeSource::SideB
        };
        let resolution = if a.state == BeliefState::Disputed {
            // Two sides agreeing that a subject is disputed is agreement
            // about the existence of a conflict, not its resolution: the
            // dispute stays provisional and leaves residue. (Unexercised
            // by the corpus; lab.js would call this "stable".)
            MergeResolution::ProvisionalConverged {
                state: a.state,
                decided_by: DecidedBy::StateAgreement,
                residue: vec![sc.residue(
                    "Both sides carried the same unresolved dispute into reunion.",
                    vec![(MergeSource::SideA, a), (MergeSource::SideB, b)],
                )],
            }
        } else if a.state == BeliefState::Provisional {
            MergeResolution::ProvisionalConverged {
                state: a.state,
                decided_by: DecidedBy::StateAgreement,
                residue: vec![],
            }
        } else {
            MergeResolution::Converged {
                state: a.state,
                decided_by: DecidedBy::StateAgreement,
                residue: vec![],
            }
        };
        return outcome(
            resolution,
            trace(
                RuleId::CleanConvergence,
                DecidedBy::StateAgreement,
                vec![format!(
                    "{} supplied the stronger retained detail (within-class, decided by {detail_by:?})",
                    side_name(stronger)
                )],
            ),
            MergeSource::Both,
            base_epoch,
            base_trust,
        );
    }

    // Corroboration tier: laundered diversity is discounted before any
    // score contest. Cross-class comparison — dominance evidence only.
    let laundering_a = a.witness.diversity == Diversity::Laundered;
    let laundering_b = b.witness.diversity == Diversity::Laundered;
    if laundering_a || laundering_b {
        // Reproduces lab.js: if both sides laundered, side A is treated as
        // the laundered one (unexercised by the corpus).
        let (laundered, other, other_source) = if laundering_a {
            (a, b, MergeSource::SideB)
        } else {
            (b, a, MergeSource::SideA)
        };
        if dominance_score(policy, other)
            >= dominance_score(policy, laundered) - policy.laundering_tolerance
        {
            let state = if other.state == BeliefState::Accepted {
                BeliefState::Provisional
            } else {
                other.state
            };
            let residue = sc.residue(
                "A louder but lower-quality witness cluster was discounted; the laundering attempt stays visible.",
                vec![(MergeSource::SideA, a), (MergeSource::SideB, b)],
            );
            return outcome(
                MergeResolution::ProvisionalConverged {
                    state,
                    decided_by: DecidedBy::Corroboration,
                    residue: vec![residue],
                },
                trace(RuleId::LaunderingDiscount, DecidedBy::Corroboration, vec![]),
                other_source,
                base_epoch,
                other.trust,
            );
        }
    }

    // Restrictive vs. permissive: the cross-class core. All comparisons
    // below consume dominance scores — the informer cannot reach them.
    if is_restrictive(a.state) || is_restrictive(b.state) {
        let (restrictive, permissive, restrictive_source) = if is_restrictive(a.state) {
            (a, b, MergeSource::SideA)
        } else {
            (b, a, MergeSource::SideB)
        };
        let dom_r = dominance_score(policy, restrictive);
        let dom_p = dominance_score(policy, permissive);

        if restrictive.epoch == permissive.epoch
            && restrictive.witness.quality != Quality::Weak
            && permissive.witness.quality != Quality::Weak
            && (dom_r - dom_p).abs() < policy.dispute_closeness
        {
            let residue = sc.residue(
                "Fresh admissible evidence remains materially unresolved; certainty is not counterfeited.",
                vec![(MergeSource::SideA, a), (MergeSource::SideB, b)],
            );
            return outcome(
                MergeResolution::ScopedDisagreement {
                    residue: NonEmpty::new(residue),
                },
                trace(RuleId::SameEpochDispute, DecidedBy::Unresolved, vec![]),
                MergeSource::Both,
                base_epoch,
                base_trust,
            );
        }

        if restrictive.epoch > permissive.epoch && dom_r >= dom_p - policy.restrictive_fresh_slack {
            let residue = sc.residue(
                "The newer restrictive path wins, but the race stays visible.",
                vec![(MergeSource::SideA, a), (MergeSource::SideB, b)],
            );
            return outcome(
                MergeResolution::Converged {
                    state: restrictive.state,
                    decided_by: DecidedBy::Freshness,
                    residue: vec![residue],
                },
                trace(
                    RuleId::FreshRestrictiveDominates,
                    DecidedBy::Freshness,
                    vec![],
                ),
                restrictive_source,
                restrictive.epoch,
                restrictive.trust,
            );
        }

        if dom_r >= dom_p + policy.restrictive_dominance {
            let residue = sc.residue(
                "The restrictive path dominates on trust and authority; the opposing history stays visible.",
                vec![(MergeSource::SideA, a), (MergeSource::SideB, b)],
            );
            return outcome(
                MergeResolution::ProvisionalConverged {
                    state: restrictive.state,
                    decided_by: DecidedBy::TrustAndAuthority,
                    residue: vec![residue],
                },
                trace(
                    RuleId::RestrictiveDominance,
                    DecidedBy::TrustAndAuthority,
                    vec![],
                ),
                restrictive_source,
                restrictive.epoch,
                restrictive.trust,
            );
        }

        let residue = sc.residue(
            "Restriction versus acceptance remained too close to resolve honestly.",
            vec![(MergeSource::SideA, a), (MergeSource::SideB, b)],
        );
        return outcome(
            MergeResolution::ScopedDisagreement {
                residue: NonEmpty::new(residue),
            },
            trace(RuleId::ConflictSurvived, DecidedBy::Unresolved, vec![]),
            MergeSource::Both,
            base_epoch,
            base_trust,
        );
    }

    // Both permissive: a within-class choice, where informer and epoch
    // slack legitimately participate.
    if is_permissive(a.state) && is_permissive(b.state) {
        let score_a = within_class_score(policy, a);
        let score_b = within_class_score(policy, b);
        let (chosen, other, source, decided_by) =
            if a.epoch > b.epoch && score_a >= score_b - policy.permissive_epoch_slack {
                (a, b, MergeSource::SideA, DecidedBy::Freshness)
            } else if b.epoch > a.epoch && score_b >= score_a - policy.permissive_epoch_slack {
                (b, a, MergeSource::SideB, DecidedBy::Freshness)
            } else if score_a > score_b + policy.permissive_margin {
                (a, b, MergeSource::SideA, attribute(policy, a, b))
            } else if score_b > score_a + policy.permissive_margin {
                (b, a, MergeSource::SideB, attribute(policy, b, a))
            } else if a.state == BeliefState::Accepted && b.state != BeliefState::Accepted {
                (a, b, MergeSource::SideA, DecidedBy::StatusPreference)
            } else if b.state == BeliefState::Accepted && a.state != BeliefState::Accepted {
                (b, a, MergeSource::SideB, DecidedBy::StatusPreference)
            } else if score_a >= score_b {
                (a, b, MergeSource::SideA, DecidedBy::InputOrder)
            } else {
                (b, a, MergeSource::SideB, DecidedBy::InputOrder)
            };

        if chosen.state == BeliefState::Provisional && other.state == BeliefState::Accepted {
            let residue = sc.residue(
                "The older acceptance stays visible: recontact did not restore innocence.",
                vec![(MergeSource::SideA, a), (MergeSource::SideB, b)],
            );
            return outcome(
                MergeResolution::ProvisionalConverged {
                    state: BeliefState::Provisional,
                    decided_by: DecidedBy::Freshness,
                    residue: vec![residue],
                },
                trace(RuleId::FreshnessDowngrade, DecidedBy::Freshness, vec![]),
                source,
                base_epoch,
                chosen.trust,
            );
        }

        let stable = chosen.state == BeliefState::Accepted
            && within_class_score(policy, chosen)
                >= within_class_score(policy, other) + policy.accept_converge_margin;
        let resolution = if stable {
            MergeResolution::Converged {
                state: chosen.state,
                decided_by,
                residue: vec![],
            }
        } else {
            MergeResolution::ProvisionalConverged {
                state: chosen.state,
                decided_by,
                residue: vec![sc.residue(
                    "Both sides leaned the same direction, but one path stayed weak enough to keep the result provisional.",
                    vec![(MergeSource::SideA, a), (MergeSource::SideB, b)],
                )],
            }
        };
        return outcome(
            resolution,
            trace(RuleId::PermissiveConverged, decided_by, vec![]),
            source,
            base_epoch,
            chosen.trust,
        );
    }

    // Fallback for mixed shapes the corpus never produces. This is a
    // cross-class comparison, so it consumes dominance scores only —
    // lab.js used its flat score here, but the branch is unexercised by
    // the corpus, so the more principled count-free rule wins (documented
    // divergence). An exact dominance tie is broken by content, not input
    // order: the more conservative state wins, so the projection is
    // side-symmetric.
    let score_a = dominance_score(policy, a);
    let score_b = dominance_score(policy, b);
    let conservatism = |s: BeliefState| match s {
        BeliefState::Removed => 8,
        BeliefState::Quarantined => 7,
        BeliefState::Disputed => 6,
        BeliefState::Suspected => 5,
        BeliefState::Introduced => 4,
        BeliefState::Witnessed => 3,
        BeliefState::Provisional => 2,
        BeliefState::Accepted => 1,
        BeliefState::Unknown => 0,
    };
    let (winner, loser, source, decided_by) = if score_a != score_b {
        let a_wins = score_a > score_b;
        let (w, l, src) = if a_wins {
            (a, b, MergeSource::SideA)
        } else {
            (b, a, MergeSource::SideB)
        };
        (w, l, src, attribute_dominance(policy, w, l))
    } else if conservatism(a.state) >= conservatism(b.state) {
        (a, b, MergeSource::SideA, DecidedBy::StatusPreference)
    } else {
        (b, a, MergeSource::SideB, DecidedBy::StatusPreference)
    };
    let _ = loser;
    let residue = sc.residue(
        "Enough disagreement survived to keep a visible scar in the merged view.",
        vec![(MergeSource::SideA, a), (MergeSource::SideB, b)],
    );
    outcome(
        MergeResolution::ProvisionalConverged {
            state: winner.state,
            decided_by,
            residue: vec![residue],
        },
        trace(RuleId::Fallback, decided_by, vec![]),
        source,
        base_epoch,
        base_trust,
    )
}

/// Attribution restricted to dominance components — for cross-class
/// decisions, where the informer must not even appear in the explanation.
fn attribute_dominance(
    policy: &MergePolicy,
    winner: &BeliefFragment,
    loser: &BeliefFragment,
) -> DecidedBy {
    let d_trust = i64::from(winner.trust.get()) - i64::from(loser.trust.get());
    let d_corr = (policy.quality_score(winner.witness.quality)
        + policy.diversity_score(winner.witness.diversity))
        - (policy.quality_score(loser.witness.quality)
            + policy.diversity_score(loser.witness.diversity));
    let d_auth = policy.authority_score(winner.authority) - policy.authority_score(loser.authority);
    if d_corr.abs() > d_trust.abs() + d_auth.abs() {
        DecidedBy::Corroboration
    } else {
        DecidedBy::TrustAndAuthority
    }
}

fn input_digest(
    ctx: &ReunionContext,
    subjects: &[SubjectId],
    a: &MergeSide,
    b: &MergeSide,
) -> [u8; 32] {
    let mut bytes = Vec::new();
    ctx.scope.write_canonical(&mut bytes);
    ctx.epoch.get().write_canonical(&mut bytes);
    ctx.round.get().write_canonical(&mut bytes);
    for side in [a, b] {
        write_str(&mut bytes, &side.label);
        side.epoch.get().write_canonical(&mut bytes);
        for subject in subjects {
            subject.write_canonical(&mut bytes);
            if let Some(f) = side.fragments.get(subject) {
                write_str(&mut bytes, f.state.as_str());
                f.epoch.get().write_canonical(&mut bytes);
                bytes.push(f.trust.get());
                bytes.push(f.witness.quality as u8);
                bytes.push(f.witness.diversity as u8);
                (u64::from(f.witness.count)).write_canonical(&mut bytes);
            } else {
                bytes.push(0xff);
            }
        }
    }
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"resonant/merge-input/v1");
    hasher.update(&bytes);
    *hasher.finalize().as_bytes()
}

/// Deterministic reunion of two sides over an explicit subject list.
pub fn deterministic_reunion(
    policy: &MergePolicy,
    ctx: &ReunionContext,
    subjects: &[SubjectId],
    side_a: &MergeSide,
    side_b: &MergeSide,
    operator_override: Option<&OperatorOverride>,
) -> ReunionOutcome {
    let digest_input = input_digest(ctx, subjects, side_a, side_b);
    let mut outcomes: Vec<SubjectMergeOutcome> = subjects
        .iter()
        .map(|subject| {
            merge_subject(
                policy,
                ctx,
                subject,
                side_a.fragments.get(subject),
                side_b.fragments.get(subject),
            )
        })
        .collect();

    if let Some(op) = operator_override {
        if let Some(target) = outcomes.iter_mut().find(|o| o.subject == op.subject) {
            let override_id = op.id();
            let sc = SubjectContext {
                ctx,
                subject: &op.subject,
            };
            let prior = std::mem::replace(
                &mut target.resolution,
                MergeResolution::Converged {
                    state: BeliefState::Unknown,
                    decided_by: DecidedBy::Override,
                    residue: vec![],
                },
            );
            let mut residue: Vec<Residue> = match prior {
                MergeResolution::Converged { residue, .. }
                | MergeResolution::ProvisionalConverged { residue, .. }
                | MergeResolution::Overridden { residue, .. } => residue,
                MergeResolution::ScopedDisagreement { residue } => residue.into(),
            };
            if residue.is_empty() {
                residue.push(sc.residue("OperatorOverride forced a visible intervention.", vec![]));
            }
            for r in &mut residue {
                r.mark_handled(override_id);
            }
            target.resolution = MergeResolution::Overridden {
                override_id,
                forced: op.forced,
                residue,
            };
            target.source = MergeSource::Override;
            target.trace.rule = RuleId::OverrideApplied;
            target.trace.decided_by = DecidedBy::Override;
            target
                .trace
                .notes
                .push(format!("operator reason: {}", op.reason));
        }
    }

    let mut rules_fired: Vec<RuleId> = Vec::new();
    for o in &outcomes {
        if !rules_fired.contains(&o.trace.rule) {
            rules_fired.push(o.trace.rule);
        }
    }
    let unresolved_residue = outcomes
        .iter()
        .flat_map(|o| o.resolution.residue())
        .filter(|r| r.handled_by().is_none())
        .count();
    let handled_residue = outcomes
        .iter()
        .flat_map(|o| o.resolution.residue())
        .filter(|r| r.handled_by().is_some())
        .count();
    let any_provisional = outcomes.iter().any(|o| {
        o.resolution.stability() == MergeStability::Provisional
            || o.resolution.project() == BeliefState::Disputed
    });
    let overall = if operator_override.is_some() {
        if unresolved_residue == 0 {
            OverallOutcome::StableWithOverride
        } else {
            OverallOutcome::ProvisionalWithOverride
        }
    } else if unresolved_residue == 0 && !any_provisional {
        OverallOutcome::Stable
    } else {
        OverallOutcome::Provisional
    };

    ReunionOutcome {
        context: ctx.clone(),
        input_digest: digest_input,
        outcomes,
        digest: ReunionDigest {
            overall,
            compared_subjects: subjects.len(),
            rules_fired,
            unresolved_residue,
            handled_residue,
        },
    }
}

/// Result of applying a reunion to a view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppliedMerge {
    pub transitions: Vec<Transition>,
    pub confirmations: usize,
    pub residue_inserted: usize,
    pub residue_superseded: usize,
}

impl ReunionOutcome {
    /// The only consumption path: every projection goes through
    /// `BeliefCell::apply` (the merge cannot bypass the state machine) and
    /// every residue lands in the view's ledger (the merge cannot drop
    /// disagreement).
    pub fn apply_to(
        self,
        view: &mut MembershipView,
        policy: &PolicyBundle,
        at: (Epoch, Round),
    ) -> Result<AppliedMerge, TransitionError> {
        let mut applied = AppliedMerge {
            transitions: Vec::new(),
            confirmations: 0,
            residue_inserted: 0,
            residue_superseded: 0,
        };
        let input_digest = self.input_digest;
        for outcome in self.outcomes {
            let projected = outcome.resolution.project();
            let (event, residue): (BeliefEvent, Vec<Residue>) = match outcome.resolution {
                MergeResolution::Overridden {
                    override_id,
                    forced,
                    residue,
                } => (
                    BeliefEvent::OverrideApplied {
                        override_id,
                        to: forced,
                    },
                    residue,
                ),
                MergeResolution::Converged { residue, .. }
                | MergeResolution::ProvisionalConverged { residue, .. } => (
                    BeliefEvent::MergeProjected {
                        input_digest,
                        to: projected,
                    },
                    residue,
                ),
                MergeResolution::ScopedDisagreement { residue } => (
                    BeliefEvent::MergeProjected {
                        input_digest,
                        to: projected,
                    },
                    residue.into(),
                ),
            };
            let cell = view.cell_mut(outcome.subject.clone(), at);
            if projected == BeliefState::Unknown && cell.state() == BeliefState::Unknown {
                applied.confirmations += 1;
            } else {
                match cell.apply(event, at, policy)? {
                    Some(transition) => applied.transitions.push(transition),
                    None => applied.confirmations += 1,
                }
            }
            for r in residue {
                if view.residue_mut().insert(r).is_some() {
                    applied.residue_superseded += 1;
                }
                applied.residue_inserted += 1;
            }
        }
        view.advance_epoch(at.0);
        Ok(applied)
    }
}
