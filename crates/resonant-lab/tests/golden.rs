//! Layer 2: golden outcomes. The oracle (lab.js port) must reproduce the
//! hand-derived expected outcome table on every scenario at full replay.

use resonant_kernel::policy::MergePolicy;
use resonant_lab::golden::{check_case, golden_cases};
use resonant_lab::oracle::{build_naive_comparison, run_deterministic_merge};
use resonant_lab::replay::materialize;
use resonant_lab::scenario::{default_scenarios_dir, load_corpus};

#[test]
fn oracle_matches_golden_table() {
    let policy = MergePolicy::lab_compat();
    let corpus = load_corpus(&default_scenarios_dir()).expect("corpus loads");

    let mut checked = 0;
    for case in golden_cases() {
        let scenario = corpus
            .iter()
            .find(|s| s.id == case.scenario_id)
            .unwrap_or_else(|| panic!("scenario {} in corpus", case.scenario_id));
        let report = check_case(&policy, scenario, &case);
        assert!(
            report.passed(),
            "{} (override: {}):\n  {}",
            report.scenario_id,
            report.apply_override,
            report.failures.join("\n  ")
        );
        checked += 1;
    }
    assert_eq!(checked, 7, "six scenarios plus the override variant");
}

/// Layer 4: naive-vs-deterministic diffs. The flagship demo is
/// trust-laundering, where naive reunion follows the six laundered
/// witnesses while the deterministic merge keeps the two strong ones.
#[test]
fn naive_comparison_diffs() {
    let policy = MergePolicy::lab_compat();
    let corpus = load_corpus(&default_scenarios_dir()).expect("corpus loads");

    // scenario id -> (diff subjects, all-erase-residue-notes?)
    let expectations: &[(&str, &[&str])] = &[
        ("deterministic-reunion-clean", &[]),
        ("deterministic-reunion-stale-witness", &["Morrow"]),
        ("deterministic-reunion-conflicting-acceptance", &["Nyx"]),
        ("deterministic-reunion-trust-laundering", &["Lumen"]),
        ("deterministic-reunion-operator-override", &["Kestrel"]),
        ("deterministic-reunion-epoch-race", &["Iona"]),
    ];

    for (id, expected_subjects) in expectations {
        let scenario = corpus.iter().find(|s| s.id == *id).unwrap();
        let current = materialize(scenario, scenario.events.len());
        let deterministic = run_deterministic_merge(&policy, scenario, &current, false);
        let comparison = build_naive_comparison(scenario, &current, &deterministic);
        let got: Vec<&str> = comparison
            .diffs
            .iter()
            .map(|d| d.subject_label.as_str())
            .collect();
        assert_eq!(&got, expected_subjects, "{id} naive diff subjects");
        for diff in &comparison.diffs {
            assert_eq!(
                diff.note, "Naive reunion would erase visible residue.",
                "{id}: every corpus diff is a residue-erasure case"
            );
        }
    }

    // The trust-laundering scenario's naive pick is the whole point:
    // Island B's louder laundered cluster wins the naive sort.
    let scenario = corpus
        .iter()
        .find(|s| s.id == "deterministic-reunion-trust-laundering")
        .unwrap();
    let current = materialize(scenario, scenario.events.len());
    let deterministic = run_deterministic_merge(&policy, scenario, &current, false);
    let comparison = build_naive_comparison(scenario, &current, &deterministic);
    let lumen = comparison
        .naive
        .iter()
        .find(|m| m.subject_label == "Lumen")
        .unwrap();
    assert_eq!(lumen.status, resonant_kernel::belief::BeliefState::Accepted);
    assert_eq!(lumen.source, "Island B");
}
