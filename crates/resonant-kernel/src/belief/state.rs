//! The canonical nine-state scoped belief lifecycle.
//!
//! PINNED (P1, closes SPEC_AUDIT High #1 "state vocabulary not fully
//! canonical"): SEMANTICS.md:274-288 is the canonical table and
//! `TRANSITION_TABLE` below is its executable form, row by row. Merge
//! output classes project onto these states (`MergeResolution::project`),
//! they are not a second lifecycle. There is deliberately no `Revoked`
//! variant: revocation is `BeliefEvent::Revoked`, a visible transition
//! event, never a durable state (SEMANTICS.md:269).
//!
//! Where the doc table lists several typical targets for one situation,
//! this module pins the conservative reading (each pin is noted inline):
//! recovery re-strengthens through `provisional`, never jumps to
//! `accepted`; quarantine releases to `provisional`; `witnessed` has no
//! direct edge to `removed`.

use crate::epoch::Epoch;
use crate::evidence::{Provenance, Stance};
use crate::id::{ClaimId, OverrideId, WitnessRecordId};
use crate::id::{SubjectId, TrustRootId, WitnessId};
use crate::scope::ScopeId;
use crate::trust::Confidence;
use crate::util::NonEmpty;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Scoped belief about a subject. Exactly the nine canonical states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BeliefState {
    Unknown,
    Introduced,
    Witnessed,
    Provisional,
    Accepted,
    Suspected,
    Disputed,
    Quarantined,
    Removed,
}

impl BeliefState {
    pub const ALL: [BeliefState; 9] = [
        BeliefState::Unknown,
        BeliefState::Introduced,
        BeliefState::Witnessed,
        BeliefState::Provisional,
        BeliefState::Accepted,
        BeliefState::Suspected,
        BeliefState::Disputed,
        BeliefState::Quarantined,
        BeliefState::Removed,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            BeliefState::Unknown => "unknown",
            BeliefState::Introduced => "introduced",
            BeliefState::Witnessed => "witnessed",
            BeliefState::Provisional => "provisional",
            BeliefState::Accepted => "accepted",
            BeliefState::Suspected => "suspected",
            BeliefState::Disputed => "disputed",
            BeliefState::Quarantined => "quarantined",
            BeliefState::Removed => "removed",
        }
    }
}

impl fmt::Display for BeliefState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Why confidence narrowed without active conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NarrowReason {
    FreshnessLoss,
    CorroborationDecay,
}

/// Why previously stronger belief degraded into suspicion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuspicionBasis {
    FreshnessLoss,
    FailedCorroboration,
    EarlyConflictPressure,
}

/// Why propagation/acceptance was suspended.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuarantineReason {
    ConflictPressure,
    TrustRepairInProgress,
    OperatorHold(OverrideId),
}

/// Why a subject stopped being treated as a member in this scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemovalBasis {
    Departure,
    PolicyViolation,
    RevocationOutcome,
}

/// What a revocation event targets. Subject lifecycle and standing
/// machines stay distinct, but share the one event type (P12).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevocationTarget {
    SubjectAcceptance { scope: ScopeId, subject: SubjectId },
    WitnessStanding { scope: ScopeId, witness: WitnessId },
    TrustRootStanding { scope: ScopeId, root: TrustRootId },
}

/// What backs a revocation: attributable records or a visible override.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevocationSupport {
    Witnessed(NonEmpty<WitnessRecordId>),
    Override(OverrideId),
}

/// Revocation is a visible transition event, not a durable state.
/// The degraded state it drives toward is chosen by policy at the caller
/// and validated against the canonical table here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevocationEvent {
    pub target: RevocationTarget,
    /// Must be one of suspected / disputed / quarantined / removed.
    pub outcome: BeliefState,
    pub supported_by: RevocationSupport,
    pub epoch: Epoch,
}

