//! Deterministic reunion: the merge precedence contract, enforced by types.
//!
//! The precedence contract (SEMANTICS.md:402-438, MERGE_AND_HEALING.md:
//! 56-92) says: {provenance admissibility, scope authority, freshness,
//! trust weight, corroboration quality/diversity} may dominate; raw
//! witness count may inform but never dominate; deterministic ordering may
//! only tie-break; unresolved material conflict must survive as residue.
//!
//! Structural enforcement, not review discipline:
//! - the admissibility gate drops fragments before any comparison
//!   (`Admissibility`);
//! - cross-class dominance decisions consume `DominanceEvidence`, a type
//!   with **no witness-count field and no rank field** — the comparator's
//!   signature cannot consult them (P3, P7);
//! - the capped `InformerScore` is count's only exit, and it is consulted
//!   only *within* a semantic class, where the choice affects which record
//!   represents the belief, never what the belief is;
//! - `MergeResolution::ScopedDisagreement` carries `NonEmpty<Residue>` —
//!   unresolved conflict without residue is unrepresentable;
//! - `ReunionOutcome` is `#[must_use]` and its only consumption path,
//!   `apply_to`, routes every projection through `BeliefCell::apply` and
//!   every residue into the view's ledger.

pub mod engine;

use crate::belief::BeliefState;
use crate::epoch::Epoch;
use crate::evidence::WitnessSummary;
use crate::id::OverrideId;
use crate::residue::Residue;
use crate::scope::ScopeAuthority;
use crate::trust::TrustGrade;
use crate::util::NonEmpty;
use serde::{Deserialize, Serialize};

/// One side's belief about one subject, as brought into reunion.
/// Deliberately mirrors the scenario corpus's member entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeliefFragment {
    pub state: BeliefState,
    pub epoch: Epoch,
    pub trust: TrustGrade,
    pub authority: ScopeAuthority,
    pub witness: WitnessSummary,
}

/// Tier 0: the gate. Inadmissible fragments never reach a comparator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Admissibility {
    Admissible,
    Inadmissible(InadmissibleReason),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InadmissibleReason {
    /// No belief was ever formed.
    NoBelief,
    /// Zero effective trust: the fragment has no standing to speak.
    NoTrust,
}

/// Which side of a two-party reunion, or both, or an override.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeSource {
    None,
    SideA,
    SideB,
    Both,
    Override,
}

/// Stability of a merged belief. Independent of residue: a merge can be
/// stable and still carry a visible scar (the epoch-race case).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MergeStability {
    Stable,
    Provisional,
    StableWithOverride,
}

/// What actually decided a merge — the one-word answer to "what dominated
/// and what did not" (INVARIANTS.md #9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecidedBy {
    /// The admissibility gate left zero or one sides standing.
    AdmissibilityGate,
    /// Both sides asserted the same state.
    StateAgreement,
    /// Corroboration quality/diversity dominated (e.g. laundering
    /// discounted).
    Corroboration,
    /// Epoch freshness dominated.
    Freshness,
    /// Trust weight and scope authority dominated.
    TrustAndAuthority,
    /// The chosen side's status was preferred between compatible
    /// permissive peers.
    StatusPreference,
    /// The capped witness-count informer decided a within-class choice.
    /// Property-tested to never decide across classes.
    Informer,
    /// Deterministic input order broke a full tie (the two-party analogue
    /// of a rank-token tie-break; named, not hidden).
    InputOrder,
    /// Nothing was allowed to dominate: the conflict is preserved.
    Unresolved,
    Override,
}

/// Which rule of the pinned calculus fired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuleId {
    NoAdmissibleInput,
    SingleSided,
    CleanConvergence,
    LaunderingDiscount,
    SameEpochDispute,
    FreshRestrictiveDominates,
    RestrictiveDominance,
    ConflictSurvived,
    FreshnessDowngrade,
    PermissiveConverged,
    Fallback,
    OverrideApplied,
}

