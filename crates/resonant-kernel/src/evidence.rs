//! Evidence objects: observations, claims, witness records.
//!
//! `Observation` (local evidence) and `Claim` (transmissible assertion) are
//! separate types with no conversion between them, so received rumor can
//! never masquerade as direct contact (PRIMITIVES.md Observation invariant).
//! Claims and witness records reference observations by id only.
//!
//! PINNED (P13): the corroboration vocabulary is `Quality` × `Diversity`,
//! the smallest vocabulary that covers the treatise's scenario corpus.
//! `Laundered` is a first-class diversity value so the trust-laundering
//! failure mode (THREAT_MODEL.md) is representable in types, not prose.

use crate::epoch::Epoch;
use crate::id::{
    CanonicalBytes, ClaimId, ObservationId, PeerId, SubjectId, WitnessId, WitnessRecordId,
};
use crate::scope::ScopeId;
use crate::trust::TrustGrade;
use serde::{Deserialize, Serialize};
use std::fmt;

/// How a local observation was made. Directness stays legible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ObservationMode {
    DirectContact,
    Timeout,
    ChallengeResponse,
    TopologyEvidence,
    AdminInspection,
}

/// Local, non-transmissible evidence. Never appears in a claim or effect —
/// only its hash does.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Observation {
    pub observer: WitnessId,
    pub subject: SubjectId,
    pub mode: ObservationMode,
    pub epoch: Epoch,
}

impl Observation {
    pub fn id(&self) -> ObservationId {
        ObservationId::from_bytes(self.digest("resonant/observation/v1"))
    }
}

impl CanonicalBytes for Observation {
    fn write_canonical(&self, out: &mut Vec<u8>) {
        self.observer.write_canonical(out);
        self.subject.write_canonical(out);
        out.push(self.mode as u8);
        self.epoch.get().write_canonical(out);
    }
}

/// What a claim proposes about its subject (distinct from what any scope
/// believes — introduction is not acceptance, INVARIANTS.md #1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AssertedState {
    Present,
    Departed,
    Compromised,
}

/// Who introduced a claim and by what right.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Provenance {
    pub introducer: PeerId,
}

/// A transmissible assertion about a subject, scoped by construction: the
/// scope is part of the claim's meaning (PRIMITIVES.md Claim invariant).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claim {
    pub subject: SubjectId,
    pub asserted: AssertedState,
    pub scope: ScopeId,
    pub provenance: Provenance,
    pub epoch: Epoch,
    pub evidence: Vec<ObservationId>,
}

impl Claim {
    pub fn id(&self) -> ClaimId {
        ClaimId::from_bytes(self.digest("resonant/claim/v1"))
    }
}

impl CanonicalBytes for Claim {
    fn write_canonical(&self, out: &mut Vec<u8>) {
        self.subject.write_canonical(out);
        out.push(self.asserted as u8);
        self.scope.write_canonical(out);
        self.provenance.introducer.write_canonical(out);
        self.epoch.get().write_canonical(out);
        (self.evidence.len() as u64).write_canonical(out);
        for ev in &self.evidence {
            ev.write_canonical(out);
        }
    }
}

/// A witness's position. Stances never collapse into a single vote
/// (PRIMITIVES.md WitnessRecord invariant).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Stance {
    Corroborate,
    Dispute,
    Suspect,
    SupportRevocation,
}

impl fmt::Display for Stance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Stance::Corroborate => "corroborate",
            Stance::Dispute => "dispute",
            Stance::Suspect => "suspect",
            Stance::SupportRevocation => "support-revocation",
        };
        f.write_str(s)
    }
}

/// A protocol-visible record of witnessing: distinct from the witness (an
/// actor) and the observation (local evidence) per MEMBERSHIP.md's witness
/// vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessRecord {
    pub witness: WitnessId,
    pub subject: SubjectId,
    pub about: Option<ClaimId>,
    pub stance: Stance,
    pub observation: ObservationId,
    /// How the referenced observation was made. Carried on the record so
    /// directness stays legible to whoever weighs the corroboration
    /// (PRIMITIVES.md: "directness stays legible").
    pub mode: ObservationMode,
    pub scope: ScopeId,
    pub epoch: Epoch,
    /// Scoped trust contribution, annotated at assembly time.
    pub trust_context: TrustGrade,
}

impl WitnessRecord {
    pub fn id(&self) -> WitnessRecordId {
        WitnessRecordId::from_bytes(self.digest("resonant/witness-record/v1"))
    }
}

impl CanonicalBytes for WitnessRecord {
    fn write_canonical(&self, out: &mut Vec<u8>) {
        self.witness.write_canonical(out);
        self.subject.write_canonical(out);
        match &self.about {
            Some(claim) => {
                out.push(1);
                claim.write_canonical(out);
            }
            None => out.push(0),
        }
        out.push(self.stance as u8);
        self.observation.write_canonical(out);
        out.push(self.mode as u8);
        self.scope.write_canonical(out);
        self.epoch.get().write_canonical(out);
        out.push(self.trust_context.get());
    }
}

/// Witness quality — how good the corroboration paths are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    Weak,
    Mixed,
    Strong,
}

/// Witness diversity — how independent the corroboration paths are.
/// `Laundered` marks corroboration manufactured inside one collusion
/// surface; it is worth less than no diversity at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Diversity {
    Laundered,
    SingleScope,
    Mixed,
    CrossScope,
}

/// The corroboration shape of a belief: count, quality, diversity.
/// INVARIANTS.md #2 "witness count is not witness quality" is enforced
/// where this is consumed: quality and diversity feed dominance, count
/// exits only through the capped informer score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WitnessSummary {
    pub count: u32,
    pub quality: Quality,
    pub diversity: Diversity,
}
