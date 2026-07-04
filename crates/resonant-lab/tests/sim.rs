//! Simulator invariants, property-tested over seeds: reproducibility,
//! post-heal convergence, honest residue on injected conflict, and
//! overrides that mark scars without erasing them.

use proptest::prelude::*;
use resonant_kernel::belief::BeliefState;
use resonant_kernel::merge::OverallOutcome;
use resonant_lab::sim::{run, SimConfig};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]

    /// Same seed, same everything: the report (including transcript chain
    /// heads) is a pure function of the config.
    #[test]
    fn same_seed_reproduces_bit_for_bit(seed in any::<u64>(), subjects in 2u32..6, nodes in 4u32..10) {
        let config = SimConfig { seed, nodes, subjects, operator_override: false };
        let one = run(&config);
        let two = run(&config);
        prop_assert_eq!(
            serde_json::to_string(&one).unwrap(),
            serde_json::to_string(&two).unwrap()
        );
    }

    /// After reunion both islands hold identical views (content hash), and
    /// the injected same-epoch removal/acceptance conflict survives as a
    /// visible dispute with unresolved residue.
    #[test]
    fn reunion_converges_and_preserves_the_conflict(seed in any::<u64>()) {
        let report = run(&SimConfig { seed, operator_override: false, ..SimConfig::default() });
        prop_assert!(report.converged, "islands diverged after reunion");
        prop_assert_eq!(report.victim_state, BeliefState::Disputed);
        prop_assert!(report.unresolved_residue >= 1, "the conflict must leave residue");
        prop_assert_eq!(report.overall, OverallOutcome::Provisional);
    }

    /// An operator override quarantines the disputed subject and takes
    /// responsibility for the residue — the scar is marked handled, never
    /// deleted.
    #[test]
    fn override_marks_but_never_erases(seed in any::<u64>()) {
        let report = run(&SimConfig { seed, operator_override: true, ..SimConfig::default() });
        prop_assert!(report.converged);
        prop_assert_eq!(report.victim_state, BeliefState::Quarantined);
        prop_assert_eq!(report.unresolved_residue, 0);
        prop_assert!(report.handled_residue >= 1, "the scar must remain, marked handled");
        prop_assert_eq!(report.overall, OverallOutcome::StableWithOverride);
    }
}
