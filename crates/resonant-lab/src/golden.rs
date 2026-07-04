//! Hand-derived golden outcomes for the scenario corpus.
//!
//! Each expectation was derived by hand-executing lab.js's merge semantics
//! over the scenario JSONs (entry scores noted inline so a failing diff is
//! self-explanatory). Shared by the conformance tests and by
//! `resonant scenario verify`.

use crate::oracle::{run_deterministic_merge, OverallOutcome, Source, Stability};
use crate::replay::materialize;
use crate::scenario::Scenario;
use resonant_kernel::belief::BeliefState;
use resonant_kernel::policy::MergePolicy;

pub struct GoldenSubject {
    pub subject_id: &'static str,
    pub status: BeliefState,
    pub source: Source,
    pub stability: Stability,
    pub has_residue: bool,
}

pub struct GoldenCase {
    pub scenario_id: &'static str,
    pub apply_override: bool,
    pub overall: OverallOutcome,
    pub unresolved_count: usize,
    pub subjects: &'static [GoldenSubject],
}

macro_rules! subjects {
    ($($id:literal => $status:ident / $source:ident / $stability:ident / $residue:literal),+ $(,)?) => {
        &[$(GoldenSubject {
            subject_id: $id,
            status: BeliefState::$status,
            source: Source::$source,
            stability: Stability::$stability,
            has_residue: $residue,
        }),+]
    };
}

/// The golden table, all cases at full replay.
pub fn golden_cases() -> Vec<GoldenCase> {
    vec![
        // 1. Clean reunion: every subject converges accepted/both/stable.
        GoldenCase {
            scenario_id: "deterministic-reunion-clean",
            apply_override: false,
            overall: OverallOutcome::Stable,
            unresolved_count: 0,
            subjects: subjects![
                "iona" => Accepted/Both/Stable/false,    // A 140 vs B 141
                "kestrel" => Accepted/Both/Stable/false, // A 138 vs B 136
                "lumen" => Accepted/Both/Stable/false,   // A 139 vs B 137 (after close-lumen-gap)
                "morrow" => Accepted/Both/Stable/false,  // A 119 vs B 120
                "nyx" => Accepted/Both/Stable/false,     // A 101 vs B 136 (after freshen-nyx)
            ],
        },
        // 2. Stale witness: fresher provisional (A, e42, score 126) beats
        // stale acceptance (B, e40, score 60); recontact does not restore
        // innocence.
        GoldenCase {
            scenario_id: "deterministic-reunion-stale-witness",
            apply_override: false,
            overall: OverallOutcome::Provisional,
            unresolved_count: 1,
            subjects: subjects![
                "iona" => Accepted/Both/Stable/false,
                "kestrel" => Accepted/Both/Stable/false,
                "lumen" => Accepted/Both/Stable/false,
                "morrow" => Provisional/IslandA/Provisional/true,
                "nyx" => Accepted/Both/Stable/false,
            ],
        },
        // 3. Conflicting acceptance: same-epoch accepted (A, 140) vs
        // quarantined (B, 138), delta 2 < 14 -> honest dispute.
        GoldenCase {
            scenario_id: "deterministic-reunion-conflicting-acceptance",
            apply_override: false,
            overall: OverallOutcome::Provisional,
            unresolved_count: 1,
            subjects: subjects![
                "iona" => Accepted/Both/Stable/false,
                "kestrel" => Accepted/Both/Stable/false,
                "lumen" => Accepted/Both/Stable/false,
                "morrow" => Accepted/Both/Stable/false,
                "nyx" => Disputed/Both/Provisional/true,
            ],
        },
        // 4. Trust laundering: 2 strong cross-scope witnesses (A, 112)
        // beat 6 laundered ones (B, 47) — quality dominates count.
        GoldenCase {
            scenario_id: "deterministic-reunion-trust-laundering",
            apply_override: false,
            overall: OverallOutcome::Provisional,
            unresolved_count: 1,
            subjects: subjects![
                "iona" => Accepted/Both/Stable/false,
                "kestrel" => Accepted/Both/Stable/false,
                "lumen" => Provisional/IslandA/Provisional/true,
                "morrow" => Accepted/Both/Stable/false,
                "nyx" => Accepted/Both/Stable/false,
            ],
        },
        // 5a. Operator override scenario without the override: regional
        // acceptance (A, 117) vs global removal (B, 111) at the same
        // epoch, delta 6 < 14 -> dispute.
        GoldenCase {
            scenario_id: "deterministic-reunion-operator-override",
            apply_override: false,
            overall: OverallOutcome::Provisional,
            unresolved_count: 1,
            subjects: subjects![
                "iona" => Accepted/Both/Stable/false,
                "kestrel" => Disputed/Both/Provisional/true,
                "lumen" => Accepted/Both/Stable/false,
                "morrow" => Accepted/Both/Stable/false,
                "nyx" => Accepted/Both/Stable/false,
            ],
        },
        // 5b. With the override: quarantine forced visibly; the residue is
        // marked handled, never erased.
        GoldenCase {
            scenario_id: "deterministic-reunion-operator-override",
            apply_override: true,
            overall: OverallOutcome::StableWithOverride,
            unresolved_count: 0,
            subjects: subjects![
                "iona" => Accepted/Both/Stable/false,
                "kestrel" => Quarantined/Override/StableWithOverride/true,
                "lumen" => Accepted/Both/Stable/false,
                "morrow" => Accepted/Both/Stable/false,
                "nyx" => Accepted/Both/Stable/false,
            ],
        },
        // 6. Epoch race: newer removal (A, e84, 139) dominates older
        // acceptance (B, e83, 136 >= 139-6) -> stable *with* a visible
        // race scar. Stability and residue are independent axes.
        GoldenCase {
            scenario_id: "deterministic-reunion-epoch-race",
            apply_override: false,
            overall: OverallOutcome::Provisional,
            unresolved_count: 1,
            subjects: subjects![
                "iona" => Removed/IslandA/Stable/true,
                "kestrel" => Accepted/Both/Stable/false,
                "lumen" => Accepted/Both/Stable/false,
                "morrow" => Accepted/Both/Stable/false,
                "nyx" => Accepted/Both/Stable/false,
            ],
        },
    ]
}

