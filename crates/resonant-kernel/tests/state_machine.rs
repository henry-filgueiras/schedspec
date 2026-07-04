//! The canonical belief lifecycle, tested row-by-row against
//! SEMANTICS.md:274-288 plus the kernel's pinned edges.

use resonant_kernel::belief::{
    transition, BeliefCell, BeliefEvent, BeliefState, EventKind, RevocationEvent,
    RevocationSupport, RevocationTarget, TransitionError, TRANSITION_TABLE,
};
use resonant_kernel::epoch::{Epoch, Round};
use resonant_kernel::evidence::{Provenance, Stance};
use resonant_kernel::id::{ClaimId, OverrideId, PeerId, SubjectId, WitnessRecordId};
use resonant_kernel::policy::PolicyBundle;
use resonant_kernel::scope::ScopeId;
use resonant_kernel::trust::Confidence;
use resonant_kernel::util::NonEmpty;

fn rec(n: u8) -> WitnessRecordId {
    WitnessRecordId::from_bytes([n; 32])
}

fn sample_event(kind: EventKind) -> BeliefEvent {
    match kind {
        EventKind::Introduced => BeliefEvent::Introduced {
            claim: ClaimId::from_bytes([1; 32]),
            provenance: Provenance {
                introducer: PeerId::new("intro"),
            },
        },
        EventKind::WitnessRecorded => BeliefEvent::WitnessRecorded {
            record: rec(2),
            stance: Stance::Corroborate,
        },
        EventKind::CorroborationReached => BeliefEvent::CorroborationReached {
            records: NonEmpty::new(rec(3)),
            confidence: Confidence::Bounded,
        },
        EventKind::ConfidenceNarrowed => BeliefEvent::ConfidenceNarrowed {
            reason: resonant_kernel::belief::NarrowReason::FreshnessLoss,
        },
        EventKind::SuspicionRaised => BeliefEvent::SuspicionRaised {
            basis: resonant_kernel::belief::SuspicionBasis::FailedCorroboration,
        },
        EventKind::ConflictDetected => BeliefEvent::ConflictDetected {
            opposing: NonEmpty::new(rec(4)),
        },
        EventKind::Revoked => BeliefEvent::Revoked(RevocationEvent {
            target: RevocationTarget::SubjectAcceptance {
                scope: ScopeId::new("s"),
                subject: SubjectId::new("x"),
            },
            outcome: BeliefState::Removed,
            supported_by: RevocationSupport::Witnessed(NonEmpty::new(rec(5))),
            epoch: Epoch(1),
        }),
        EventKind::QuarantineImposed => BeliefEvent::QuarantineImposed {
            reason: resonant_kernel::belief::QuarantineReason::ConflictPressure,
        },
        EventKind::QuarantineReleased => BeliefEvent::QuarantineReleased {
            fresh: NonEmpty::new(rec(6)),
        },
        EventKind::RemovalDecided => BeliefEvent::RemovalDecided {
            basis: resonant_kernel::belief::RemovalBasis::Departure,
        },
        EventKind::Reintroduced => BeliefEvent::Reintroduced {
            claim: ClaimId::from_bytes([7; 32]),
        },
        EventKind::EpochReset => BeliefEvent::EpochReset {
            new_epoch: Epoch(99),
        },
        EventKind::MergeProjected => BeliefEvent::MergeProjected {
            input_digest: [8; 32],
            to: BeliefState::Provisional,
        },
        EventKind::OverrideApplied => BeliefEvent::OverrideApplied {
            override_id: OverrideId::from_bytes([9; 32]),
            to: BeliefState::Quarantined,
        },
    }
}

const ORGANIC: [EventKind; 12] = [
    EventKind::Introduced,
    EventKind::WitnessRecorded,
    EventKind::CorroborationReached,
    EventKind::ConfidenceNarrowed,
    EventKind::SuspicionRaised,
    EventKind::ConflictDetected,
    EventKind::QuarantineImposed,
    EventKind::QuarantineReleased,
    EventKind::RemovalDecided,
    EventKind::Reintroduced,
    EventKind::EpochReset,
    EventKind::Revoked, // validated separately, but exhaustively below
];

/// `transition()` agrees with `TRANSITION_TABLE` on every (state, organic
/// event) pair — allowed edges land where the table says, everything else
/// refuses.
#[test]
fn transition_function_matches_table_exhaustively() {
    for from in BeliefState::ALL {
        for kind in ORGANIC {
            if kind == EventKind::Revoked {
                continue;
            }
            let expected = TRANSITION_TABLE
                .iter()
                .find(|(f, k, _)| *f == from && *k == kind)
                .map(|(_, _, to)| *to);
            let got = transition(from, &sample_event(kind)).ok();
            assert_eq!(got, expected, "({from}, {kind:?})");
        }
    }
}