impl RuleId {
    pub fn describe(self) -> &'static str {
        match self {
            RuleId::NoAdmissibleInput => "No admissible input survived for this subject.",
            RuleId::SingleSided => "Single-sided admissible state carried forward.",
            RuleId::CleanConvergence => "Matching local outcomes converge cleanly.",
            RuleId::LaunderingDiscount => {
                "Trust and corroboration quality dominated raw witness count."
            }
            RuleId::SameEpochDispute => "Same-epoch high-trust conflict survives as residue.",
            RuleId::FreshRestrictiveDominates => {
                "Fresh restrictive evidence dominated older permissive evidence."
            }
            RuleId::RestrictiveDominance => {
                "Trust and scope authority favored the restrictive path."
            }
            RuleId::ConflictSurvived => "Conflict survived the deterministic reunion pass.",
            RuleId::FreshnessDowngrade => "Freshness dominated older permissive acceptance.",
            RuleId::PermissiveConverged => {
                "Permissive paths converged under freshness and trust weighting."
            }
            RuleId::Fallback => {
                "Fallback deterministic comparison retained the stronger admissible path."
            }
            RuleId::OverrideApplied => "OperatorOverride applied visibly.",
        }
    }
}

/// The typed outcome classes. These project onto the canonical nine states
/// (`project`) — they are a projection, not a second lifecycle (P1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeResolution {
    /// Stable convergence. May still carry residue (a visible race scar).
    Converged {
        state: BeliefState,
        decided_by: DecidedBy,
        residue: Vec<Residue>,
    },
    /// Usable but tentative convergence.
    ProvisionalConverged {
        state: BeliefState,
        decided_by: DecidedBy,
        residue: Vec<Residue>,
    },
    /// Material conflict that nothing was allowed to dominate. Residue is
    /// mandatory *by type*: there is no empty constructor for `NonEmpty`.
    ScopedDisagreement { residue: NonEmpty<Residue> },
    /// A visible operator intervention.
    Overridden {
        override_id: OverrideId,
        forced: BeliefState,
        residue: Vec<Residue>,
    },
}

impl MergeResolution {
    /// Total projection onto the canonical states.
    pub fn project(&self) -> BeliefState {
        match self {
            MergeResolution::Converged { state, .. }
            | MergeResolution::ProvisionalConverged { state, .. } => *state,
            MergeResolution::ScopedDisagreement { .. } => BeliefState::Disputed,
            MergeResolution::Overridden { forced, .. } => *forced,
        }
    }

    pub fn decided_by(&self) -> DecidedBy {
        match self {
            MergeResolution::Converged { decided_by, .. }
            | MergeResolution::ProvisionalConverged { decided_by, .. } => *decided_by,
            MergeResolution::ScopedDisagreement { .. } => DecidedBy::Unresolved,
            MergeResolution::Overridden { .. } => DecidedBy::Override,
        }
    }

    pub fn stability(&self) -> MergeStability {
        match self {
            MergeResolution::Converged { .. } => MergeStability::Stable,
            MergeResolution::ProvisionalConverged { .. }
            | MergeResolution::ScopedDisagreement { .. } => MergeStability::Provisional,
            MergeResolution::Overridden { .. } => MergeStability::StableWithOverride,
        }
    }

    pub fn residue(&self) -> Vec<&Residue> {
        match self {
            MergeResolution::Converged { residue, .. }
            | MergeResolution::ProvisionalConverged { residue, .. }
            | MergeResolution::Overridden { residue, .. } => residue.iter().collect(),
            MergeResolution::ScopedDisagreement { residue } => residue.iter().collect(),
        }
    }
}

/// Per-subject merge trace: the merge explaining itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeTrace {
    pub rule: RuleId,
    pub decided_by: DecidedBy,
    /// Sides dropped at the gate, with reasons.
    pub gate_drops: Vec<(MergeSource, InadmissibleReason)>,
    /// Count-free dominance scores per side (None if not admitted).
    pub dominance: [Option<i64>; 2],
    /// Capped informer scores per side.
    pub informer: [Option<i64>; 2],
    pub notes: Vec<String>,
}

/// One subject's reunion result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubjectMergeOutcome {
    pub subject: crate::id::SubjectId,
    pub resolution: MergeResolution,
    pub trace: MergeTrace,
    pub source: MergeSource,
    pub epoch: Epoch,
    pub trust: TrustGrade,
}

/// Overall reunion outcome classes (mirrors the lab digest vocabulary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OverallOutcome {
    Stable,
    Provisional,
    StableWithOverride,
    ProvisionalWithOverride,
}

/// The reunion digest: outcome, inputs, rules fired, unresolved residue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReunionDigest {
    pub overall: OverallOutcome,
    pub compared_subjects: usize,
    pub rules_fired: Vec<RuleId>,
    pub unresolved_residue: usize,
    pub handled_residue: usize,
}
