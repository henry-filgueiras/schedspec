//! The sans-IO driver: inputs in, effects and transcript out.
//!
//! The kernel owns scoped views, trust-root standings, and the round
//! counter. It performs no IO of any kind: every stimulus arrives as an
//! `Input` value, every consequence leaves as an `Effect` value, and every
//! decision — including refusals — is sealed into the caller's
//! `TranscriptSink`. Because `handle` is a pure function of
//! (state, input), a recorded input sequence replays to a byte-identical
//! transcript chain (`verify_replay`), which is what makes the transcript
//! a proof object rather than a narrative.

use crate::belief::{BeliefEvent, MembershipView, TransitionError};
use crate::digest::{compare, DigestVerdict, RepairDigest};
use crate::epoch::{Epoch, Round};
use crate::evidence::{Claim, WitnessRecord};
use crate::id::{PeerId, SubjectId, TrustRootId, WitnessRecordId};
use crate::merge::engine::{deterministic_reunion, MergeSide, ReunionContext};
use crate::merge::MergeResolution;
use crate::operator::OperatorOverride;
use crate::policy::PolicyBundle;
use crate::rank::{permutation_rank, ExclusionReason, RankDomain, RankSeed, RankedSelection};
use crate::scope::ScopeId;
use crate::transcript::{TranscriptEvent, TranscriptSink};
use crate::trust::{Confidence, RootBasis, RootEvent, TrustGrade, TrustRootStanding};
use crate::util::NonEmpty;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Everything that can happen to the kernel, as data. These are the
//  driver-facing surface of the ten MECHANICS.md loops.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Input {
    /// `introduce()`: a claim admitted into its scope.
    Introduce(Claim),
    /// A witness record arrived for a subject in a scope.
    WitnessRecordReceived(WitnessRecord),
    /// The policy layer judged scoped corroboration strong enough to
    /// strengthen belief one step.
    CorroborationAssessed {
        scope: ScopeId,
        subject: SubjectId,
        records: NonEmpty<WitnessRecordId>,
        confidence: Confidence,
    },
    /// The policy layer decided a subject is no longer a member here.
    RemovalAssessed {
        scope: ScopeId,
        subject: SubjectId,
        basis: crate::belief::RemovalBasis,
    },
    /// The policy layer decided to suspend a subject pending repair.
    QuarantineAssessed {
        scope: ScopeId,
        subject: SubjectId,
        reason: crate::belief::QuarantineReason,
    },
    /// The policy layer decided a quarantine can end; release requires
    /// fresh corroboration by construction (P6).
    QuarantineReleaseAssessed {
        scope: ScopeId,
        subject: SubjectId,
        fresh: NonEmpty<WitnessRecordId>,
    },
    /// A visible operator override applied directly to a scoped belief
    /// (outside a reunion). Forces the state and takes responsibility for
    /// the subject's live residue — marking it handled, never erasing it.
    Override(OperatorOverride),
    /// `form_candidates()` + `select_ranked()`: an accountable selection.
    RequestWitnessSelection {
        domain: RankDomain,
        scope: ScopeId,
        subject: Option<SubjectId>,
        pool: Vec<PeerId>,
        excluded: Vec<(PeerId, ExclusionReason)>,
        take: usize,
    },
    /// `deterministic_reunion()`: reconcile two sides into this scope's
    /// view, optionally under a visible operator override. The rendezvous
    /// round is part of the agreed reunion input — not local state — so
    /// every participant mints identical residue ids and their views
    /// converge content-hash-for-content-hash.
    ReunionRequested {
        scope: ScopeId,
        round: Round,
        subjects: Vec<SubjectId>,
        side_a: MergeSide,
        side_b: MergeSide,
        operator_override: Option<OperatorOverride>,
    },
    /// A repair digest arrived from a peer view of the same scope.
    DigestReceived(RepairDigest),
    /// Trust-root lifecycle (P4).
    TrustRootProposed {
        root: TrustRootId,
        scope: ScopeId,
        basis: RootBasis,
        grade: TrustGrade,
    },
    TrustRootEvent {
        root: TrustRootId,
        scope: ScopeId,
        event: RootEvent,
    },
    EpochAdvanced {
        scope: ScopeId,
        epoch: Epoch,
    },
    /// Advance the kernel round (hysteresis clock).
    Tick,
}