/// Events that move scoped belief. Every variant carries the evidence that
/// justifies it — transitions cannot happen without evidence, by
/// construction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BeliefEvent {
    /// A claim was admitted into scope. Introduction is not acceptance.
    Introduced {
        claim: ClaimId,
        provenance: Provenance,
    },
    /// A witness record arrived. Only moves introduced -> witnessed; in
    /// later states the record is absorbed as evidence without a
    /// transition.
    WitnessRecorded {
        record: WitnessRecordId,
        stance: Stance,
    },
    /// Scoped corroboration got strong enough to strengthen belief one
    /// step. Pinned conservative ladder: witnessed -> provisional,
    /// provisional -> accepted, suspected -> provisional,
    /// disputed -> provisional. Recovery never jumps straight to accepted.
    CorroborationReached {
        records: NonEmpty<WitnessRecordId>,
        confidence: Confidence,
    },
    /// Confidence narrowed without active conflict:
    /// accepted -> provisional, provisional -> witnessed,
    /// suspected -> witnessed.
    ConfidenceNarrowed {
        reason: NarrowReason,
    },
    /// Previously stronger belief degraded.
    SuspicionRaised {
        basis: SuspicionBasis,
    },
    /// Fresh admissible evidence is in active conflict.
    ConflictDetected {
        opposing: NonEmpty<WitnessRecordId>,
    },
    /// Revocation: an event, never a state.
    Revoked(RevocationEvent),
    QuarantineImposed {
        reason: QuarantineReason,
    },
    /// Release requires fresh corroboration by construction (P6).
    QuarantineReleased {
        fresh: NonEmpty<WitnessRecordId>,
    },
    RemovalDecided {
        basis: RemovalBasis,
    },
    /// removed -> introduced with a new claim.
    Reintroduced {
        claim: ClaimId,
    },
    /// removed -> unknown, only in a strictly newer epoch (gated in
    /// `BeliefCell::apply`).
    EpochReset {
        new_epoch: Epoch,
    },
    /// A merge outcome projected onto this cell. The merge engine's own
    /// typing constrains what can be projected; the cell records the merge
    /// input digest as evidence.
    MergeProjected {
        input_digest: [u8; 32],
        to: BeliefState,
    },
    /// An operator override. Sovereign but visible: it can force any
    /// state, and the transition permanently cites the override id, so it
    /// can never masquerade as organic convergence.
    OverrideApplied {
        override_id: OverrideId,
        to: BeliefState,
    },
}

/// Discriminant of `BeliefEvent`, used in tables and errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EventKind {
    Introduced,
    WitnessRecorded,
    CorroborationReached,
    ConfidenceNarrowed,
    SuspicionRaised,
    ConflictDetected,
    Revoked,
    QuarantineImposed,
    QuarantineReleased,
    RemovalDecided,
    Reintroduced,
    EpochReset,
    MergeProjected,
    OverrideApplied,
}

impl BeliefEvent {
    pub fn kind(&self) -> EventKind {
        match self {
            BeliefEvent::Introduced { .. } => EventKind::Introduced,
            BeliefEvent::WitnessRecorded { .. } => EventKind::WitnessRecorded,
            BeliefEvent::CorroborationReached { .. } => EventKind::CorroborationReached,
            BeliefEvent::ConfidenceNarrowed { .. } => EventKind::ConfidenceNarrowed,
            BeliefEvent::SuspicionRaised { .. } => EventKind::SuspicionRaised,
            BeliefEvent::ConflictDetected { .. } => EventKind::ConflictDetected,
            BeliefEvent::Revoked(_) => EventKind::Revoked,
            BeliefEvent::QuarantineImposed { .. } => EventKind::QuarantineImposed,
            BeliefEvent::QuarantineReleased { .. } => EventKind::QuarantineReleased,
            BeliefEvent::RemovalDecided { .. } => EventKind::RemovalDecided,
            BeliefEvent::Reintroduced { .. } => EventKind::Reintroduced,
            BeliefEvent::EpochReset { .. } => EventKind::EpochReset,
            BeliefEvent::MergeProjected { .. } => EventKind::MergeProjected,
            BeliefEvent::OverrideApplied { .. } => EventKind::OverrideApplied,
        }
    }
}

