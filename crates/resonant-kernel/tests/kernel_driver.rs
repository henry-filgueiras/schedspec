//! The sans-IO driver, the transcript chain, replay verification, the
//! digest contract, and the trust-root standing machine.

use resonant_kernel::belief::BeliefState;
use resonant_kernel::digest::{compare, DigestVerdict, RepairDigest};
use resonant_kernel::epoch::Epoch;
use resonant_kernel::evidence::{
    AssertedState, Claim, Observation, ObservationMode, Provenance, Stance, WitnessRecord,
};
use resonant_kernel::id::{PeerId, SubjectId, TrustRootId, WitnessId};
use resonant_kernel::kernel::{verify_replay, Effect, Input, Kernel};
use resonant_kernel::policy::PolicyBundle;
use resonant_kernel::rank::{ExclusionReason, RankDomain};
use resonant_kernel::scope::ScopeId;
use resonant_kernel::transcript::{Transcript, TranscriptEvent, TranscriptSink};
use resonant_kernel::trust::{
    Confidence, RevocationSupport, RootBasis, RootEvent, RootState, TrustGrade, TrustRootStanding,
};
use resonant_kernel::util::NonEmpty;

fn scope() -> ScopeId {
    ScopeId::new("rack-7")
}

fn subject() -> SubjectId {
    SubjectId::new("iona")
}

fn claim() -> Claim {
    Claim {
        subject: subject(),
        asserted: AssertedState::Present,
        scope: scope(),
        provenance: Provenance {
            introducer: PeerId::new("gateway"),
        },
        epoch: Epoch(1),
        evidence: vec![],
    }
}

fn witness_record(witness: &str, stance: Stance) -> WitnessRecord {
    let observation = Observation {
        observer: WitnessId::new(witness),
        subject: subject(),
        mode: ObservationMode::DirectContact,
        epoch: Epoch(1),
    };
    WitnessRecord {
        witness: WitnessId::new(witness),
        subject: subject(),
        about: None,
        stance,
        observation: observation.id(),
        mode: ObservationMode::DirectContact,
        scope: scope(),
        epoch: Epoch(1),
        trust_context: TrustGrade::new(70),
    }
}

/// A subject's full journey, transcripted end to end: introduce, witness,
/// wait out hysteresis (the refusal is visible), strengthen, explain.
#[test]
fn lifecycle_is_fully_transcripted_and_replayable() {
    let mut kernel = Kernel::new(PolicyBundle::default());
    let mut transcript = Transcript::new();
    let record = witness_record("w1", Stance::Corroborate);
    let record_id = record.id();

    let inputs = vec![
        Input::EpochAdvanced {
            scope: scope(),
            epoch: Epoch(1),
        },
        Input::Introduce(claim()),
        Input::WitnessRecordReceived(record),
        // Too soon: hysteresis holds (visible refusal, not silence).
        Input::CorroborationAssessed {
            scope: scope(),
            subject: subject(),
            records: NonEmpty::new(record_id),
            confidence: Confidence::Bounded,
        },
        Input::Tick,
        Input::Tick,
        Input::CorroborationAssessed {
            scope: scope(),
            subject: subject(),
            records: NonEmpty::new(record_id),
            confidence: Confidence::Bounded,
        },
    ];
    for input in &inputs {
        kernel.handle(input.clone(), &mut transcript);
    }

    let view = kernel.view(&scope()).unwrap();
    assert_eq!(
        view.belief(&subject()).unwrap().state(),
        BeliefState::Provisional
    );

    // The chain is intact and the story is queryable.
    transcript.verify_chain().unwrap();
    let story = transcript.explain(&scope(), &subject());
    assert!(story
        .iter()
        .any(|e| matches!(e.event, TranscriptEvent::ClaimAdmitted { .. })));
    assert!(
        story.iter().any(
            |e| matches!(&e.event, TranscriptEvent::TransitionRefused { reason, .. }
            if reason.contains("hysteresis"))
        ),
        "the hysteresis hold must be visible in the story"
    );
    assert!(story.iter().any(|e| matches!(
        e.event,
        TranscriptEvent::TransitionApplied {
            to: BeliefState::Provisional,
            ..
        }
    )));

    // Replaying the same inputs reproduces the chain digest-for-digest.
    verify_replay(PolicyBundle::default(), &inputs, &transcript).unwrap();

    // A tampered transcript fails verification.
    let mut tampered = transcript.clone();
    tampered.record(TranscriptEvent::TickAdvanced {
        round: resonant_kernel::epoch::Round(99),
    });
    assert!(verify_replay(PolicyBundle::default(), &inputs, &tampered).is_err());
}

