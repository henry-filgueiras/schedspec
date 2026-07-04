//! Residue: unresolved disagreement that must remain visible.
//!
//! INVARIANTS.md #4 enforced by API shape: `Residue` has no public
//! constructor (only the merge engine and quarantine paths create it) and
//! `ResidueLedger` has no public remove. The only exits are `resolve` (with
//! evidence) and `supersede` (visibly replaced by a fresher tension), both
//! of which are transcripted by their callers.
//!
//! PINNED (P11, closes OPEN_QUESTIONS "residue growth vs operational
//! usefulness"): growth is bounded by superseding same-tension entries —
//! a visible replacement, never a silent TTL. Compaction may summarize
//! but ids and counts are never erased.

use crate::belief::state::BeliefState;
use crate::epoch::{Epoch, Round};
use crate::id::{write_str, CanonicalBytes, OverrideId, ResidueId, SubjectId, WitnessRecordId};
use crate::scope::ScopeId;
use crate::util::NonEmpty;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

/// The key of scoped belief: different scopes may legitimately hold
/// different views of the same subject at the same time.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BeliefKey {
    pub scope: ScopeId,
    pub subject: SubjectId,
}

/// One side of a preserved conflict.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TensionSide {
    /// Which view/island/source contributed this position.
    pub source: String,
    pub state: BeliefState,
    pub epoch: Epoch,
}

/// The opposing positions that could not be honestly flattened.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ConflictTension {
    pub sides: Vec<TensionSide>,
    pub detail: String,
}

impl ConflictTension {
    /// Tension content, for display and audit. Deliberately NOT part of
    /// residue identity — see `Residue::new`.
    fn content_bytes(&self, out: &mut Vec<u8>) {
        for side in &self.sides {
            write_str(out, &side.source);
            write_str(out, side.state.as_str());
        }
        write_str(out, &self.detail);
    }
}

/// What justifies closing a residue entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionEvidence {
    /// Fresh corroboration settled the tension.
    Corroborated(NonEmpty<WitnessRecordId>),
    /// A visible operator override took responsibility for it.
    Override(OverrideId),
    /// A later merge converged stably on this subject with no surviving
    /// conflict: the tension resolved itself. The merge input digest is
    /// the evidence.
    StableConvergence { input_digest: [u8; 32] },
}

/// A preserved piece of unresolved disagreement. `#[must_use]` and
/// privately constructed: there is no public path that creates or discards
/// one silently.
#[must_use = "residue must be preserved in a ledger or explicitly resolved; dropping it violates INVARIANTS.md 'residue must remain visible'"]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Residue {
    id: ResidueId,
    key: BeliefKey,
    tension: ConflictTension,
    born: (Epoch, Round),
    /// Set when a visible override took responsibility. The residue stays
    /// in the ledger — an override marks a scar, it does not erase one.
    handled_by: Option<OverrideId>,
}

impl Residue {
    /// PINNED (P20, forced by wire testing): residue identity is
    /// content-addressed on the belief key alone — "there is unresolved
    /// disagreement about this subject in this scope". The tension sides,
    /// detail, and birth coordinates are content, not identity. This is
    /// what makes cascaded pairwise reunions converge: when A⟷C reunite
    /// first and B⟷C reunite against C's already-merged view, the two
    /// reunions see different tension shapes (removed-vs-accepted, then
    /// removed-vs-disputed) but mint the *same* residue id, so ledgers —
    /// and therefore repair digests — reach a fixpoint instead of
    /// diverging forever on residue identity.
    pub(crate) fn new(key: BeliefKey, tension: ConflictTension, born: (Epoch, Round)) -> Self {
        let mut bytes = Vec::new();
        key.scope.write_canonical(&mut bytes);
        key.subject.write_canonical(&mut bytes);
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"resonant/residue/v2");
        hasher.update(&bytes);
        let id = ResidueId::from_bytes(*hasher.finalize().as_bytes());
        Self {
            id,
            key,
            tension,
            born,
            handled_by: None,
        }
    }

    pub fn id(&self) -> ResidueId {
        self.id
    }

    pub fn key(&self) -> &BeliefKey {
        &self.key
    }

    pub fn tension(&self) -> &ConflictTension {
        &self.tension
    }

    pub fn born(&self) -> (Epoch, Round) {
        self.born
    }

    pub fn handled_by(&self) -> Option<&OverrideId> {
        self.handled_by.as_ref()
    }

    pub(crate) fn mark_handled(&mut self, by: OverrideId) {
        self.handled_by = Some(by);
    }

    /// Content digest (sides, detail, birth), used to distinguish a
    /// same-identity re-mint from an identical duplicate.
    fn content_digest(&self) -> [u8; 32] {
        let mut bytes = Vec::new();
        self.tension.content_bytes(&mut bytes);
        self.born.0.get().write_canonical(&mut bytes);
        self.born.1.get().write_canonical(&mut bytes);
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"resonant/residue-content/v1");
        hasher.update(&bytes);
        *hasher.finalize().as_bytes()
    }
}

