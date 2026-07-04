//! Policy-shaped knobs, with pinned reference defaults.
//!
//! PINNED (P6, closes SEMANTICS.md:241-249 "exact quarantine policy —
//! intentionally open" and TRUST.md's unquantified hysteresis): the
//! defaults below are one concrete, small, overridable instantiation.
//! Strengthening is slow (hysteresis), weakening is immediate — the
//! conservative reading of TRUST.md's safety asymmetry.
//!
//! PINNED (P7, closes SPEC_AUDIT Medium #5 "merge precedence duplicated
//! with no owner" and OPEN_QUESTIONS "merge precedence looseness"):
//! `MergePolicy::lab_compat()` pins the score tables and margins to the
//! Deterministic Reunion Lab's constants (docs/deterministic-reunion-lab/
//! lab.js lines 2-17), making the JS lab and this kernel two renderings of
//! one calculus. Margins live here, in policy — never in the comparator.

use crate::evidence::{Diversity, Quality};
use crate::scope::ScopeAuthority;
use serde::{Deserialize, Serialize};

/// The full policy surface of the kernel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyBundle {
    /// Rounds a cell must hold its state before a strengthening
    /// transition (`CorroborationReached`) is allowed.
    pub strengthen_hysteresis_rounds: u64,
    /// Rounds in quarantine before release is allowed (release also
    /// requires fresh corroboration by construction of the event).
    pub quarantine_release_rounds: u64,
    /// Evidence older than this many epochs behind the merging scope's
    /// current epoch is classified `Stale`.
    pub stale_after_epochs: u64,
    pub merge: MergePolicy,
}

impl Default for PolicyBundle {
    fn default() -> Self {
        Self {
            strengthen_hysteresis_rounds: 2,
            quarantine_release_rounds: 2,
            stale_after_epochs: 2,
            merge: MergePolicy::lab_compat(),
        }
    }
}

/// Score tables and margins for the merge calculus. All integers; the
/// kernel has no floats.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergePolicy {
    pub quality_strong: i64,
    pub quality_mixed: i64,
    pub quality_weak: i64,
    pub diversity_cross_scope: i64,
    pub diversity_mixed: i64,
    pub diversity_single_scope: i64,
    pub diversity_laundered: i64,
    pub scope_global: i64,
    pub scope_regional: i64,
    pub scope_local: i64,
    /// Cap on the witness-count informer: `min(count * 2, cap)`. The cap is
    /// why raw count can inform but never overwhelm — its total possible
    /// contribution is bounded.
    pub informer_cap: i64,
    /// One-sided evidence is only `stable` at or above this score.
    pub one_sided_stable: i64,
    /// choosePermissive: a fresher permissive entry wins if its score is
    /// within this slack of the other side.
    pub permissive_epoch_slack: i64,
    /// choosePermissive: score margin that decides between permissive peers.
    pub permissive_margin: i64,
    /// Laundering discount: the clean side wins if its score is within this
    /// tolerance below the laundered side.
    pub laundering_tolerance: i64,
    /// Same-epoch restrictive/permissive conflict closer than this is an
    /// honest dispute — the canonical residue case.
    pub dispute_closeness: i64,
    /// A fresher restrictive entry dominates if its score is at least
    /// (other - slack).
    pub restrictive_fresh_slack: i64,
    /// A same-or-older restrictive entry needs this much score advantage to
    /// dominate.
    pub restrictive_dominance: i64,
    /// Both-permissive convergence is only `stable` if the accepted side
    /// leads by this margin.
    pub accept_converge_margin: i64,
}

impl MergePolicy {
    /// The Deterministic Reunion Lab's exact constants.
    pub fn lab_compat() -> Self {
        Self {
            quality_strong: 22,
            quality_mixed: 10,
            quality_weak: -8,
            diversity_cross_scope: 14,
            diversity_mixed: 6,
            diversity_single_scope: 0,
            diversity_laundered: -16,
            scope_global: 12,
            scope_regional: 4,
            scope_local: 0,
            informer_cap: 10,
            one_sided_stable: 95,
            permissive_epoch_slack: 12,
            permissive_margin: 4,
            laundering_tolerance: 8,
            dispute_closeness: 14,
            restrictive_fresh_slack: 6,
            restrictive_dominance: 14,
            accept_converge_margin: 8,
        }
    }

    pub fn quality_score(&self, quality: Quality) -> i64 {
        match quality {
            Quality::Strong => self.quality_strong,
            Quality::Mixed => self.quality_mixed,
            Quality::Weak => self.quality_weak,
        }
    }

    pub fn diversity_score(&self, diversity: Diversity) -> i64 {
        match diversity {
            Diversity::CrossScope => self.diversity_cross_scope,
            Diversity::Mixed => self.diversity_mixed,
            Diversity::SingleScope => self.diversity_single_scope,
            Diversity::Laundered => self.diversity_laundered,
        }
    }

    pub fn authority_score(&self, authority: ScopeAuthority) -> i64 {
        match authority {
            ScopeAuthority::Global => self.scope_global,
            ScopeAuthority::Regional => self.scope_regional,
            ScopeAuthority::Local => self.scope_local,
        }
    }

    pub fn informer_score(&self, count: u32) -> i64 {
        (i64::from(count) * 2).min(self.informer_cap)
    }
}
