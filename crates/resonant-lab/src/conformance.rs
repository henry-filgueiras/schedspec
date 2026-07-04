//! Adapter between the scenario corpus and the kernel's merge engine,
//! plus the structural-equivalence checks between kernel and oracle.

use crate::oracle;
use crate::replay::CurrentState;
use crate::scenario::{IslandView, MemberEntry, Scenario, Scope};
use resonant_kernel::epoch::{Epoch, Round};
use resonant_kernel::evidence::WitnessSummary;
use resonant_kernel::id::{OperatorId, SubjectId};
use resonant_kernel::merge::engine::{
    deterministic_reunion, MergeSide, ReunionContext, ReunionOutcome,
};
use resonant_kernel::merge::{BeliefFragment, MergeSource, MergeStability, OverallOutcome};
use resonant_kernel::operator::OperatorOverride;
use resonant_kernel::policy::MergePolicy;
use resonant_kernel::scope::{ScopeAuthority, ScopeId};
use resonant_kernel::trust::TrustGrade;
use std::collections::BTreeMap;

pub fn to_fragment(entry: &MemberEntry) -> BeliefFragment {
    BeliefFragment {
        state: entry.status,
        epoch: Epoch(entry.epoch),
        trust: TrustGrade::new(entry.trust_weight.clamp(0, 100) as u8),
        authority: match entry.scope {
            Scope::Global => ScopeAuthority::Global,
            Scope::RegionalA | Scope::RegionalB => ScopeAuthority::Regional,
        },
        witness: WitnessSummary {
            count: entry.witness_summary.count,
            quality: entry.witness_summary.quality,
            diversity: entry.witness_summary.diversity,
        },
    }
}

pub fn to_side(island: &IslandView) -> MergeSide {
    MergeSide {
        label: island.label.clone(),
        epoch: Epoch(island.local_epoch),
        fragments: island
            .members
            .iter()
            .map(|(id, entry)| (SubjectId::new(id.clone()), to_fragment(entry)))
            .collect::<BTreeMap<_, _>>(),
    }
}

pub fn to_override(op: &crate::scenario::OperatorOverride, epoch: Epoch) -> OperatorOverride {
    OperatorOverride {
        operator: OperatorId::new("scenario-operator"),
        subject: SubjectId::new(op.subject_id.clone()),
        forced: op.status,
        reason: op.reason.clone(),
        epoch,
    }
}

/// Run the kernel's deterministic reunion over a materialized scenario
/// state.
pub fn run_kernel_reunion(
    policy: &MergePolicy,
    scenario: &Scenario,
    current: &CurrentState,
    apply_override: bool,
) -> ReunionOutcome {
    let ctx = ReunionContext {
        scope: ScopeId::new("reunion"),
        epoch: Epoch(
            current
                .island_a
                .local_epoch
                .max(current.island_b.local_epoch),
        ),
        round: Round(current.steps_replayed as u64),
    };
    let subjects: Vec<SubjectId> = scenario
        .subjects
        .iter()
        .map(|s| SubjectId::new(s.id.clone()))
        .collect();
    let side_a = to_side(&current.island_a);
    let side_b = to_side(&current.island_b);
    let op = if apply_override && scenario.allow_operator_override {
        scenario
            .operator_override
            .as_ref()
            .map(|o| to_override(o, ctx.epoch))
    } else {
        None
    };
    deterministic_reunion(policy, &ctx, &subjects, &side_a, &side_b, op.as_ref())
}

fn source_matches(kernel: MergeSource, oracle: oracle::Source) -> bool {
    matches!(
        (kernel, oracle),
        (MergeSource::None, oracle::Source::None)
            | (MergeSource::SideA, oracle::Source::IslandA)
            | (MergeSource::SideB, oracle::Source::IslandB)
            | (MergeSource::Both, oracle::Source::Both)
            | (MergeSource::Override, oracle::Source::Override)
    )
}

