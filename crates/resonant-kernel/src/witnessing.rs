//! Evidence bookkeeping and witness-summary derivation.
//!
//! PINNED (P19, closes the gap the docs and lab both leave open: where a
//! `WitnessSummary` actually comes from): quality and diversity are
//! derived from the recorded evidence itself —
//!
//! - **count** is the number of distinct corroborating witnesses;
//! - **quality** comes from the observation modes behind the records:
//!   challenge-response and direct contact are strong evidence, timeouts
//!   and topology inference are weak. Mostly-strong modes with at least
//!   two witnesses grade `Strong`; mostly-weak modes grade `Weak`;
//!   anything else grades `Mixed`;
//! - **diversity** comes from the vouch lineage of the witnesses: every
//!   witness is traced to the root of its introduction chain, and the
//!   number of distinct roots is the number of genuinely independent
//!   corroboration paths. Three or more roots grade `CrossScope`, two
//!   grade `Mixed`, one grades `SingleScope` — and one root with a loud
//!   cluster (four or more witnesses) of non-strong quality grades
//!   `Laundered`: volume manufactured inside a single introduction
//!   lineage is worth less than no diversity at all.
//!
//! The `EvidenceBook` also builds `MergeSide`s from live views, replacing
//! hand-assembled fragments, and offers the standard advancement policy
//! (strengthen belief only when the summary clears quality and diversity
//! gates) shared by the theater harness and the network node.

use crate::belief::{BeliefState, MembershipView};
use crate::evidence::{
    Claim, Diversity, ObservationMode, Quality, Stance, WitnessRecord, WitnessSummary,
};
use crate::id::{SubjectId, WitnessId, WitnessRecordId};
use crate::merge::engine::MergeSide;
use crate::merge::BeliefFragment;
use crate::scope::{ScopeAuthority, ScopeId};
use crate::trust::TrustGrade;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Full witness records plus vouch lineage, per scope. This is the
/// node-local memory the kernel's belief cells deliberately do not carry
/// (cells keep record *ids*; the book keeps the records).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceBook {
    /// (scope, subject) -> record id -> record.
    records: BTreeMap<(ScopeId, SubjectId), BTreeMap<WitnessRecordId, WitnessRecord>>,
    /// (scope, member) -> who vouched them in. Fed from claim provenance.
    lineage: BTreeMap<(ScopeId, WitnessId), WitnessId>,
    /// Trust roots per scope. Lineage climbing stops *below* a trust
    /// root: in a room every chain eventually reaches the creator, so the
    /// meaningful independence unit is the branch just under the root —
    /// two members vouched directly by the creator are two independent
    /// lineages, while four puppets vouched by one member are one.
    trust_roots: BTreeMap<ScopeId, BTreeSet<WitnessId>>,
}

impl EvidenceBook {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a claim's provenance: the introducer becomes the subject's
    /// lineage parent (first introduction wins; re-vouching does not
    /// rewrite history).
    pub fn record_claim(&mut self, claim: &Claim) {
        let member = WitnessId::new(claim.subject.as_str());
        let voucher = WitnessId::new(claim.provenance.introducer.as_str());
        if member != voucher {
            self.lineage
                .entry((claim.scope.clone(), member))
                .or_insert(voucher);
        }
    }

    pub fn record_witness(&mut self, record: WitnessRecord) {
        self.records
            .entry((record.scope.clone(), record.subject.clone()))
            .or_default()
            .insert(record.id(), record);
    }

    /// Mark a witness as a trust root in a scope (e.g. the room creator).
    pub fn mark_trust_root(&mut self, scope: &ScopeId, witness: WitnessId) {
        self.trust_roots
            .entry(scope.clone())
            .or_default()
            .insert(witness);
    }

    fn is_trust_root(&self, scope: &ScopeId, witness: &WitnessId) -> bool {
        self.trust_roots
            .get(scope)
            .is_some_and(|roots| roots.contains(witness))
    }

    /// Follow the vouch chain upward (cycle-guarded), stopping at the last
    /// ancestor *below* a trust root: the independent lineage branch.
    pub fn lineage_root(&self, scope: &ScopeId, witness: &WitnessId) -> WitnessId {
        let mut current = witness.clone();
        let mut seen = BTreeSet::new();
        while let Some(parent) = self.lineage.get(&(scope.clone(), current.clone())) {
            if self.is_trust_root(scope, parent) || !seen.insert(current.clone()) {
                break;
            }
            current = parent.clone();
        }
        current
    }

