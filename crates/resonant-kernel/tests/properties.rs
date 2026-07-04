//! Property tests for the merge precedence contract and permutation rank.
//!
//! These are the executable form of the treatise's invariants:
//! - witness count may inform, never dominate (INVARIANTS.md #2);
//! - deterministic ordering may only tie-break (SEMANTICS.md:424-427);
//! - residue is conserved through `apply_to` (INVARIANTS.md #4);
//! - rank selections are reconstructable by any observer
//!   (PERMUTATION_RANK.md "accountable determinism").

use proptest::prelude::*;
use resonant_kernel::belief::{BeliefState, MembershipView};
use resonant_kernel::epoch::{Epoch, Round};
use resonant_kernel::evidence::{Diversity, Quality, WitnessSummary};
use resonant_kernel::id::{PeerId, SubjectId};
use resonant_kernel::merge::engine::{
    deterministic_reunion, MergeSide, ReunionContext, ReunionOutcome,
};
use resonant_kernel::merge::{BeliefFragment, DecidedBy};
use resonant_kernel::policy::{MergePolicy, PolicyBundle};
use resonant_kernel::rank::{permutation_rank, reconstruct, RankDomain, RankSeed};
use resonant_kernel::scope::{ScopeAuthority, ScopeId};
use resonant_kernel::trust::TrustGrade;
use std::collections::BTreeMap;

fn any_state() -> impl Strategy<Value = BeliefState> {
    prop::sample::select(BeliefState::ALL.to_vec())
}

fn any_quality() -> impl Strategy<Value = Quality> {
    prop::sample::select(vec![Quality::Weak, Quality::Mixed, Quality::Strong])
}

fn any_diversity() -> impl Strategy<Value = Diversity> {
    prop::sample::select(vec![
        Diversity::Laundered,
        Diversity::SingleScope,
        Diversity::Mixed,
        Diversity::CrossScope,
    ])
}

fn any_authority() -> impl Strategy<Value = ScopeAuthority> {
    prop::sample::select(vec![
        ScopeAuthority::Local,
        ScopeAuthority::Regional,
        ScopeAuthority::Global,
    ])
}

prop_compose! {
    fn any_fragment()(
        state in any_state(),
        epoch in 0u64..6,
        trust in 0u8..=100,
        authority in any_authority(),
        quality in any_quality(),
        diversity in any_diversity(),
        count in 0u32..12,
    ) -> BeliefFragment {
        BeliefFragment {
            state,
            epoch: Epoch(epoch),
            trust: TrustGrade::new(trust),
            authority,
            witness: WitnessSummary { count, quality, diversity },
        }
    }
}

fn reunion_of(a: &BeliefFragment, b: &BeliefFragment) -> ReunionOutcome {
    let policy = MergePolicy::lab_compat();
    let ctx = ReunionContext {
        scope: ScopeId::new("prop"),
        epoch: Epoch(10),
        round: Round(1),
    };
    let subject = SubjectId::new("subject");
    let side = |label: &str, f: &BeliefFragment| MergeSide {
        label: label.into(),
        epoch: f.epoch,
        fragments: BTreeMap::from([(subject.clone(), f.clone())]),
    };
    deterministic_reunion(
        &policy,
        &ctx,
        std::slice::from_ref(&subject),
        &side("a", a),
        &side("b", b),
        None,
    )
}

fn both_permissive(a: &BeliefFragment, b: &BeliefFragment) -> bool {
    let permissive = |s: BeliefState| matches!(s, BeliefState::Accepted | BeliefState::Provisional);
    permissive(a.state) && permissive(b.state)
}

