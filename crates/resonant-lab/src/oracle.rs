//! The lab.js merge engine, ported line-for-line.
//!
//! This module is the conformance oracle: a direct port of `mergeSubject`,
//! `runDeterministicMerge`, `runNaiveReunion`, `buildNaiveComparison`, and
//! `computeIslandDigest` from docs/deterministic-reunion-lab/lab.js,
//! string-exact on explanations and rule summaries. The kernel's tiered
//! merge engine is tested for structural equivalence against this port on
//! the whole corpus.
//!
//! Deliberate deviations from lab.js (all API-shape, none behavioral on
//! the corpus):
//! - the replayed step count is a parameter instead of global UI state;
//! - the naive-comparison null-dereference (lab.js reads
//!   `deterministicMember.residue` before its null guard) is unreachable
//!   here because of `Option`;
//! - ties resolved toward Island A by input order are *named* in the rule
//!   summary source field rather than left implicit — an accident of JS
//!   evaluation order turned into accountable determinism.

use crate::replay::CurrentState;
use crate::scenario::{MemberEntry, Scenario, Scope, Subject};
use resonant_kernel::belief::BeliefState;
use resonant_kernel::evidence::{Diversity, Quality};
use resonant_kernel::policy::MergePolicy;
use serde::Serialize;
use std::collections::BTreeMap;

pub const MEANINGLESS_SCORE: i64 = -999;

/// Where a merged belief came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Source {
    None,
    IslandA,
    IslandB,
    Both,
    Override,
}

impl Source {
    pub fn as_str(self) -> &'static str {
        match self {
            Source::None => "none",
            Source::IslandA => "island_a",
            Source::IslandB => "island_b",
            Source::Both => "both",
            Source::Override => "override",
        }
    }
}