/// Everything the kernel wants done, as data. The driver does the IO.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Effect {
    WitnessSetSelected(RankedSelection),
    ShareDigest(RepairDigest),
    FetchDetail {
        scope: ScopeId,
        subjects: Vec<SubjectId>,
    },
    HoldForRepair {
        scope: ScopeId,
        reason: String,
    },
}

/// The kernel state: scoped views, trust standings, the round counter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Kernel {
    policy: PolicyBundle,
    views: BTreeMap<ScopeId, MembershipView>,
    trust_roots: BTreeMap<(TrustRootId, ScopeId), TrustRootStanding>,
    round: Round,
}

impl Kernel {
    pub fn new(policy: PolicyBundle) -> Self {
        Self {
            policy,
            views: BTreeMap::new(),
            trust_roots: BTreeMap::new(),
            round: Round(0),
        }
    }

    pub fn view(&self, scope: &ScopeId) -> Option<&MembershipView> {
        self.views.get(scope)
    }

    pub fn round(&self) -> Round {
        self.round
    }

    pub fn trust_root(&self, root: &TrustRootId, scope: &ScopeId) -> Option<&TrustRootStanding> {
        self.trust_roots.get(&(root.clone(), scope.clone()))
    }

    pub fn policy(&self) -> &PolicyBundle {
        &self.policy
    }

    fn view_mut(&mut self, scope: &ScopeId) -> &mut MembershipView {
        self.views
            .entry(scope.clone())
            .or_insert_with(|| MembershipView::new(scope.clone(), Epoch(0)))
    }

    /// Apply a belief event to a cell, transcripting the transition or the
    /// refusal. Invalid protocol inputs are absorbed visibly, never
    /// silently and never as a panic.
    fn apply_belief(
        &mut self,
        scope: &ScopeId,
        subject: &SubjectId,
        event: BeliefEvent,
        sink: &mut impl TranscriptSink,
    ) {
        let round = self.round;
        let policy = self.policy.clone();
        let view = self.view_mut(scope);
        let at = (view.epoch(), round);
        let kind = event.kind();
        let cell = view.cell_mut(subject.clone(), at);
        match cell.apply(event, at, &policy) {
            Ok(Some(transition)) => sink.record(TranscriptEvent::TransitionApplied {
                scope: scope.clone(),
                subject: subject.clone(),
                from: transition.from,
                to: transition.to,
                event: kind,
                at,
            }),
            Ok(None) => {}
            Err(error) => sink.record(TranscriptEvent::TransitionRefused {
                scope: scope.clone(),
                subject: subject.clone(),
                event: kind,
                reason: refusal_reason(&error),
            }),
        }
    }