/// Failures found when checking one golden case against an engine's output.
pub struct CaseReport {
    pub scenario_id: String,
    pub apply_override: bool,
    pub failures: Vec<String>,
}

impl CaseReport {
    pub fn passed(&self) -> bool {
        self.failures.is_empty()
    }
}

/// Run the oracle at full replay for one golden case and compare.
pub fn check_case(policy: &MergePolicy, scenario: &Scenario, case: &GoldenCase) -> CaseReport {
    let current = materialize(scenario, scenario.events.len());
    let merge = run_deterministic_merge(policy, scenario, &current, case.apply_override);
    let mut failures = Vec::new();

    if merge.digest.overall_outcome != case.overall {
        failures.push(format!(
            "overall outcome: expected {:?}, got {:?}",
            case.overall, merge.digest.overall_outcome
        ));
    }
    if merge.digest.unresolved.len() != case.unresolved_count {
        failures.push(format!(
            "unresolved residue: expected {}, got {}",
            case.unresolved_count,
            merge.digest.unresolved.len()
        ));
    }
    for expected in case.subjects {
        let Some(member) = merge
            .members
            .iter()
            .find(|m| m.subject_id == expected.subject_id)
        else {
            failures.push(format!(
                "subject {} missing from merge output",
                expected.subject_id
            ));
            continue;
        };
        if member.status != expected.status {
            failures.push(format!(
                "{}: status expected {}, got {}",
                expected.subject_id, expected.status, member.status
            ));
        }
        if member.source != expected.source {
            failures.push(format!(
                "{}: source expected {:?}, got {:?}",
                expected.subject_id, expected.source, member.source
            ));
        }
        if member.stability != expected.stability {
            failures.push(format!(
                "{}: stability expected {:?}, got {:?}",
                expected.subject_id, expected.stability, member.stability
            ));
        }
        if member.residue.is_some() != expected.has_residue {
            failures.push(format!(
                "{}: residue expected {}, got {}",
                expected.subject_id,
                expected.has_residue,
                member.residue.is_some()
            ));
        }
    }

    CaseReport {
        scenario_id: case.scenario_id.to_string(),
        apply_override: case.apply_override,
        failures,
    }
}
