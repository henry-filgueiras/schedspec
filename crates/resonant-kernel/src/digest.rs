//! Repair digests: honest summaries of scoped belief.
//!
//! PINNED (P5, closes SPEC_AUDIT High #3 "digest semantics under-specified
//! relative to their load-bearing role"): a `RepairDigest` is constructible
//! only from a live view (`RepairDigest::of`), must preserve scope, epoch,
//! per-state counts, a content hash, the live residue ids (ids are never
//! dropped), and per-subject (state, epoch) summaries; it may omit all
//! evidence bodies. The honesty flags are computed from the ledger, not
//! asserted by the sender. Digest comparison can trigger detail fetch or a
//! repair hold — but the merge engine consumes only full fragments, so a
//! summary can never be merged as if it were evidence.

use crate::belief::{BeliefState, MembershipView};
use crate::epoch::Epoch;
use crate::id::{write_str, CanonicalBytes, ResidueId, SubjectId};
use crate::scope::ScopeId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairDigest {
    pub scope: ScopeId,
    pub epoch: Epoch,
    pub state_counts: BTreeMap<BeliefState, usize>,
    /// Per-subject (state, epoch): summary, never evidence.
    pub subject_summaries: BTreeMap<SubjectId, (BeliefState, Epoch)>,
    /// Live residue ids. A digest that misrepresented unresolved
    /// disagreement as settled would have to lie here — and the field is
    /// computed, not asserted.
    pub residue_ids: Vec<ResidueId>,
    pub unhandled_residue: usize,
    pub live_disputes: usize,
    pub content_hash: [u8; 32],
}

impl RepairDigest {
    /// The only constructor: honesty is computed from the view.
    pub fn of(view: &MembershipView) -> Self {
        let mut state_counts: BTreeMap<BeliefState, usize> = BTreeMap::new();
        let mut subject_summaries = BTreeMap::new();
        let mut live_disputes = 0;
        for (subject, cell) in view.subjects() {
            *state_counts.entry(cell.state()).or_insert(0) += 1;
            if cell.state() == BeliefState::Disputed {
                live_disputes += 1;
            }
            subject_summaries.insert(subject.clone(), (cell.state(), cell.epoch()));
        }
        let residue_ids = view.residue().ids();
        let unhandled_residue = view.residue().unhandled().count();

        let mut bytes = Vec::new();
        view.scope().write_canonical(&mut bytes);
        view.epoch().get().write_canonical(&mut bytes);
        for (subject, (state, epoch)) in &subject_summaries {
            subject.write_canonical(&mut bytes);
            write_str(&mut bytes, state.as_str());
            epoch.get().write_canonical(&mut bytes);
        }
        for id in &residue_ids {
            id.write_canonical(&mut bytes);
        }
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"resonant/repair-digest/v1");
        hasher.update(&bytes);

        Self {
            scope: view.scope().clone(),
            epoch: view.epoch(),
            state_counts,
            subject_summaries,
            residue_ids,
            unhandled_residue,
            live_disputes,
            content_hash: *hasher.finalize().as_bytes(),
        }
    }

    pub fn has_unresolved_disagreement(&self) -> bool {
        self.unhandled_residue > 0 || self.live_disputes > 0
    }
}

/// What comparing digests licenses. Never a merge: `FetchDetail` asks for
/// full fragments, `HoldForRepair` suspends until reconciliation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigestVerdict {
    NoAction,
    FetchDetail(Vec<SubjectId>),
    HoldForRepair(String),
}

/// Compare a local digest with a remote one from the same scope.
pub fn compare(local: &RepairDigest, remote: &RepairDigest) -> DigestVerdict {
    if local.scope != remote.scope {
        return DigestVerdict::HoldForRepair(format!(
            "scope mismatch: {} vs {} — cross-scope digests do not reconcile by fetch",
            local.scope, remote.scope
        ));
    }
    if local.content_hash == remote.content_hash {
        return DigestVerdict::NoAction;
    }
    if remote.has_unresolved_disagreement() {
        return DigestVerdict::HoldForRepair(
            "remote view carries unresolved disagreement; hold for deterministic reunion".into(),
        );
    }
    let differing: Vec<SubjectId> = local
        .subject_summaries
        .iter()
        .filter(|(subject, summary)| remote.subject_summaries.get(*subject) != Some(summary))
        .map(|(subject, _)| subject.clone())
        .chain(
            remote
                .subject_summaries
                .keys()
                .filter(|s| !local.subject_summaries.contains_key(*s))
                .cloned(),
        )
        .collect();
    DigestVerdict::FetchDetail(differing)
}