    /// Handle one input. Deterministic and total: same state + same input
    /// always yields the same effects and the same transcript events.
    pub fn handle(&mut self, input: Input, sink: &mut impl TranscriptSink) -> Vec<Effect> {
        match input {
            Input::Introduce(claim) => {
                let scope = claim.scope.clone();
                let subject = claim.subject.clone();
                let claim_id = claim.id();
                sink.record(TranscriptEvent::ClaimAdmitted {
                    scope: scope.clone(),
                    subject: subject.clone(),
                    claim: claim_id,
                });
                self.apply_belief(
                    &scope,
                    &subject,
                    BeliefEvent::Introduced {
                        claim: claim_id,
                        provenance: claim.provenance,
                    },
                    sink,
                );
                vec![]
            }
            Input::WitnessRecordReceived(record) => {
                let scope = record.scope.clone();
                let subject = record.subject.clone();
                let record_id = record.id();
                sink.record(TranscriptEvent::WitnessRecorded {
                    scope: scope.clone(),
                    subject: subject.clone(),
                    record: record_id,
                    stance: record.stance,
                });
                self.apply_belief(
                    &scope,
                    &subject,
                    BeliefEvent::WitnessRecorded {
                        record: record_id,
                        stance: record.stance,
                    },
                    sink,
                );
                vec![]
            }
            Input::CorroborationAssessed {
                scope,
                subject,
                records,
                confidence,
            } => {
                self.apply_belief(
                    &scope,
                    &subject,
                    BeliefEvent::CorroborationReached {
                        records,
                        confidence,
                    },
                    sink,
                );
                vec![]
            }
            Input::RemovalAssessed {
                scope,
                subject,
                basis,
            } => {
                self.apply_belief(
                    &scope,
                    &subject,
                    BeliefEvent::RemovalDecided { basis },
                    sink,
                );
                vec![]
            }
            Input::QuarantineAssessed {
                scope,
                subject,
                reason,
            } => {
                self.apply_belief(
                    &scope,
                    &subject,
                    BeliefEvent::QuarantineImposed { reason },
                    sink,
                );
                vec![]
            }
            Input::QuarantineReleaseAssessed {
                scope,
                subject,
                fresh,
            } => {
                self.apply_belief(
                    &scope,
                    &subject,
                    BeliefEvent::QuarantineReleased { fresh },
                    sink,
                );
                vec![]
            }
            Input::Override(op) => {
                let override_id = op.id();
                // Route the override into whichever scoped view holds live
                // belief about the subject.
                let scope = self
                    .views
                    .iter()
                    .find(|(_, view)| view.belief(&op.subject).is_some())
                    .map(|(scope, _)| scope.clone());
                let Some(scope) = scope else {
                    return vec![];
                };
                sink.record(TranscriptEvent::OverrideApplied {
                    scope: scope.clone(),
                    subject: op.subject.clone(),
                    override_id,
                    forced: op.forced,
                });
                self.apply_belief(
                    &scope,
                    &op.subject,
                    BeliefEvent::OverrideApplied {
                        override_id,
                        to: op.forced,
                    },
                    sink,
                );
                let view = self.view_mut(&scope);
                let marked = view
                    .residue_mut()
                    .mark_all_handled(&op.subject, override_id);
                for residue in view.residue().iter() {
                    if marked > 0 && residue.key().subject == op.subject {
                        sink.record(TranscriptEvent::ResiduePreserved {
                            scope: scope.clone(),
                            subject: op.subject.clone(),
                            residue: residue.id(),
                            superseded: None,
                            handled_by_override: true,
                        });
                    }
                }
                let digest = RepairDigest::of(self.view(&scope).expect("view exists"));
                vec![Effect::ShareDigest(digest)]
            }
            Input::RequestWitnessSelection {
                domain,
                scope,
                subject,
                pool,
                excluded,
                take,
            } => {
                let epoch = self.view_mut(&scope).epoch();
                let seed = RankSeed {
                    domain,
                    scope,
                    subject,
                    epoch,
                    round: self.round,
                };
                let selection = permutation_rank(seed, pool, excluded, take);
                sink.record(TranscriptEvent::RankComputed {
                    selection: selection.clone(),
                });
                vec![Effect::WitnessSetSelected(selection)]
            }
            Input::ReunionRequested {
                scope,
                round,
                subjects,
                side_a,
                side_b,
                operator_override,
            } => {
                let epoch = self
                    .view_mut(&scope)
                    .epoch()
                    .max(side_a.epoch)
                    .max(side_b.epoch);
                let ctx = ReunionContext {
                    scope: scope.clone(),
                    epoch,
                    round,
                };
                let outcome = deterministic_reunion(
                    &self.policy.merge,
                    &ctx,
                    &subjects,
                    &side_a,
                    &side_b,
                    operator_override.as_ref(),
                );

                for subject_outcome in &outcome.outcomes {
                    sink.record(TranscriptEvent::MergeEvaluated {
                        scope: scope.clone(),
                        subject: subject_outcome.subject.clone(),
                        rule: subject_outcome.trace.rule,
                        decided_by: subject_outcome.trace.decided_by,
                        projected: subject_outcome.resolution.project(),
                    });
                    if let MergeResolution::Overridden {
                        override_id,
                        forced,
                        ..
                    } = &subject_outcome.resolution
                    {
                        sink.record(TranscriptEvent::OverrideApplied {
                            scope: scope.clone(),
                            subject: subject_outcome.subject.clone(),
                            override_id: *override_id,
                            forced: *forced,
                        });
                    }
                    for residue in subject_outcome.resolution.residue() {
                        sink.record(TranscriptEvent::ResiduePreserved {
                            scope: scope.clone(),
                            subject: subject_outcome.subject.clone(),
                            residue: residue.id(),
                            superseded: None,
                            handled_by_override: residue.handled_by().is_some(),
                        });
                    }
                }
                sink.record(TranscriptEvent::ReunionCompleted {
                    scope: scope.clone(),
                    input_digest: outcome.input_digest,
                    overall: outcome.digest.overall,
                    unresolved_residue: outcome.digest.unresolved_residue,
                });

                let policy = self.policy.clone();
                let view = self.view_mut(&scope);
                let at = (epoch, round);
                match outcome.apply_to(view, &policy, at) {
                    Ok(_applied) => vec![Effect::ShareDigest(RepairDigest::of(view))],
                    Err(error) => {
                        // Unreachable for engine-produced projections, but
                        // refusal stays visible rather than panicking.
                        sink.record(TranscriptEvent::TransitionRefused {
                            scope: scope.clone(),
                            subject: SubjectId::new("<reunion>"),
                            event: crate::belief::EventKind::MergeProjected,
                            reason: refusal_reason(&error),
                        });
                        vec![]
                    }
                }
            }
            Input::DigestReceived(remote) => {
                let scope = remote.scope.clone();
                let view = self.view_mut(&scope);
                let local = RepairDigest::of(view);
                match compare(&local, &remote) {
                    DigestVerdict::NoAction => vec![],
                    DigestVerdict::FetchDetail(subjects) => {
                        vec![Effect::FetchDetail { scope, subjects }]
                    }
                    DigestVerdict::HoldForRepair(reason) => {
                        vec![Effect::HoldForRepair { scope, reason }]
                    }
                }
            }
            Input::TrustRootProposed {
                root,
                scope,
                basis,
                grade,
            } => {
                let standing =
                    TrustRootStanding::propose(root.clone(), scope.clone(), basis, grade);
                self.trust_roots.insert((root, scope), standing);
                vec![]
            }
            Input::TrustRootEvent { root, scope, event } => {
                if let Some(standing) = self.trust_roots.get_mut(&(root.clone(), scope.clone())) {
                    let basis = standing.basis().kind().to_string();
                    if let Ok((from, to)) = standing.apply(&event) {
                        sink.record(TranscriptEvent::TrustRootChanged {
                            scope,
                            root,
                            from,
                            to,
                            basis,
                        });
                    }
                }
                vec![]
            }
            Input::EpochAdvanced { scope, epoch } => {
                self.view_mut(&scope).advance_epoch(epoch);
                sink.record(TranscriptEvent::EpochAdvanced { scope, epoch });
                vec![]
            }
            Input::Tick => {
                self.round = self.round.next();
                sink.record(TranscriptEvent::TickAdvanced { round: self.round });
                vec![]
            }
        }
    }
}

