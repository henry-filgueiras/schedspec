//! The transcript: a tamper-evident, replayable second representation of
//! every decision the kernel makes.
//!
//! INVARIANTS.md #9 — "the protocol is not correct enough if it converges
//! but cannot explain itself" — is implemented here as architecture, not
//! logging: kernel entry points take `&mut impl TranscriptSink`, so there
//! is no code path that decides without emitting; events are hash-chained
//! (`SealedEvent`) so the transcript is tamper-evident; and because the
//! kernel is deterministic, re-running the recorded inputs must reproduce
//! the chain digest-for-digest (`verify_replay` in `kernel`).

use crate::belief::{BeliefState, EventKind};
use crate::epoch::{Epoch, Round};
use crate::evidence::Stance;
use crate::id::{ClaimId, OverrideId, ResidueId, SubjectId, TrustRootId, WitnessRecordId};
use crate::merge::{DecidedBy, OverallOutcome, RuleId};
use crate::rank::RankedSelection;
use crate::scope::ScopeId;
use crate::trust::RootState;
use serde::{Deserialize, Serialize};

/// Every decision the kernel can make, as data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TranscriptEvent {
    ClaimAdmitted {
        scope: ScopeId,
        subject: SubjectId,
        claim: ClaimId,
    },
    WitnessRecorded {
        scope: ScopeId,
        subject: SubjectId,
        record: WitnessRecordId,
        stance: Stance,
    },
    TransitionApplied {
        scope: ScopeId,
        subject: SubjectId,
        from: BeliefState,
        to: BeliefState,
        event: EventKind,
        at: (Epoch, Round),
    },
    /// Refusals are visible too: a hysteresis hold is a decision.
    TransitionRefused {
        scope: ScopeId,
        subject: SubjectId,
        event: EventKind,
        reason: String,
    },
    RankComputed {
        selection: RankedSelection,
    },
    MergeEvaluated {
        scope: ScopeId,
        subject: SubjectId,
        rule: RuleId,
        decided_by: DecidedBy,
        projected: BeliefState,
    },
    ResiduePreserved {
        scope: ScopeId,
        subject: SubjectId,
        residue: ResidueId,
        superseded: Option<ResidueId>,
        handled_by_override: bool,
    },
    ReunionCompleted {
        scope: ScopeId,
        input_digest: [u8; 32],
        overall: OverallOutcome,
        unresolved_residue: usize,
    },
    TrustRootChanged {
        scope: ScopeId,
        root: TrustRootId,
        from: RootState,
        to: RootState,
        basis: String,
    },
    OverrideApplied {
        scope: ScopeId,
        subject: SubjectId,
        override_id: OverrideId,
        forced: BeliefState,
    },
    EpochAdvanced {
        scope: ScopeId,
        epoch: Epoch,
    },
    TickAdvanced {
        round: Round,
    },
}

impl TranscriptEvent {
    /// Does this event concern the given (scope, subject)?
    pub fn touches(&self, scope: &ScopeId, subject: &SubjectId) -> bool {
        use TranscriptEvent as E;
        match self {
            E::ClaimAdmitted {
                scope: s,
                subject: x,
                ..
            }
            | E::WitnessRecorded {
                scope: s,
                subject: x,
                ..
            }
            | E::TransitionApplied {
                scope: s,
                subject: x,
                ..
            }
            | E::TransitionRefused {
                scope: s,
                subject: x,
                ..
            }
            | E::MergeEvaluated {
                scope: s,
                subject: x,
                ..
            }
            | E::ResiduePreserved {
                scope: s,
                subject: x,
                ..
            }
            | E::OverrideApplied {
                scope: s,
                subject: x,
                ..
            } => s == scope && x == subject,
            E::RankComputed { selection } => {
                &selection.seed.scope == scope && selection.seed.subject.as_ref() == Some(subject)
            }
            E::ReunionCompleted { scope: s, .. } | E::EpochAdvanced { scope: s, .. } => s == scope,
            E::TrustRootChanged { .. } | E::TickAdvanced { .. } => false,
        }
    }
}

/// An event sealed into the hash chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SealedEvent {
    pub seq: u64,
    pub parent: [u8; 32],
    pub event: TranscriptEvent,
    pub digest: [u8; 32],
}

fn seal_digest(parent: &[u8; 32], seq: u64, event: &TranscriptEvent) -> [u8; 32] {
    let body = serde_json::to_vec(event).expect("transcript events serialize");
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"resonant/transcript/v1");
    hasher.update(parent);
    hasher.update(&seq.to_le_bytes());
    hasher.update(&body);
    *hasher.finalize().as_bytes()
}

/// Where transcript events go. Kernel decision paths require one, so a
/// decision without a transcript entry is unrepresentable.
pub trait TranscriptSink {
    fn record(&mut self, event: TranscriptEvent);
}

/// A sink that drops events, for callers that genuinely only want the
/// return value (tests, throwaway queries). Using it is a visible choice.
pub struct DiscardSink;

impl TranscriptSink for DiscardSink {
    fn record(&mut self, _event: TranscriptEvent) {}
}

/// The default sink: an append-only, hash-chained event log.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transcript {
    events: Vec<SealedEvent>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ChainError {
    #[error("event {seq}: sequence out of order")]
    Sequence { seq: u64 },
    #[error("event {seq}: parent digest does not match previous event")]
    Parent { seq: u64 },
    #[error("event {seq}: sealed digest does not match event content")]
    Digest { seq: u64 },
}

impl Transcript {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn events(&self) -> &[SealedEvent] {
        &self.events
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Digest of the whole chain (the last event's digest).
    pub fn head(&self) -> Option<[u8; 32]> {
        self.events.last().map(|e| e.digest)
    }

    /// Verify the hash chain end to end.
    pub fn verify_chain(&self) -> Result<(), ChainError> {
        let mut parent = [0u8; 32];
        for (index, sealed) in self.events.iter().enumerate() {
            let seq = index as u64;
            if sealed.seq != seq {
                return Err(ChainError::Sequence { seq: sealed.seq });
            }
            if sealed.parent != parent {
                return Err(ChainError::Parent { seq });
            }
            if seal_digest(&parent, seq, &sealed.event) != sealed.digest {
                return Err(ChainError::Digest { seq });
            }
            parent = sealed.digest;
        }
        Ok(())
    }

    /// The causal chain for one (scope, subject): introduction, witness
    /// records, transitions (including refusals), merge traces, residue
    /// lineage, overrides.
    pub fn explain(&self, scope: &ScopeId, subject: &SubjectId) -> Vec<&SealedEvent> {
        self.events
            .iter()
            .filter(|e| e.event.touches(scope, subject))
            .collect()
    }
}

impl TranscriptSink for Transcript {
    fn record(&mut self, event: TranscriptEvent) {
        let seq = self.events.len() as u64;
        let parent = self.head().unwrap_or([0u8; 32]);
        let digest = seal_digest(&parent, seq, &event);
        self.events.push(SealedEvent {
            seq,
            parent,
            event,
            digest,
        });
    }
}
