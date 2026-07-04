//! Trust weight, confidence, and the trust-root standing machine.
//!
//! PINNED (P8, closes SEMANTICS.md "whether weights are numeric, ordinal,
//! or category-based — intentionally open"): `TrustGrade` is an ordinal
//! 0..=100 with named bands, and `Confidence` is a separate enum with no
//! arithmetic bridge — the docs require the two "must remain distinct",
//! so they share no numeric domain.
//!
//! PINNED (P4, closes SPEC_AUDIT High #4 "trust-root lifecycle remains too
//! open" and OPEN_QUESTIONS "trust-root promotion/demotion"): trust-root
//! standing is a typed state machine per (root, scope). The `EarnedHistory`
//! basis — the docs' main re-entry path for hidden authority — must pass
//! through `Probation` carrying the witness history that justified it, and
//! widening to another scope is an explicit, transcripted event. Demotion
//! taints dependent merges, forcing residue (MERGE_AND_HEALING.md rule 3).

use crate::id::{OverrideId, TrustRootId, WitnessRecordId};
use crate::scope::ScopeId;
use crate::util::NonEmpty;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Ordinal, bucketed trust weight in 0..=100.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrustGrade(u8);

impl TrustGrade {
    pub fn new(grade: u8) -> Self {
        Self(grade.min(100))
    }

    pub fn get(self) -> u8 {
        self.0
    }

    pub fn band(self) -> TrustBand {
        match self.0 {
            0..=19 => TrustBand::Floor,
            20..=49 => TrustBand::Weak,
            50..=69 => TrustBand::Ordinary,
            70..=89 => TrustBand::Strong,
            _ => TrustBand::Foundational,
        }
    }
}

impl fmt::Display for TrustGrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.band(), self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TrustBand {
    Floor,
    Weak,
    Ordinary,
    Strong,
    Foundational,
}

impl fmt::Display for TrustBand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TrustBand::Floor => "floor",
            TrustBand::Weak => "weak",
            TrustBand::Ordinary => "ordinary",
            TrustBand::Strong => "strong",
            TrustBand::Foundational => "foundational",
        };
        f.write_str(s)
    }
}

/// Composite belief strength. Produced only from (trust, freshness,
/// corroboration, conflict pressure); never arithmetic on `TrustGrade`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Confidence {
    Collapsed,
    Low,
    Bounded,
    Strong,
}

/// Where a trust root's standing comes from. The basis is permanently
/// attached — earned standing stays distinguishable from installed
/// standing forever.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RootBasis {
    OperatorPolicy {
        operator_note: String,
    },
    IdentityLineage {
        lineage: String,
    },
    /// Standing earned from converged witness history. Must enter through
    /// probation, and carries the history that justified it.
    EarnedHistory {
        justification: NonEmpty<WitnessRecordId>,
    },
}

impl RootBasis {
    pub fn kind(&self) -> &'static str {
        match self {
            RootBasis::OperatorPolicy { .. } => "operator-policy",
            RootBasis::IdentityLineage { .. } => "identity-lineage",
            RootBasis::EarnedHistory { .. } => "earned-history",
        }
    }
}

/// Trust-root standing states, per (root, scope).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RootState {
    Proposed,
    Probation,
    Active,
    Narrowed,
    Suspended,
    Revoked,
}

impl fmt::Display for RootState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            RootState::Proposed => "proposed",
            RootState::Probation => "probation",
            RootState::Active => "active",
            RootState::Narrowed => "narrowed",
            RootState::Suspended => "suspended",
            RootState::Revoked => "revoked",
        };
        f.write_str(s)
    }
}

/// Events that move trust-root standing. Every promotion carries its
/// justification; every widening is explicit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RootEvent {
    /// Proposed -> Active (installed bases) or Proposed -> Probation
    /// (earned basis — no shortcut to Active).
    Admitted,
    /// Probation -> Active, carrying the fresh corroboration that ended
    /// probation.
    ProbationPassed {
        corroboration: NonEmpty<WitnessRecordId>,
    },
    /// Active -> Narrowed: standing reduced to a smaller scope.
    Narrowed { to_scope: ScopeId },
    /// Narrowed -> Active within the narrowed scope.
    Reaffirmed,
    /// Active | Narrowed | Probation -> Suspended.
    Suspended { reason: String },
    /// Suspended -> Active.
    Reinstated { review_note: String },
    /// Any live state -> Revoked. Terminal; revocation of standing is an
    /// event with support, mirroring subject-level revocation.
    Revoked { support: RevocationSupport },
}

/// What backs a revocation: attributable witness records or a visible
/// operator override — never anonymous.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevocationSupport {
    Witnessed(NonEmpty<WitnessRecordId>),
    Override(OverrideId),
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("trust root {root} in scope {scope}: invalid standing move {from} on {event}")]
pub struct InvalidRootMove {
    pub root: TrustRootId,
    pub scope: ScopeId,
    pub from: RootState,
    pub event: String,
}

/// The standing of one trust root in one scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRootStanding {
    root: TrustRootId,
    scope: ScopeId,
    basis: RootBasis,
    state: RootState,
    grade: TrustGrade,
    history: Vec<(RootState, RootState)>,
}

impl TrustRootStanding {
    /// A new root always starts `Proposed`, whatever its basis.
    pub fn propose(root: TrustRootId, scope: ScopeId, basis: RootBasis, grade: TrustGrade) -> Self {
        Self {
            root,
            scope,
            basis,
            state: RootState::Proposed,
            grade,
            history: Vec::new(),
        }
    }

    pub fn state(&self) -> RootState {
        self.state
    }

    pub fn basis(&self) -> &RootBasis {
        &self.basis
    }

    pub fn grade(&self) -> TrustGrade {
        self.grade
    }

    pub fn root(&self) -> &TrustRootId {
        &self.root
    }

    pub fn scope(&self) -> &ScopeId {
        &self.scope
    }

    /// Effective trust contributed to merges: zero unless standing is live.
    pub fn effective_grade(&self) -> TrustGrade {
        match self.state {
            RootState::Active | RootState::Narrowed => self.grade,
            RootState::Probation => TrustGrade::new(self.grade.get() / 2),
            _ => TrustGrade::new(0),
        }
    }

    /// Apply a standing event. Returns (from, to) on success.
    pub fn apply(&mut self, event: &RootEvent) -> Result<(RootState, RootState), InvalidRootMove> {
        use RootEvent as E;
        use RootState as S;
        let from = self.state;
        let to = match (from, event) {
            // Earned standing has no shortcut past probation.
            (S::Proposed, E::Admitted) => match self.basis {
                RootBasis::EarnedHistory { .. } => S::Probation,
                _ => S::Active,
            },
            (S::Probation, E::ProbationPassed { .. }) => S::Active,
            (S::Active, E::Narrowed { .. }) => S::Narrowed,
            (S::Narrowed, E::Reaffirmed) => S::Active,
            (S::Active | S::Narrowed | S::Probation, E::Suspended { .. }) => S::Suspended,
            (S::Suspended, E::Reinstated { .. }) => S::Active,
            (
                S::Proposed | S::Probation | S::Active | S::Narrowed | S::Suspended,
                E::Revoked { .. },
            ) => S::Revoked,
            _ => {
                return Err(InvalidRootMove {
                    root: self.root.clone(),
                    scope: self.scope.clone(),
                    from,
                    event: format!("{event:?}"),
                })
            }
        };
        self.state = to;
        self.history.push((from, to));
        Ok((from, to))
    }
}
