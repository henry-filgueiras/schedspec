//! Permutation rank: seeded, reconstructable, accountable ordering.
//!
//! PINNED (P2, closes PERMUTATION_RANK.md "the exact hash or ranking
//! function is intentionally open"): candidates are ordered by
//! `(blake3::keyed_hash(K, candidate_bytes), candidate_id)` where
//! `K = blake3("resonant/rank/v1" || canonical(seed))`. Properties:
//! - **accountable determinism**: any observer holding the visible seed
//!   and pool reconstructs the exact order (`reconstruct` verifies this);
//! - **stability under pool change**: adding or removing a candidate does
//!   not reshuffle the others (per-candidate hashing, not a permutation
//!   index);
//! - **hotspot damping**: `round` is part of the seed, so selection
//!   pressure rotates every round (OPEN_QUESTIONS "hotspot risk");
//! - **total order**: the candidate id is the final component, so equal
//!   tokens (32-byte collisions aside) cannot produce ambiguous ranks.
//!
//! PINNED (P3 boundary): rank orders and tie-breaks; it never resolves a
//! material conflict. The merge engine's cross-class comparator has no
//! rank parameter — see `merge::engine`.

use crate::epoch::{Epoch, Round};
use crate::id::{CanonicalBytes, PeerId, SubjectId};
use crate::scope::ScopeId;
use serde::{Deserialize, Serialize};

/// What the selection is for. Domain separation prevents one context's
/// order from leaking authority into another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RankDomain {
    WitnessSelection,
    Dissemination,
    Rendezvous,
    Repair,
}

impl RankDomain {
    fn tag(self) -> &'static str {
        match self {
            RankDomain::WitnessSelection => "witness-selection",
            RankDomain::Dissemination => "dissemination",
            RankDomain::Rendezvous => "rendezvous",
            RankDomain::Repair => "repair",
        }
    }
}

/// Everything that determines an ordering. All fields are protocol-visible
/// values, which is what makes the ordering reconstructable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankSeed {
    pub domain: RankDomain,
    pub scope: ScopeId,
    pub subject: Option<SubjectId>,
    pub epoch: Epoch,
    pub round: Round,
}

impl CanonicalBytes for RankSeed {
    fn write_canonical(&self, out: &mut Vec<u8>) {
        self.domain.tag().write_canonical(out);
        self.scope.write_canonical(out);
        match &self.subject {
            Some(subject) => {
                out.push(1);
                subject.write_canonical(out);
            }
            None => out.push(0),
        }
        self.epoch.get().write_canonical(out);
        self.round.get().write_canonical(out);
    }
}

/// A candidate's position token under a seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RankToken(pub [u8; 32]);

impl RankToken {
    pub fn short_hex(&self) -> String {
        self.0[..6].iter().map(|b| format!("{b:02x}")).collect()
    }
}

pub fn rank_token(seed: &RankSeed, candidate: &PeerId) -> RankToken {
    let key = seed.digest("resonant/rank/v1");
    let mut hasher = blake3::Hasher::new_keyed(&key);
    hasher.update(&candidate.canonical_bytes());
    RankToken(*hasher.finalize().as_bytes())
}

/// Why a pool member was not eligible. Exclusions travel with the
/// selection so candidate formation stays inspectable (MECHANICS.md
/// `form_candidates`). Topology speaks only here — as an eligibility
/// annotation, never as meaning (P10).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExclusionReason {
    TopologyPolicy(String),
    Quarantined,
    Removed,
    StaleEpoch,
    OperatorHold,
}

/// The accountable selection object (PRIMITIVES.md WitnessSet): seed,
/// pool, exclusions with reasons, full ranked order, and the taken slice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankedSelection {
    pub seed: RankSeed,
    pub seed_digest: [u8; 32],
    pub pool: Vec<PeerId>,
    pub excluded: Vec<(PeerId, ExclusionReason)>,
    pub ranked: Vec<(PeerId, RankToken)>,
    pub selected: Vec<PeerId>,
}

/// Deterministically rank the eligible pool under `seed` and select the
/// first `take` peers.
pub fn permutation_rank(
    seed: RankSeed,
    pool: Vec<PeerId>,
    excluded: Vec<(PeerId, ExclusionReason)>,
    take: usize,
) -> RankedSelection {
    let mut ranked: Vec<(PeerId, RankToken)> = pool
        .iter()
        .filter(|peer| !excluded.iter().any(|(ex, _)| ex == *peer))
        .map(|peer| (peer.clone(), rank_token(&seed, peer)))
        .collect();
    ranked.sort_by(|(peer_a, tok_a), (peer_b, tok_b)| tok_a.cmp(tok_b).then(peer_a.cmp(peer_b)));
    let selected = ranked
        .iter()
        .take(take)
        .map(|(peer, _)| peer.clone())
        .collect();
    let seed_digest = seed.digest("resonant/rank-seed/v1");
    RankedSelection {
        seed,
        seed_digest,
        pool,
        excluded,
        ranked,
        selected,
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum RankDivergence {
    #[error("seed digest does not match the recorded seed")]
    SeedDigest,
    #[error("recomputed order differs from the recorded order at position {position}")]
    Order { position: usize },
    #[error("recorded selection is not the head of the recomputed order")]
    Selection,
}

/// Independently recompute a recorded selection from its visible seed and
/// pool. This is the "any observer can reconstruct why" contract.
pub fn reconstruct(selection: &RankedSelection) -> Result<(), RankDivergence> {
    let fresh = permutation_rank(
        selection.seed.clone(),
        selection.pool.clone(),
        selection.excluded.clone(),
        selection.selected.len(),
    );
    if fresh.seed_digest != selection.seed_digest {
        return Err(RankDivergence::SeedDigest);
    }
    for (position, (recorded, fresh)) in
        selection.ranked.iter().zip(fresh.ranked.iter()).enumerate()
    {
        if recorded != fresh {
            return Err(RankDivergence::Order { position });
        }
    }
    if selection.ranked.len() != fresh.ranked.len() {
        return Err(RankDivergence::Order {
            position: fresh.ranked.len().min(selection.ranked.len()),
        });
    }
    if selection.selected != fresh.selected {
        return Err(RankDivergence::Selection);
    }
    Ok(())
}