/// Stability of a merged belief. Independent of residue: scenario 6's
/// epoch race merges `stable` yet still carries a visible scar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Stability {
    Stable,
    Provisional,
    StableWithOverride,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OracleResidue {
    pub subject_label: String,
    pub detail: String,
    pub handled_by_override: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MergedMember {
    pub subject_id: String,
    pub subject_label: String,
    pub source: Source,
    pub status: BeliefState,
    pub stability: Stability,
    pub epoch: u64,
    pub trust_weight: i64,
    pub explanation: String,
    pub rule_summary: Vec<String>,
    pub residue: Option<OracleResidue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OverallOutcome {
    Stable,
    Provisional,
    StableWithOverride,
    ProvisionalWithOverride,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MergeDigest {
    pub overall_outcome: OverallOutcome,
    pub compared_subjects: usize,
    pub inputs: Vec<String>,
    pub rules_fired: Vec<String>,
    pub unresolved: Vec<OracleResidue>,
    pub applied_override: Option<crate::scenario::OperatorOverride>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeterministicMerge {
    pub members: Vec<MergedMember>,
    pub residues: Vec<OracleResidue>,
    pub digest: MergeDigest,
}

pub fn is_meaningful(entry: Option<&MemberEntry>) -> bool {
    entry.is_some_and(|e| e.status != BeliefState::Unknown && e.trust_weight > 0)
}

pub fn is_restrictive(entry: &MemberEntry) -> bool {
    matches!(
        entry.status,
        BeliefState::Removed | BeliefState::Quarantined
    )
}

pub fn is_permissive(entry: &MemberEntry) -> bool {
    matches!(
        entry.status,
        BeliefState::Accepted | BeliefState::Provisional
    )
}

pub fn entry_score(policy: &MergePolicy, entry: Option<&MemberEntry>) -> i64 {
    if !is_meaningful(entry) {
        return MEANINGLESS_SCORE;
    }
    let e = entry.expect("meaningful implies present");
    let scope_score = match e.scope {
        Scope::Global => policy.scope_global,
        Scope::RegionalA | Scope::RegionalB => policy.scope_regional,
    };
    e.trust_weight
        + policy.quality_score(e.witness_summary.quality)
        + policy.diversity_score(e.witness_summary.diversity)
        + scope_score
        + policy.informer_score(e.witness_summary.count)
}

enum PermissiveChoice {
    A,
    B,
}

fn choose_permissive(policy: &MergePolicy, a: &MemberEntry, b: &MemberEntry) -> PermissiveChoice {
    let score_a = entry_score(policy, Some(a));
    let score_b = entry_score(policy, Some(b));
    if a.epoch > b.epoch && score_a >= score_b - policy.permissive_epoch_slack {
        return PermissiveChoice::A;
    }
    if b.epoch > a.epoch && score_b >= score_a - policy.permissive_epoch_slack {
        return PermissiveChoice::B;
    }
    if score_a > score_b + policy.permissive_margin {
        return PermissiveChoice::A;
    }
    if score_b > score_a + policy.permissive_margin {
        return PermissiveChoice::B;
    }
    if a.status == BeliefState::Accepted && b.status != BeliefState::Accepted {
        return PermissiveChoice::A;
    }
    if b.status == BeliefState::Accepted && a.status != BeliefState::Accepted {
        return PermissiveChoice::B;
    }
    // Tie resolved toward Island A by deterministic input order.
    if score_a >= score_b {
        PermissiveChoice::A
    } else {
        PermissiveChoice::B
    }
}

pub fn merge_subject(
    policy: &MergePolicy,
    subject: &Subject,
    entry_a: Option<&MemberEntry>,
    entry_b: Option<&MemberEntry>,
) -> MergedMember {
    let has_a = is_meaningful(entry_a);
    let has_b = is_meaningful(entry_b);
    let score_a = entry_score(policy, entry_a);
    let score_b = entry_score(policy, entry_b);
    let mut result = MergedMember {
        subject_id: subject.id.clone(),
        subject_label: subject.label.clone(),
        source: Source::None,
        status: BeliefState::Unknown,
        stability: Stability::Stable,
        epoch: entry_a
            .map_or(0, |e| e.epoch)
            .max(entry_b.map_or(0, |e| e.epoch)),
        trust_weight: entry_a
            .map_or(0, |e| e.trust_weight)
            .max(entry_b.map_or(0, |e| e.trust_weight)),
        explanation: "No admissible view survived the reunion.".into(),
        rule_summary: Vec::new(),
        residue: None,
    };

    if !has_a && !has_b {
        result
            .rule_summary
            .push("No admissible input survived for this subject.".into());
        return result;
    }

    // One-sided admissible evidence.
    if has_a != has_b {
        let (entry, source, island) = if has_a {
            (entry_a.unwrap(), Source::IslandA, "Island A")
        } else {
            (entry_b.unwrap(), Source::IslandB, "Island B")
        };
        result.source = source;
        result.status = entry.status;
        result.stability = if entry_score(policy, Some(entry)) >= policy.one_sided_stable
            && entry.status != BeliefState::Provisional
        {
            Stability::Stable
        } else {
            Stability::Provisional
        };
        result.epoch = entry.epoch;
        result.trust_weight = entry.trust_weight;
        result.explanation = format!("Only {island} carried an admissible path into reunion.");
        result
            .rule_summary
            .push("Single-sided admissible state carried forward.".into());
        if result.stability == Stability::Provisional {
            result.residue = Some(OracleResidue {
                subject_label: subject.label.clone(),
                detail: "Only one island had enough evidence to speak. Reunion preserves that asymmetry instead of pretending shared certainty.".into(),
                handled_by_override: false,
            });
        }
        return result;
    }

    let a = entry_a.unwrap();
    let b = entry_b.unwrap();

    if a.status == b.status {
        result.source = Source::Both;
        result.status = a.status;
        result.stability = if a.status == BeliefState::Provisional {
            Stability::Provisional
        } else {
            Stability::Stable
        };
        result.epoch = a.epoch.max(b.epoch);
        result.trust_weight = a.trust_weight.max(b.trust_weight);
        result.explanation = format!("Both islands converge on {}.", a.status);
        result
            .rule_summary
            .push("Matching local outcomes converge cleanly.".into());
        result.rule_summary.push(
            if score_a >= score_b {
                "Island A supplied the stronger retained detail."
            } else {
                "Island B supplied the stronger retained detail."
            }
            .into(),
        );
        return result;
    }

    let laundering_a = a.witness_summary.diversity == Diversity::Laundered;
    let laundering_b = b.witness_summary.diversity == Diversity::Laundered;

    if laundering_a || laundering_b {
        // lab.js quirk, reproduced: if both sides were laundered, Island A
        // is arbitrarily treated as the laundered one. Unexercised by the
        // corpus.
        let (laundered, laundered_source, other, other_source, other_island) = if laundering_a {
            (a, "Island A", b, Source::IslandB, "Island B")
        } else {
            (b, "Island B", a, Source::IslandA, "Island A")
        };
        if entry_score(policy, Some(other))
            >= entry_score(policy, Some(laundered)) - policy.laundering_tolerance
        {
            result.source = other_source;
            result.status = if other.status == BeliefState::Accepted {
                BeliefState::Provisional
            } else {
                other.status
            };
            result.stability = Stability::Provisional;
            result.epoch = other.epoch.max(laundered.epoch);
            result.trust_weight = other.trust_weight;
            result.explanation =
                "Low-quality reinforcing witnesses were discounted during reunion.".into();
            result
                .rule_summary
                .push("Trust and corroboration quality dominated raw witness count.".into());
            result.rule_summary.push(format!(
                "{other_island} preserved the more admissible path."
            ));
            result.residue = Some(OracleResidue {
                subject_label: subject.label.clone(),
                detail: format!("{laundered_source} accumulated a louder but lower-quality witness cluster. The merge retains that laundering attempt as visible residue."),
                handled_by_override: false,
            });
            return result;
        }
    }

    if is_restrictive(a) || is_restrictive(b) {
        // lab.js quirk, reproduced: if both sides are restrictive with
        // differing statuses, side B is mislabeled "permissive".
        // Unexercised by the corpus.
        let (restrictive, permissive, restrictive_source, permissive_island) = if is_restrictive(a)
        {
            (a, b, Source::IslandA, "Island B")
        } else {
            (b, a, Source::IslandB, "Island A")
        };

        if restrictive.epoch == permissive.epoch
            && restrictive.witness_summary.quality != Quality::Weak
            && permissive.witness_summary.quality != Quality::Weak
            && (score_a - score_b).abs() < policy.dispute_closeness
        {
            result.source = Source::Both;
            result.status = BeliefState::Disputed;
            result.stability = Stability::Provisional;
            result.explanation = "Fresh admissible evidence remains materially unresolved after the allowed dominance checks.".into();
            result
                .rule_summary
                .push("Same-epoch high-trust conflict survives as residue.".into());
            result.residue = Some(OracleResidue {
                subject_label: subject.label.clone(),
                detail: format!("Island A says {}. Island B says {}. Both paths remain fresh enough that deterministic reunion refuses to counterfeit certainty.", a.status, b.status),
                handled_by_override: false,
            });
            return result;
        }

        if restrictive.epoch > permissive.epoch
            && entry_score(policy, Some(restrictive))
                >= entry_score(policy, Some(permissive)) - policy.restrictive_fresh_slack
        {
            result.source = restrictive_source;
            result.status = restrictive.status;
            result.stability = Stability::Stable;
            result.epoch = restrictive.epoch;
            result.trust_weight = restrictive.trust_weight;
            result.explanation =
                "Newer restrictive evidence dominates the older permissive path.".into();
            result
                .rule_summary
                .push("Fresh restrictive evidence dominated older permissive evidence.".into());
            result.residue = Some(OracleResidue {
                subject_label: subject.label.clone(),
                detail: format!("{permissive_island} still carried a coherent permissive path. The race remains visible even though the newer restrictive path wins."),
                handled_by_override: false,
            });
            return result;
        }

        if entry_score(policy, Some(restrictive))
            >= entry_score(policy, Some(permissive)) + policy.restrictive_dominance
        {
            result.source = restrictive_source;
            result.status = restrictive.status;
            result.stability = Stability::Provisional;
            result.epoch = restrictive.epoch;
            result.trust_weight = restrictive.trust_weight;
            result.explanation = "Restrictive evidence dominates on trust and admissibility, but the opposing path remains visible.".into();
            result
                .rule_summary
                .push("Trust and scope authority favored the restrictive path.".into());
            result.residue = Some(OracleResidue {
                subject_label: subject.label.clone(),
                detail: "The merge chooses the restrictive path, but it keeps the opposing history as residue rather than pretending the split never happened.".into(),
                handled_by_override: false,
            });
            return result;
        }

        result.source = Source::Both;
        result.status = BeliefState::Disputed;
        result.stability = Stability::Provisional;
        result.explanation =
            "Restriction versus acceptance remained too close to resolve honestly.".into();
        result
            .rule_summary
            .push("Conflict survived the deterministic reunion pass.".into());
        result.residue = Some(OracleResidue {
            subject_label: subject.label.clone(),
            detail: format!("Restriction and acceptance remained materially close for {}. The conflict stays visible.", subject.label),
            handled_by_override: false,
        });
        return result;
    }

    if is_permissive(a) && is_permissive(b) {
        let choice = choose_permissive(policy, a, b);
        let (chosen, other, source) = match choice {
            PermissiveChoice::A => (a, b, Source::IslandA),
            PermissiveChoice::B => (b, a, Source::IslandB),
        };
        result.source = source;
        result.status = chosen.status;
        result.epoch = a.epoch.max(b.epoch);
        result.trust_weight = chosen.trust_weight;

        if chosen.status == BeliefState::Provisional && other.status == BeliefState::Accepted {
            result.stability = Stability::Provisional;
            result.explanation =
                "Fresher admissible evidence downgrades the older acceptance path.".into();
            result
                .rule_summary
                .push("Freshness dominated older permissive acceptance.".into());
            result.residue = Some(OracleResidue {
                subject_label: subject.label.clone(),
                detail: format!("{} on the older path remains visible as residue, because recontact did not restore innocence.", other.status),
                handled_by_override: false,
            });
            return result;
        }

        result.stability = if chosen.status == BeliefState::Accepted
            && entry_score(policy, Some(chosen))
                >= entry_score(policy, Some(other)) + policy.accept_converge_margin
        {
            Stability::Stable
        } else {
            Stability::Provisional
        };
        result.explanation =
            "Both islands lean permissive, and the stronger path closes the gap.".into();
        result
            .rule_summary
            .push("Permissive paths converged under freshness and trust weighting.".into());
        if result.stability == Stability::Provisional {
            result.residue = Some(OracleResidue {
                subject_label: subject.label.clone(),
                detail: "The two islands leaned the same direction, but one path remained weaker enough that the merged result stays provisional.".into(),
                handled_by_override: false,
            });
        }
        return result;
    }

    // Fallback: mixed shapes not caught above (unreachable from corpus
    // inputs — kept for parity with lab.js).
    let a_wins = score_a >= score_b;
    result.source = if a_wins {
        Source::IslandA
    } else {
        Source::IslandB
    };
    result.status = if a_wins { a.status } else { b.status };
    result.stability = Stability::Provisional;
    result.epoch = a.epoch.max(b.epoch);
    result.trust_weight = a.trust_weight.max(b.trust_weight);
    result.explanation =
        "The reunion retained the stronger path, but not without visible caution.".into();
    result
        .rule_summary
        .push("Fallback deterministic comparison retained the stronger admissible path.".into());
    result.residue = Some(OracleResidue {
        subject_label: subject.label.clone(),
        detail: "This subject still carried enough disagreement to keep a visible scar in the merged view.".into(),
        handled_by_override: false,
    });
    result
}

/// Whole-view deterministic reunion, with optional operator override.
pub fn run_deterministic_merge(
    policy: &MergePolicy,
    scenario: &Scenario,
    current: &CurrentState,
    apply_override: bool,
) -> DeterministicMerge {
    let mut merged: Vec<MergedMember> = scenario
        .subjects
        .iter()
        .map(|subject| {
            merge_subject(
                policy,
                subject,
                current.island_a.members.get(&subject.id),
                current.island_b.members.get(&subject.id),
            )
        })
        .collect();

    if apply_override && scenario.allow_operator_override {
        if let Some(op) = &scenario.operator_override {
            if let Some(target) = merged.iter_mut().find(|m| m.subject_id == op.subject_id) {
                target.status = op.status;
                target.source = Source::Override;
                target.stability = Stability::StableWithOverride;
                target.explanation = op.reason.clone();
                target
                    .rule_summary
                    .push("OperatorOverride applied visibly.".into());
                match &mut target.residue {
                    Some(residue) => {
                        residue.handled_by_override = true;
                        residue.detail.push_str(
                            " OperatorOverride forced a visible intervention instead of silent collapse.",
                        );
                    }
                    None => {
                        target.residue = Some(OracleResidue {
                            subject_label: target.subject_label.clone(),
                            detail: "OperatorOverride forced a visible intervention.".into(),
                            handled_by_override: true,
                        });
                    }
                }
            }
        }
    }

    let residues: Vec<OracleResidue> = merged.iter().filter_map(|m| m.residue.clone()).collect();
    let unresolved: Vec<OracleResidue> = residues
        .iter()
        .filter(|r| !r.handled_by_override)
        .cloned()
        .collect();

    // Distinct rules in first-appearance order (JS Set semantics).
    let mut rules_fired: Vec<String> = Vec::new();
    for member in &merged {
        for rule in &member.rule_summary {
            if !rules_fired.contains(rule) {
                rules_fired.push(rule.clone());
            }
        }
    }

    let any_provisional = merged
        .iter()
        .any(|m| m.stability == Stability::Provisional || m.status == BeliefState::Disputed);
    let overall_outcome = if apply_override {
        if unresolved.is_empty() {
            OverallOutcome::StableWithOverride
        } else {
            OverallOutcome::ProvisionalWithOverride
        }
    } else if unresolved.is_empty() && !any_provisional {
        OverallOutcome::Stable
    } else {
        OverallOutcome::Provisional
    };

    let digest = MergeDigest {
        overall_outcome,
        compared_subjects: scenario.subjects.len(),
        inputs: vec![
            format!("Island A epoch {}", current.island_a.local_epoch),
            format!("Island B epoch {}", current.island_b.local_epoch),
            format!("{} divergence event(s) replayed", current.steps_replayed),
        ],
        rules_fired,
        unresolved,
        applied_override: if apply_override {
            scenario.operator_override.clone()
        } else {
            None
        },
    };

    DeterministicMerge {
        members: merged,
        residues,
        digest,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NaiveMember {
    pub subject_label: String,
    pub status: BeliefState,
    pub source: String,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NaiveDiff {
    pub subject_label: String,
    pub naive_status: BeliefState,
    pub deterministic_status: BeliefState,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NaiveComparison {
    pub naive: Vec<NaiveMember>,
    pub diffs: Vec<NaiveDiff>,
}

fn status_order(status: BeliefState) -> i64 {
    match status {
        BeliefState::Accepted => 5,
        BeliefState::Provisional => 4,
        BeliefState::Quarantined => 3,
        BeliefState::Removed => 2,
        BeliefState::Disputed => 1,
        _ => 0,
    }
}

/// "Latest or loudest wins": the strawman reunion the deterministic merge
/// is compared against.
pub fn run_naive_reunion(scenario: &Scenario, current: &CurrentState) -> Vec<NaiveMember> {
    scenario
        .subjects
        .iter()
        .map(|subject| {
            let entry_a = current.island_a.members.get(&subject.id);
            let entry_b = current.island_b.members.get(&subject.id);
            let mut candidates: Vec<(&str, &MemberEntry)> = Vec::new();
            if is_meaningful(entry_a) {
                candidates.push(("Island A", entry_a.unwrap()));
            }
            if is_meaningful(entry_b) {
                candidates.push(("Island B", entry_b.unwrap()));
            }
            if candidates.is_empty() {
                return NaiveMember {
                    subject_label: subject.label.clone(),
                    status: BeliefState::Unknown,
                    source: "none".into(),
                    note: "No admissible input.".into(),
                };
            }
            // Stable sort, descending (epoch, count, status order, trust):
            // ties keep input order, i.e. favor Island A — reproducing
            // JS Array.sort on a two-element array.
            candidates.sort_by(|(_, l), (_, r)| {
                r.epoch
                    .cmp(&l.epoch)
                    .then(r.witness_summary.count.cmp(&l.witness_summary.count))
                    .then(status_order(r.status).cmp(&status_order(l.status)))
                    .then(r.trust_weight.cmp(&l.trust_weight))
            });
            let (source, chosen) = candidates[0];
            NaiveMember {
                subject_label: subject.label.clone(),
                status: chosen.status,
                source: source.into(),
                note: "Latest or loudest path wins. Residue is discarded.".into(),
            }
        })
        .collect()
}

pub fn build_naive_comparison(
    scenario: &Scenario,
    current: &CurrentState,
    deterministic: &DeterministicMerge,
) -> NaiveComparison {
    let naive = run_naive_reunion(scenario, current);
    let diffs = scenario
        .subjects
        .iter()
        .filter_map(|subject| {
            let det = deterministic
                .members
                .iter()
                .find(|m| m.subject_id == subject.id)?;
            let nai = naive.iter().find(|m| m.subject_label == subject.label)?;
            let hides_residue = det.residue.as_ref().is_some_and(|r| !r.handled_by_override);
            if nai.status != det.status || hides_residue {
                Some(NaiveDiff {
                    subject_label: subject.label.clone(),
                    naive_status: nai.status,
                    deterministic_status: det.status,
                    note: if hides_residue {
                        "Naive reunion would erase visible residue.".into()
                    } else {
                        format!(
                            "Naive reunion would choose {} from {}.",
                            nai.status, nai.source
                        )
                    },
                })
            } else {
                None
            }
        })
        .collect();

    NaiveComparison { naive, diffs }
}

/// Per-island status counts, for display parity with the lab page.
pub fn compute_island_digest(island: &crate::scenario::IslandView) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for member in island.members.values() {
        *counts
            .entry(member.status.as_str().to_string())
            .or_insert(0) += 1;
    }
    counts
}