/// The canonical table (SEMANTICS.md:274-288), organic edges only.
/// `Revoked`, `MergeProjected`, and `OverrideApplied` carry their target in
/// the event and are validated separately in `transition`.
pub const TRANSITION_TABLE: &[(BeliefState, EventKind, BeliefState)] = {
    use BeliefState as S;
    use EventKind as E;
    &[
        (S::Unknown, E::Introduced, S::Introduced),
        (S::Introduced, E::WitnessRecorded, S::Witnessed),
        (S::Introduced, E::QuarantineImposed, S::Quarantined),
        (S::Introduced, E::RemovalDecided, S::Removed),
        (S::Witnessed, E::CorroborationReached, S::Provisional),
        (S::Witnessed, E::SuspicionRaised, S::Suspected),
        (S::Witnessed, E::ConflictDetected, S::Disputed),
        (S::Witnessed, E::QuarantineImposed, S::Quarantined),
        // Pinned: no direct witnessed -> removed edge; removal from this
        // stage must pass through quarantine, dispute, or an override.
        (S::Provisional, E::CorroborationReached, S::Accepted),
        (S::Provisional, E::ConfidenceNarrowed, S::Witnessed),
        (S::Provisional, E::SuspicionRaised, S::Suspected),
        (S::Provisional, E::ConflictDetected, S::Disputed),
        (S::Provisional, E::QuarantineImposed, S::Quarantined),
        (S::Provisional, E::RemovalDecided, S::Removed),
        (S::Accepted, E::ConfidenceNarrowed, S::Provisional),
        (S::Accepted, E::SuspicionRaised, S::Suspected),
        (S::Accepted, E::ConflictDetected, S::Disputed),
        (S::Accepted, E::QuarantineImposed, S::Quarantined),
        (S::Accepted, E::RemovalDecided, S::Removed),
        (S::Suspected, E::CorroborationReached, S::Provisional),
        (S::Suspected, E::ConfidenceNarrowed, S::Witnessed),
        (S::Suspected, E::ConflictDetected, S::Disputed),
        (S::Suspected, E::QuarantineImposed, S::Quarantined),
        (S::Suspected, E::RemovalDecided, S::Removed),
        (S::Disputed, E::CorroborationReached, S::Provisional),
        (S::Disputed, E::QuarantineImposed, S::Quarantined),
        (S::Disputed, E::RemovalDecided, S::Removed),
        (S::Quarantined, E::QuarantineReleased, S::Provisional),
        (S::Quarantined, E::RemovalDecided, S::Removed),
        (S::Removed, E::Reintroduced, S::Introduced),
        (S::Removed, E::EpochReset, S::Unknown),
    ]
};

#[derive(Debug, Error, PartialEq, Eq, Serialize, Deserialize)]
#[error("invalid transition: {from} does not accept {event:?}")]
pub struct InvalidTransition {
    pub from: BeliefState,
    pub event: EventKind,
}

/// Compute the target state for an event, or refuse.
///
/// Organic events follow `TRANSITION_TABLE` exactly (property-tested to
/// agree with it). Target-carrying events are validated:
/// - `Revoked` may only drive toward one of the four degraded states, and
///   only along edges the canonical table already permits for the source
///   state.
/// - `MergeProjected` may not resurrect (`-> unknown` except from removed),
///   may not re-introduce, and a projection onto the current state is a
///   confirmation, handled as a no-op by the cell.
/// - `OverrideApplied` may force any state; sovereignty is paid for with
///   permanent visibility.
pub fn transition(
    from: BeliefState,
    event: &BeliefEvent,
) -> Result<BeliefState, InvalidTransition> {
    use BeliefState::*;
    let refuse = || InvalidTransition {
        from,
        event: event.kind(),
    };
    match event {
        BeliefEvent::Revoked(rev) => {
            let degraded = matches!(rev.outcome, Suspected | Disputed | Quarantined | Removed);
            if !degraded {
                return Err(refuse());
            }
            let allowed: &[BeliefState] = match from {
                Provisional | Accepted => &[Suspected, Disputed, Quarantined, Removed],
                Suspected => &[Disputed, Quarantined, Removed],
                Disputed => &[Quarantined, Removed],
                Quarantined => &[Removed],
                _ => &[],
            };
            if allowed.contains(&rev.outcome) {
                Ok(rev.outcome)
            } else {
                Err(refuse())
            }
        }
        BeliefEvent::MergeProjected { to, .. } => {
            let resurrects = *to == Unknown && from != Removed;
            // A merge may carry an introduction into a scope that had no
            // live belief, but never re-introduce over one.
            let reintroduces = *to == Introduced && !matches!(from, Unknown | Removed);
            if resurrects || reintroduces {
                Err(refuse())
            } else {
                Ok(*to)
            }
        }
        BeliefEvent::OverrideApplied { to, .. } => Ok(*to),
        organic => TRANSITION_TABLE
            .iter()
            .find(|(f, k, _)| *f == from && *k == organic.kind())
            .map(|(_, _, to)| *to)
            .ok_or_else(refuse),
    }
}