/// Witness selection is an accountable effect: the selection rides out as
/// data and is independently reconstructable from the transcript.
#[test]
fn witness_selection_is_accountable() {
    let mut kernel = Kernel::new(PolicyBundle::default());
    let mut transcript = Transcript::new();
    let pool: Vec<PeerId> = ["p1", "p2", "p3", "p4", "p5"]
        .into_iter()
        .map(PeerId::new)
        .collect();
    let effects = kernel.handle(
        Input::RequestWitnessSelection {
            domain: RankDomain::WitnessSelection,
            scope: scope(),
            subject: Some(subject()),
            pool,
            excluded: vec![(PeerId::new("p3"), ExclusionReason::Quarantined)],
            take: 2,
        },
        &mut transcript,
    );
    let [Effect::WitnessSetSelected(selection)] = &effects[..] else {
        panic!("expected a witness set effect");
    };
    assert_eq!(selection.selected.len(), 2);
    assert!(!selection.selected.contains(&PeerId::new("p3")));
    resonant_kernel::rank::reconstruct(selection).unwrap();
    assert!(transcript
        .events()
        .iter()
        .any(|e| matches!(e.event, TranscriptEvent::RankComputed { .. })));
}

/// The digest honesty flags are computed from the ledger, and digest
/// comparison never merges: it fetches detail or holds.
#[test]
fn digest_contract() {
    use resonant_kernel::belief::MembershipView;

    let mut view = MembershipView::new(scope(), Epoch(3));
    let digest = RepairDigest::of(&view);
    assert!(!digest.has_unresolved_disagreement());
    assert_eq!(compare(&digest, &digest), DigestVerdict::NoAction);

    // Grow a belief so the views differ.
    let policy = PolicyBundle::default();
    let cell = view.cell_mut(subject(), (Epoch(3), resonant_kernel::epoch::Round(0)));
    cell.apply(
        resonant_kernel::belief::BeliefEvent::Introduced {
            claim: claim().id(),
            provenance: Provenance {
                introducer: PeerId::new("gateway"),
            },
        },
        (Epoch(3), resonant_kernel::epoch::Round(0)),
        &policy,
    )
    .unwrap();
    let grown = RepairDigest::of(&view);
    let empty = RepairDigest::of(&MembershipView::new(scope(), Epoch(3)));
    match compare(&empty, &grown) {
        DigestVerdict::FetchDetail(subjects) => assert_eq!(subjects, vec![subject()]),
        other => panic!("expected FetchDetail, got {other:?}"),
    }

    // Cross-scope digests never reconcile by fetch.
    let foreign = RepairDigest::of(&MembershipView::new(ScopeId::new("elsewhere"), Epoch(3)));
    assert!(matches!(
        compare(&digest, &foreign),
        DigestVerdict::HoldForRepair(_)
    ));
}

/// The trust-root standing machine: earned standing has no shortcut past
/// probation, and revocation is terminal.
#[test]
fn trust_root_lifecycle() {
    let record = witness_record("w1", Stance::Corroborate).id();

    // Operator-installed roots activate directly.
    let mut installed = TrustRootStanding::propose(
        TrustRootId::new("ca-root"),
        scope(),
        RootBasis::OperatorPolicy {
            operator_note: "bootstrap".into(),
        },
        TrustGrade::new(90),
    );
    assert_eq!(
        installed.apply(&RootEvent::Admitted).unwrap(),
        (RootState::Proposed, RootState::Active)
    );
    assert_eq!(installed.effective_grade(), TrustGrade::new(90));

    // Earned standing must pass through probation, at half weight.
    let mut earned = TrustRootStanding::propose(
        TrustRootId::new("veteran-witness"),
        scope(),
        RootBasis::EarnedHistory {
            justification: NonEmpty::new(record),
        },
        TrustGrade::new(80),
    );
    assert_eq!(
        earned.apply(&RootEvent::Admitted).unwrap(),
        (RootState::Proposed, RootState::Probation)
    );
    assert_eq!(earned.effective_grade(), TrustGrade::new(40));
    earned
        .apply(&RootEvent::ProbationPassed {
            corroboration: NonEmpty::new(record),
        })
        .unwrap();
    assert_eq!(earned.state(), RootState::Active);

    // Revocation is terminal and zeroes effective trust.
    earned
        .apply(&RootEvent::Revoked {
            support: RevocationSupport::Witnessed(NonEmpty::new(record)),
        })
        .unwrap();
    assert_eq!(earned.state(), RootState::Revoked);
    assert_eq!(earned.effective_grade(), TrustGrade::new(0));
    assert!(earned
        .apply(&RootEvent::Reinstated {
            review_note: "no".into()
        })
        .is_err());
}