fn refusal_reason(error: &TransitionError) -> String {
    error.to_string()
}

/// Where a replay diverged from the recorded transcript.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ReplayDivergence {
    #[error("replay produced {got} events, transcript has {expected}")]
    Length { expected: usize, got: usize },
    #[error("first divergence at transcript seq {seq}")]
    Event { seq: u64 },
}

/// Re-run a recorded input sequence on a fresh kernel and require the
/// transcript chain to match digest-for-digest. Because the kernel is
/// deterministic, a verified transcript *is* the decision history — not a
/// story about it.
pub fn verify_replay(
    policy: PolicyBundle,
    inputs: &[Input],
    recorded: &crate::transcript::Transcript,
) -> Result<(), ReplayDivergence> {
    let mut kernel = Kernel::new(policy);
    let mut fresh = crate::transcript::Transcript::new();
    for input in inputs {
        kernel.handle(input.clone(), &mut fresh);
    }
    if fresh.len() != recorded.len() {
        return Err(ReplayDivergence::Length {
            expected: recorded.len(),
            got: fresh.len(),
        });
    }
    for (a, b) in fresh.events().iter().zip(recorded.events()) {
        if a.digest != b.digest {
            return Err(ReplayDivergence::Event { seq: b.seq });
        }
    }
    Ok(())
}
