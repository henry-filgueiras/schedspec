//! Layer 3: kernel ≡ oracle. The kernel's typed, tiered merge engine must
//! reproduce the lab.js oracle's structural outcomes on every scenario, at
//! every replay prefix, with and without the operator override.
//!
//! This is the load-bearing conformance claim: the kernel's cross-class
//! comparisons are count-free (`DominanceEvidence` has no count field)
//! while lab.js folds the capped count into one flat score — and the
//! outcomes still agree everywhere the corpus reaches.

use resonant_kernel::policy::MergePolicy;
use resonant_lab::conformance::{compare, run_kernel_reunion};
use resonant_lab::oracle::run_deterministic_merge;
use resonant_lab::replay::materialize;
use resonant_lab::scenario::{default_scenarios_dir, load_corpus};

#[test]
fn kernel_matches_oracle_on_every_prefix() {
    let policy = MergePolicy::lab_compat();
    let corpus = load_corpus(&default_scenarios_dir()).expect("corpus loads");
    let mut combinations = 0;

    for scenario in &corpus {
        for steps in 0..=scenario.events.len() {
            let current = materialize(scenario, steps);
            let override_options: &[bool] = if scenario.allow_operator_override {
                &[false, true]
            } else {
                &[false]
            };
            for &apply_override in override_options {
                let oracle_merge =
                    run_deterministic_merge(&policy, scenario, &current, apply_override);
                let kernel = run_kernel_reunion(&policy, scenario, &current, apply_override);
                let failures = compare(&kernel, &oracle_merge);
                assert!(
                    failures.is_empty(),
                    "{} steps={steps} override={apply_override}:\n  {}",
                    scenario.id,
                    failures.join("\n  ")
                );
                combinations += 1;
            }
        }
    }
    // 5 scenarios x 3 prefixes + override scenario x 3 prefixes x 2.
    assert_eq!(combinations, 21, "the sweep covered every combination");
}