/// A closed residue entry: the tension plus what closed it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedResidue {
    pub residue: Residue,
    pub evidence: ResolutionEvidence,
    pub at: (Epoch, Round),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ResidueError {
    #[error("no live residue with id {0}")]
    Unknown(ResidueId),
}

/// The ledger of live and resolved disagreement carried by every view.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidueLedger {
    entries: BTreeMap<ResidueId, Residue>,
    resolved: Vec<ResolvedResidue>,
}

impl ResidueLedger {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert new residue. Identity is subject-level (P20), so a fresh
    /// observation of the same subject's conflict supersedes the previous
    /// tension content in place (visible replacement, not silent dedup).
    /// An existing override responsibility mark survives the re-mint: an
    /// operator took responsibility for this subject's conflict, and a
    /// propagation race must not silently reopen the scar. Returns the id
    /// when an entry was superseded.
    pub(crate) fn insert(&mut self, mut residue: Residue) -> Option<ResidueId> {
        match self.entries.get(&residue.id()) {
            Some(existing) if existing.content_digest() == residue.content_digest() => None,
            Some(existing) => {
                if let Some(by) = existing.handled_by {
                    residue.handled_by = Some(by);
                }
                let id = residue.id();
                self.entries.insert(id, residue);
                Some(id)
            }
            None => {
                self.entries.insert(residue.id(), residue);
                None
            }
        }
    }

    /// Mark every live residue for a subject as handled by a visible
    /// override. The entries stay in the ledger — an override takes
    /// responsibility for a scar, it does not erase one. Returns how many
    /// entries were marked.
    pub(crate) fn mark_all_handled(&mut self, subject: &SubjectId, by: OverrideId) -> usize {
        let mut marked = 0;
        for residue in self.entries.values_mut() {
            if residue.key.subject == *subject && residue.handled_by.is_none() {
                residue.mark_handled(by);
                marked += 1;
            }
        }
        marked
    }

    /// Resolve every live residue for a subject with the given evidence.
    /// Entries move to the resolved record — history is never erased.
    pub(crate) fn resolve_for_subject(
        &mut self,
        subject: &SubjectId,
        evidence: &ResolutionEvidence,
        at: (Epoch, Round),
    ) -> Vec<ResolvedResidue> {
        let ids: Vec<ResidueId> = self
            .entries
            .values()
            .filter(|r| r.key.subject == *subject)
            .map(Residue::id)
            .collect();
        ids.into_iter()
            .filter_map(|id| self.resolve(id, evidence.clone(), at).ok())
            .collect()
    }

    /// Close a residue entry with evidence. The entry moves to the
    /// resolved record; its existence is never erased.
    pub fn resolve(
        &mut self,
        id: ResidueId,
        evidence: ResolutionEvidence,
        at: (Epoch, Round),
    ) -> Result<ResolvedResidue, ResidueError> {
        let residue = self.entries.remove(&id).ok_or(ResidueError::Unknown(id))?;
        let record = ResolvedResidue {
            residue,
            evidence,
            at,
        };
        self.resolved.push(record.clone());
        Ok(record)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Residue> {
        self.entries.values()
    }

    pub fn ids(&self) -> Vec<ResidueId> {
        self.entries.keys().copied().collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Live residue not yet taken responsibility for by an override.
    pub fn unhandled(&self) -> impl Iterator<Item = &Residue> {
        self.entries.values().filter(|r| r.handled_by.is_none())
    }

    pub fn resolved(&self) -> &[ResolvedResidue] {
        &self.resolved
    }
}