    fn corroborating(&self, scope: &ScopeId, subject: &SubjectId) -> Vec<&WitnessRecord> {
        self.records
            .get(&(scope.clone(), subject.clone()))
            .map(|records| {
                records
                    .values()
                    .filter(|r| r.stance == Stance::Corroborate)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Corroborating record ids, for evidence-carrying belief events.
    pub fn corroborating_ids(&self, scope: &ScopeId, subject: &SubjectId) -> Vec<WitnessRecordId> {
        self.corroborating(scope, subject)
            .iter()
            .map(|r| r.id())
            .collect()
    }

    /// Derive the corroboration shape from the recorded evidence (P19).
    pub fn summarize(&self, scope: &ScopeId, subject: &SubjectId) -> WitnessSummary {
        let records = self.corroborating(scope, subject);
        let mut witnesses: BTreeSet<&WitnessId> = BTreeSet::new();
        let mut roots: BTreeSet<WitnessId> = BTreeSet::new();
        let mut strong_modes = 0usize;
        let mut weak_modes = 0usize;
        for record in &records {
            if witnesses.insert(&record.witness) {
                roots.insert(self.lineage_root(scope, &record.witness));
            }
            match observation_strength(record) {
                Strength::Strong => strong_modes += 1,
                Strength::Weak => weak_modes += 1,
            }
        }
        let count = witnesses.len() as u32;

        let quality = if count >= 2 && strong_modes * 3 >= records.len() * 2 {
            Quality::Strong
        } else if records.is_empty() || weak_modes * 2 >= records.len() {
            Quality::Weak
        } else {
            Quality::Mixed
        };

        let diversity = match roots.len() {
            0 | 1 => {
                if count >= 4 && quality != Quality::Strong {
                    Diversity::Laundered
                } else {
                    Diversity::SingleScope
                }
            }
            2 => Diversity::Mixed,
            _ => Diversity::CrossScope,
        };

        WitnessSummary {
            count,
            quality,
            diversity,
        }
    }

    /// The trust a subject's standing carries into a merge: the strongest
    /// trust context among its corroborating records (evidence-backed),
    /// with a floor for subjects that exist but have no corroboration yet.
    pub fn standing_trust(&self, scope: &ScopeId, subject: &SubjectId) -> TrustGrade {
        self.corroborating(scope, subject)
            .iter()
            .map(|r| r.trust_context)
            .max()
            .unwrap_or(TrustGrade::new(10))
    }

    /// Build one party's reunion side from a live view plus this book's
    /// evidence. Replaces hand-assembled fragments.
    pub fn merge_side(
        &self,
        label: impl Into<String>,
        view: &MembershipView,
        authority: ScopeAuthority,
    ) -> MergeSide {
        let scope = view.scope();
        MergeSide {
            label: label.into(),
            epoch: view.epoch(),
            fragments: view
                .subjects()
                .map(|(subject, cell)| {
                    let fragment = BeliefFragment {
                        state: cell.state(),
                        epoch: cell.epoch(),
                        trust: self.standing_trust(scope, subject),
                        authority,
                        witness: self.summarize(scope, subject),
                    };
                    (subject.clone(), fragment)
                })
                .collect(),
        }
    }
}

enum Strength {
    Strong,
    Weak,
}

fn observation_strength(record: &WitnessRecord) -> Strength {
    match record.mode {
        ObservationMode::ChallengeResponse
        | ObservationMode::DirectContact
        | ObservationMode::AdminInspection => Strength::Strong,
        ObservationMode::Timeout | ObservationMode::TopologyEvidence => Strength::Weak,
    }
}

/// The standard advancement gate: strengthen scoped belief only when the
/// derived summary clears both the quality bar and the laundering check.
/// This is the policy beat that makes the trust-laundering story fail at
/// the door — six loud sockpuppets never satisfy it.
pub fn clears_advancement_gate(summary: &WitnessSummary) -> bool {
    summary.count >= 2
        && summary.quality != Quality::Weak
        && summary.diversity != Diversity::Laundered
}

/// Subjects in a view that are ready to strengthen one step, given the
/// book's evidence. Returns (subject, corroborating ids) pairs; the caller
/// turns them into `CorroborationAssessed` inputs at its own cadence.
pub fn ready_to_advance(
    book: &EvidenceBook,
    view: &MembershipView,
) -> Vec<(SubjectId, Vec<WitnessRecordId>)> {
    view.subjects()
        .filter(|(_, cell)| {
            matches!(
                cell.state(),
                BeliefState::Witnessed | BeliefState::Provisional | BeliefState::Suspected
            )
        })
        .filter(|(subject, _)| clears_advancement_gate(&book.summarize(view.scope(), subject)))
        .map(|(subject, _)| {
            (
                subject.clone(),
                book.corroborating_ids(view.scope(), subject),
            )
        })
        .collect()
}

/// A deterministic rendezvous round shared by all reunion participants:
/// strictly ahead of every advertised local round, and varied by the
/// content of the two sides so distinct concurrent reunions mint distinct
/// residue identities (P18).
pub fn rendezvous_round(
    local_rounds: &[u64],
    digest_hash_a: &[u8; 32],
    digest_hash_b: &[u8; 32],
) -> crate::epoch::Round {
    const SPREAD: u64 = 16;
    let (lo, hi) = if digest_hash_a <= digest_hash_b {
        (digest_hash_a, digest_hash_b)
    } else {
        (digest_hash_b, digest_hash_a)
    };
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"resonant/reunion-rendezvous/v1");
    hasher.update(lo);
    hasher.update(hi);
    let digest = hasher.finalize();
    let jitter = u64::from_le_bytes(digest.as_bytes()[..8].try_into().expect("8 bytes")) % SPREAD;
    let max_round = local_rounds.iter().copied().max().unwrap_or(0);
    crate::epoch::Round(max_round + SPREAD + jitter)
}