proptest! {
    /// Count may inform (within a semantic class) but never dominate:
    /// whenever the two sides disagree across classes, inflating either
    /// side's witness count arbitrarily never changes the projected state.
    #[test]
    fn count_cannot_dominate_across_classes(
        a in any_fragment(),
        b in any_fragment(),
        boost_a in 0u32..500,
        boost_b in 0u32..500,
    ) {
        prop_assume!(a.state != b.state);
        prop_assume!(!both_permissive(&a, &b));

        let baseline = reunion_of(&a, &b);
        let mut a2 = a.clone();
        let mut b2 = b.clone();
        a2.witness.count += boost_a;
        b2.witness.count += boost_b;
        let inflated = reunion_of(&a2, &b2);

        prop_assert_eq!(
            baseline.outcomes[0].resolution.project(),
            inflated.outcomes[0].resolution.project(),
            "witness count changed a cross-class outcome"
        );
    }

    /// The informer never decides across classes: any outcome decided by
    /// `Informer` involves two same-class (permissive) fragments.
    #[test]
    fn informer_only_decides_within_class(a in any_fragment(), b in any_fragment()) {
        let outcome = reunion_of(&a, &b);
        if outcome.outcomes[0].resolution.decided_by() == DecidedBy::Informer {
            prop_assert!(
                both_permissive(&a, &b),
                "informer decided a cross-class merge: {:?} vs {:?}",
                a.state,
                b.state
            );
        }
    }

    /// The engine is a pure function: identical inputs give identical
    /// outcomes, bit for bit.
    #[test]
    fn reunion_is_deterministic(a in any_fragment(), b in any_fragment()) {
        prop_assert_eq!(reunion_of(&a, &b), reunion_of(&a, &b));
    }

    /// Side order does not change the projected belief (only which side
    /// label supplies retained detail). The two documented lab.js quirk
    /// shapes — both sides laundered, or both sides restrictive with
    /// different statuses — are excluded: they inherit lab.js's
    /// input-order asymmetry, reproduced deliberately.
    #[test]
    fn projection_is_side_symmetric(a in any_fragment(), b in any_fragment()) {
        let restrictive =
            |s: BeliefState| matches!(s, BeliefState::Removed | BeliefState::Quarantined);
        prop_assume!(!(
            a.witness.diversity == Diversity::Laundered
                && b.witness.diversity == Diversity::Laundered
        ));
        prop_assume!(!(restrictive(a.state) && restrictive(b.state) && a.state != b.state));

        let ab = reunion_of(&a, &b);
        let ba = reunion_of(&b, &a);
        prop_assert_eq!(
            ab.outcomes[0].resolution.project(),
            ba.outcomes[0].resolution.project()
        );
    }

    /// Residue conservation: applying a reunion to a view inserts exactly
    /// the residue the resolutions carry — none is dropped on the way to
    /// the ledger, and unresolved disagreement always leaves at least one
    /// ledger entry.
    #[test]
    fn residue_is_conserved_through_apply(a in any_fragment(), b in any_fragment()) {
        let outcome = reunion_of(&a, &b);
        let carried: usize = outcome.outcomes.iter().map(|o| o.resolution.residue().len()).sum();
        let disputed = outcome.outcomes[0].resolution.project() == BeliefState::Disputed;

        let mut view = MembershipView::new(ScopeId::new("prop"), Epoch(10));
        let applied = outcome
            .apply_to(&mut view, &PolicyBundle::default(), (Epoch(10), Round(1)))
            .expect("merge projections are valid transitions");

        prop_assert_eq!(applied.residue_inserted, carried);
        prop_assert_eq!(
            view.residue().len() + applied.residue_superseded,
            carried
        );
        if disputed {
            prop_assert!(!view.residue().is_empty(), "a dispute must leave visible residue");
        }
    }

    /// Any recorded rank selection is independently reconstructable, and
    /// adding a candidate to the pool never reshuffles the relative order
    /// of the existing candidates.
    #[test]
    fn rank_is_reconstructable_and_pool_stable(
        pool in prop::collection::btree_set("[a-z]{1,6}", 1..12),
        extra in "[A-Z]{1,6}",
        round in 0u64..8,
        take in 0usize..6,
    ) {
        let seed = RankSeed {
            domain: RankDomain::WitnessSelection,
            scope: ScopeId::new("prop"),
            subject: None,
            epoch: Epoch(3),
            round: Round(round),
        };
        let peers: Vec<PeerId> = pool.iter().map(|p| PeerId::new(p.clone())).collect();

        let selection = permutation_rank(seed.clone(), peers.clone(), vec![], take);
        prop_assert_eq!(reconstruct(&selection), Ok(()));

        let mut grown = peers.clone();
        grown.push(PeerId::new(extra));
        let grown_selection = permutation_rank(seed, grown, vec![], take);
        let restricted: Vec<&PeerId> = grown_selection
            .ranked
            .iter()
            .map(|(p, _)| p)
            .filter(|p| peers.contains(p))
            .collect();
        let original: Vec<&PeerId> = selection.ranked.iter().map(|(p, _)| p).collect();
        prop_assert_eq!(restricted, original, "pool growth reshuffled existing candidates");
    }

    /// Changing only the seed's round can change the order (hotspot
    /// damping) but never the membership of the ranked set, and every
    /// ordering under every round remains reconstructable.
    #[test]
    fn rank_rounds_rotate_but_preserve_membership(
        pool in prop::collection::btree_set("[a-z]{1,6}", 2..10),
        round_a in 0u64..4,
        round_b in 4u64..8,
    ) {
        let peers: Vec<PeerId> = pool.iter().map(|p| PeerId::new(p.clone())).collect();
        let seed = |round: u64| RankSeed {
            domain: RankDomain::Repair,
            scope: ScopeId::new("prop"),
            subject: Some(SubjectId::new("s")),
            epoch: Epoch(1),
            round: Round(round),
        };
        let sel_a = permutation_rank(seed(round_a), peers.clone(), vec![], peers.len());
        let sel_b = permutation_rank(seed(round_b), peers.clone(), vec![], peers.len());
        prop_assert_eq!(reconstruct(&sel_a), Ok(()));
        prop_assert_eq!(reconstruct(&sel_b), Ok(()));
        let mut members_a: Vec<&PeerId> = sel_a.selected.iter().collect();
        let mut members_b: Vec<&PeerId> = sel_b.selected.iter().collect();
        members_a.sort();
        members_b.sort();
        prop_assert_eq!(members_a, members_b);
    }
}