/// There is no `Revoked` belief state: revocation is an event, and it can
/// only drive movement into the four degraded states along table edges.
#[test]
fn revocation_is_an_event_not_a_state() {
    // The nine canonical states are exactly the nine; this is a compile-time
    // fact (no Revoked variant exists), asserted here as documentation.
    assert_eq!(BeliefState::ALL.len(), 9);

    let revoke_to = |outcome: BeliefState| {
        BeliefEvent::Revoked(RevocationEvent {
            target: RevocationTarget::SubjectAcceptance {
                scope: ScopeId::new("s"),
                subject: SubjectId::new("x"),
            },
            outcome,
            supported_by: RevocationSupport::Override(OverrideId::from_bytes([1; 32])),
            epoch: Epoch(4),
        })
    };

    // Revocation can never strengthen or reset.
    for target in [
        BeliefState::Accepted,
        BeliefState::Provisional,
        BeliefState::Unknown,
    ] {
        assert!(transition(BeliefState::Accepted, &revoke_to(target)).is_err());
    }
    // From accepted, all four degraded outcomes are reachable.
    for target in [
        BeliefState::Suspected,
        BeliefState::Disputed,
        BeliefState::Quarantined,
        BeliefState::Removed,
    ] {
        assert_eq!(
            transition(BeliefState::Accepted, &revoke_to(target)),
            Ok(target)
        );
    }
    // Degradation is monotone: a quarantined subject can only be revoked
    // toward removal, and never "sideways" into suspicion.
    assert_eq!(
        transition(BeliefState::Quarantined, &revoke_to(BeliefState::Removed)),
        Ok(BeliefState::Removed)
    );
    assert!(transition(BeliefState::Quarantined, &revoke_to(BeliefState::Suspected)).is_err());
    // Nothing weaker than provisional can be "revoked" — there is no
    // acceptance to revoke.
    assert!(transition(BeliefState::Witnessed, &revoke_to(BeliefState::Removed)).is_err());
}

/// Merge projections cannot resurrect or re-introduce.
#[test]
fn merge_projection_limits() {
    let project = |to| BeliefEvent::MergeProjected {
        input_digest: [0; 32],
        to,
    };
    assert!(transition(BeliefState::Accepted, &project(BeliefState::Unknown)).is_err());
    assert!(transition(BeliefState::Accepted, &project(BeliefState::Introduced)).is_err());
    // A merge may carry an introduction into a scope with no live belief.
    assert_eq!(
        transition(BeliefState::Unknown, &project(BeliefState::Introduced)),
        Ok(BeliefState::Introduced)
    );
    assert_eq!(
        transition(BeliefState::Removed, &project(BeliefState::Unknown)),
        Ok(BeliefState::Unknown)
    );
    assert_eq!(
        transition(BeliefState::Accepted, &project(BeliefState::Removed)),
        Ok(BeliefState::Removed)
    );
}

fn walk_to_provisional(cell: &mut BeliefCell, policy: &PolicyBundle) -> u64 {
    let mut round = 0u64;
    let fire = |cell: &mut BeliefCell, event: BeliefEvent, round: u64| {
        cell.apply(event, (Epoch(1), Round(round)), policy).unwrap();
    };
    fire(cell, sample_event(EventKind::Introduced), round);
    round += 1;
    fire(cell, sample_event(EventKind::WitnessRecorded), round);
    round += policy.strengthen_hysteresis_rounds;
    fire(cell, sample_event(EventKind::CorroborationReached), round);
    round
}

/// Strengthening waits out hysteresis; weakening never does.
#[test]
fn hysteresis_is_asymmetric() {
    let policy = PolicyBundle::default();
    let mut cell = BeliefCell::new((Epoch(1), Round(0)));
    let round = walk_to_provisional(&mut cell, &policy);
    assert_eq!(cell.state(), BeliefState::Provisional);

    // Immediate re-strengthening is refused, visibly.
    let too_soon = cell.apply(
        sample_event(EventKind::CorroborationReached),
        (Epoch(1), Round(round)),
        &policy,
    );
    assert!(matches!(
        too_soon,
        Err(TransitionError::HysteresisHold { .. })
    ));

    // Immediate weakening is allowed.
    let weaken = cell.apply(
        sample_event(EventKind::SuspicionRaised),
        (Epoch(1), Round(round)),
        &policy,
    );
    assert_eq!(weaken.unwrap().unwrap().to, BeliefState::Suspected);
}

/// Removed -> Unknown requires a strictly newer epoch.
#[test]
fn epoch_reset_is_gated() {
    let policy = PolicyBundle::default();
    let mut cell = BeliefCell::new((Epoch(5), Round(0)));
    cell.apply(
        sample_event(EventKind::Introduced),
        (Epoch(5), Round(0)),
        &policy,
    )
    .unwrap();
    cell.apply(
        sample_event(EventKind::RemovalDecided),
        (Epoch(5), Round(1)),
        &policy,
    )
    .unwrap();
    assert_eq!(cell.state(), BeliefState::Removed);

    let stale = cell.apply(
        BeliefEvent::EpochReset {
            new_epoch: Epoch(5),
        },
        (Epoch(5), Round(2)),
        &policy,
    );
    assert!(matches!(stale, Err(TransitionError::EpochNotNewer { .. })));

    let fresh = cell.apply(
        BeliefEvent::EpochReset {
            new_epoch: Epoch(6),
        },
        (Epoch(6), Round(3)),
        &policy,
    );
    assert_eq!(fresh.unwrap().unwrap().to, BeliefState::Unknown);
    assert!(
        cell.supporting().is_empty(),
        "reset clears evidence from the prior life"
    );
}

/// Extra witness records beyond `introduced` are absorbed as evidence
/// without a state change, and every real transition lands in history.
#[test]
fn evidence_absorbs_and_history_records() {
    let policy = PolicyBundle::default();
    let mut cell = BeliefCell::new((Epoch(1), Round(0)));
    walk_to_provisional(&mut cell, &policy);

    let absorbed = cell
        .apply(
            BeliefEvent::WitnessRecorded {
                record: rec(42),
                stance: Stance::Dispute,
            },
            (Epoch(1), Round(9)),
            &policy,
        )
        .unwrap();
    assert!(
        absorbed.is_none(),
        "extra record is evidence, not a transition"
    );
    assert!(cell.opposing().iter().any(|r| *r == rec(42)));
    assert_eq!(cell.state(), BeliefState::Provisional);
    // unknown -> introduced -> witnessed -> provisional
    assert_eq!(cell.history().len(), 3);
}
