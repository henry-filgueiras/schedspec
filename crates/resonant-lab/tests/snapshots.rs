//! Layer 3b: string-exact freeze of the oracle's full output (merged view,
//! digest, residues, naive comparison) per scenario at full replay.
//! Reviewed once against the in-browser lab; thereafter these snapshots
//! are the parity contract — editing a scenario JSON or the oracle shows
//! up as a reviewable diff, never silent drift.

use resonant_kernel::policy::MergePolicy;
use resonant_lab::oracle::{build_naive_comparison, run_deterministic_merge};
use resonant_lab::replay::materialize;
use resonant_lab::scenario::{default_scenarios_dir, load_corpus};

#[test]
fn oracle_output_snapshots() {
    let policy = MergePolicy::lab_compat();
    let corpus = load_corpus(&default_scenarios_dir()).expect("corpus loads");

    for scenario in &corpus {
        let current = materialize(scenario, scenario.events.len());
        let override_options: &[bool] = if scenario.allow_operator_override {
            &[false, true]
        } else {
            &[false]
        };
        for &apply_override in override_options {
            let deterministic =
                run_deterministic_merge(&policy, scenario, &current, apply_override);
            let naive = build_naive_comparison(scenario, &current, &deterministic);
            let name = if apply_override {
                format!("{}__override", scenario.id)
            } else {
                scenario.id.clone()
            };
            insta::assert_json_snapshot!(
                name,
                serde_json::json!({
                    "merge": deterministic,
                    "naive": naive,
                })
            );
        }
    }
}