fn stability_matches(kernel: MergeStability, oracle: oracle::Stability) -> bool {
    matches!(
        (kernel, oracle),
        (MergeStability::Stable, oracle::Stability::Stable)
            | (MergeStability::Provisional, oracle::Stability::Provisional)
            | (
                MergeStability::StableWithOverride,
                oracle::Stability::StableWithOverride
            )
    )
}

fn overall_matches(kernel: OverallOutcome, oracle: oracle::OverallOutcome) -> bool {
    matches!(
        (kernel, oracle),
        (OverallOutcome::Stable, oracle::OverallOutcome::Stable)
            | (
                OverallOutcome::Provisional,
                oracle::OverallOutcome::Provisional
            )
            | (
                OverallOutcome::StableWithOverride,
                oracle::OverallOutcome::StableWithOverride
            )
            | (
                OverallOutcome::ProvisionalWithOverride,
                oracle::OverallOutcome::ProvisionalWithOverride
            )
    )
}

/// Compare the kernel's reunion against the oracle's merge, structurally:
/// per-subject state, source, stability, epoch, trust, residue presence and
/// handled flag, plus overall outcome and unresolved count. Explanation
/// strings are the oracle's own surface and are not compared.
pub fn compare(kernel: &ReunionOutcome, oracle_merge: &oracle::DeterministicMerge) -> Vec<String> {
    let mut failures = Vec::new();

    for member in &oracle_merge.members {
        let Some(outcome) = kernel
            .outcomes
            .iter()
            .find(|o| o.subject.as_str() == member.subject_id)
        else {
            failures.push(format!("{}: missing from kernel output", member.subject_id));
            continue;
        };
        let id = &member.subject_id;
        if outcome.resolution.project() != member.status {
            failures.push(format!(
                "{id}: state {} vs oracle {}",
                outcome.resolution.project(),
                member.status
            ));
        }
        if !source_matches(outcome.source, member.source) {
            failures.push(format!(
                "{id}: source {:?} vs oracle {:?}",
                outcome.source, member.source
            ));
        }
        if !stability_matches(outcome.resolution.stability(), member.stability) {
            failures.push(format!(
                "{id}: stability {:?} vs oracle {:?}",
                outcome.resolution.stability(),
                member.stability
            ));
        }
        if outcome.epoch.get() != member.epoch {
            failures.push(format!(
                "{id}: epoch {} vs oracle {}",
                outcome.epoch, member.epoch
            ));
        }
        if i64::from(outcome.trust.get()) != member.trust_weight {
            failures.push(format!(
                "{id}: trust {} vs oracle {}",
                outcome.trust.get(),
                member.trust_weight
            ));
        }
        let kernel_residue = outcome.resolution.residue();
        let oracle_residue = member.residue.as_ref();
        if kernel_residue.is_empty() != oracle_residue.is_none() {
            failures.push(format!(
                "{id}: residue presence {} vs oracle {}",
                !kernel_residue.is_empty(),
                oracle_residue.is_some()
            ));
        }
        if let Some(or) = oracle_residue {
            let kernel_handled = kernel_residue.iter().all(|r| r.handled_by().is_some())
                && !kernel_residue.is_empty();
            if kernel_handled != or.handled_by_override {
                failures.push(format!(
                    "{id}: residue handled {} vs oracle {}",
                    kernel_handled, or.handled_by_override
                ));
            }
        }
    }

    if !overall_matches(kernel.digest.overall, oracle_merge.digest.overall_outcome) {
        failures.push(format!(
            "overall {:?} vs oracle {:?}",
            kernel.digest.overall, oracle_merge.digest.overall_outcome
        ));
    }
    if kernel.digest.unresolved_residue != oracle_merge.digest.unresolved.len() {
        failures.push(format!(
            "unresolved {} vs oracle {}",
            kernel.digest.unresolved_residue,
            oracle_merge.digest.unresolved.len()
        ));
    }

    failures
}
