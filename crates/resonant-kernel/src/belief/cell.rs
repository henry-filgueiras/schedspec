//! One cell of scoped belief: state + evidence + hysteresis + history.

use crate::belief::state::{transition, BeliefEvent, BeliefState, EventKind, InvalidTransition};
use crate::epoch::{Epoch, Round};
use crate::evidence::Stance;
use crate::id::WitnessRecordId;
use crate::policy::PolicyBundle;
use crate::trust::Confidence;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

/// A recorded state change: the from/to pair plus the evidence-carrying
/// event that caused it and when it happened. The per-subject history of
/// these is the explainability spine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transition {
    pub from: BeliefState,
    pub to: BeliefState,
    pub event: BeliefEvent,
    pub at: (Epoch, Round),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TransitionError {
    #[error(transparent)]
    Invalid(#[from] InvalidTransition),
    /// Strengthening refused because the state is too young. Visible, not
    /// silent: callers transcript this as `TransitionRefused`.
    #[error("hysteresis hold in {state}: held {held_rounds} of {required_rounds} rounds")]
    HysteresisHold {
        state: BeliefState,
        held_rounds: u64,
        required_rounds: u64,
    },
    #[error("epoch reset requires a strictly newer epoch: current {current}, offered {offered}")]
    EpochNotNewer { current: Epoch, offered: Epoch },
}

/// Scoped belief about one subject. The state is private: the only mutator
/// is `apply`, which runs the canonical transition table plus the
/// hysteresis gate — illegal transitions are unrepresentable at this
/// boundary rather than discouraged by convention.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeliefCell {
    state: BeliefState,
    since_round: Round,
    epoch: Epoch,
    confidence: Confidence,
    supporting: BTreeSet<WitnessRecordId>,
    opposing: BTreeSet<WitnessRecordId>,
    history: Vec<Transition>,
}

impl BeliefCell {
    pub fn new(at: (Epoch, Round)) -> Self {
        Self {
            state: BeliefState::Unknown,
            since_round: at.1,
            epoch: at.0,
            confidence: Confidence::Collapsed,
            supporting: BTreeSet::new(),
            opposing: BTreeSet::new(),
            history: Vec::new(),
        }
    }

    pub fn state(&self) -> BeliefState {
        self.state
    }

    pub fn epoch(&self) -> Epoch {
        self.epoch
    }

    pub fn confidence(&self) -> Confidence {
        self.confidence
    }

    pub fn history(&self) -> &[Transition] {
        &self.history
    }

    pub fn supporting(&self) -> &BTreeSet<WitnessRecordId> {
        &self.supporting
    }

    pub fn opposing(&self) -> &BTreeSet<WitnessRecordId> {
        &self.opposing
    }

    /// Apply an event. `Ok(Some(_))` is a state change, `Ok(None)` means
    /// the evidence was absorbed without a transition (e.g. an additional
    /// witness record while already witnessed, or a merge confirming the
    /// current state).
    pub fn apply(
        &mut self,
        event: BeliefEvent,
        at: (Epoch, Round),
        policy: &PolicyBundle,
    ) -> Result<Option<Transition>, TransitionError> {
        // Evidence absorption without transition.
        match &event {
            BeliefEvent::WitnessRecorded { record, stance }
                if self.state != BeliefState::Introduced =>
            {
                if matches!(self.state, BeliefState::Unknown | BeliefState::Removed) {
                    return Err(InvalidTransition {
                        from: self.state,
                        event: event.kind(),
                    }
                    .into());
                }
                self.absorb_record(*record, *stance);
                return Ok(None);
            }
            BeliefEvent::MergeProjected { to, .. } | BeliefEvent::OverrideApplied { to, .. }
                if *to == self.state =>
            {
                return Ok(None);
            }
            BeliefEvent::EpochReset { new_epoch } if *new_epoch <= self.epoch => {
                return Err(TransitionError::EpochNotNewer {
                    current: self.epoch,
                    offered: *new_epoch,
                });
            }
            _ => {}
        }

        let to = transition(self.state, &event)?;

        // Hysteresis: strengthening is slow, weakening is immediate (P6).
        let required = match event.kind() {
            EventKind::CorroborationReached => Some(policy.strengthen_hysteresis_rounds),
            EventKind::QuarantineReleased => Some(policy.quarantine_release_rounds),
            _ => None,
        };
        if let Some(required_rounds) = required {
            let held_rounds = at.1.get().saturating_sub(self.since_round.get());
            if held_rounds < required_rounds {
                return Err(TransitionError::HysteresisHold {
                    state: self.state,
                    held_rounds,
                    required_rounds,
                });
            }
        }

        // Absorb the event's evidence into the record sets.
        match &event {
            BeliefEvent::WitnessRecorded { record, stance } => self.absorb_record(*record, *stance),
            BeliefEvent::CorroborationReached { records, .. }
            | BeliefEvent::QuarantineReleased { fresh: records } => {
                self.supporting.extend(records.iter().copied());
            }
            BeliefEvent::ConflictDetected { opposing } => {
                self.opposing.extend(opposing.iter().copied());
            }
            _ => {}
        }

        self.confidence = match &event {
            BeliefEvent::CorroborationReached { confidence, .. } => *confidence,
            BeliefEvent::ConfidenceNarrowed { .. } => Confidence::Bounded,
            BeliefEvent::SuspicionRaised { .. } => Confidence::Low,
            BeliefEvent::ConflictDetected { .. } | BeliefEvent::Revoked(_) => Confidence::Collapsed,
            BeliefEvent::QuarantineImposed { .. } => Confidence::Collapsed,
            BeliefEvent::QuarantineReleased { .. } => Confidence::Low,
            _ => self.confidence,
        };

        let recorded = Transition {
            from: self.state,
            to,
            event,
            at,
        };
        self.state = to;
        self.since_round = at.1;
        self.epoch = at.0.max(self.epoch);
        if let BeliefEvent::EpochReset { new_epoch } = &recorded.event {
            self.epoch = *new_epoch;
            self.supporting.clear();
            self.opposing.clear();
        }
        self.history.push(recorded.clone());
        Ok(Some(recorded))
    }

    fn absorb_record(&mut self, record: WitnessRecordId, stance: Stance) {
        match stance {
            Stance::Corroborate => {
                self.supporting.insert(record);
            }
            Stance::Dispute | Stance::Suspect | Stance::SupportRevocation => {
                self.opposing.insert(record);
            }
        }
    }
}
