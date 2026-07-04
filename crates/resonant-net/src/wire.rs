//! Versioned wire envelopes. serde_json for the demo phase — readable
//! frames beat compact ones while debugging — and the encode/decode
//! boundary lives entirely in this module so a later flip to postcard or
//! CBOR is one-file work. Identity hashes come from `CanonicalBytes`,
//! never from serde, so the wire format can never change ids.

use resonant_kernel::belief::BeliefState;
use resonant_kernel::digest::RepairDigest;
use resonant_kernel::evidence::{Claim, WitnessRecord};
use resonant_kernel::merge::engine::MergeSide;
use resonant_kernel::operator::OperatorOverride;
use resonant_kernel::scope::ScopeId;
use serde::{Deserialize, Serialize};

pub const WIRE_VERSION: u16 = 1;

/// Everything that travels over gossipsub.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GossipMsg {
    Chat {
        text: String,
    },
    Claim(Claim),
    Witness(WitnessRecord),
    Digest(RepairDigest),
    Moderation {
        subject: String,
        action: Moderation,
        reason: String,
    },
    Override(OperatorOverride),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Moderation {
    Mute,
    Ban,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    pub v: u16,
    pub msg: GossipMsg,
}

pub fn encode(msg: GossipMsg) -> Vec<u8> {
    serde_json::to_vec(&Envelope {
        v: WIRE_VERSION,
        msg,
    })
    .expect("wire types serialize")
}

pub fn decode(data: &[u8]) -> Result<GossipMsg, String> {
    let envelope: Envelope =
        serde_json::from_slice(data).map_err(|e| format!("undecodable frame: {e}"))?;
    if envelope.v != WIRE_VERSION {
        return Err(format!("unsupported wire version {}", envelope.v));
    }
    Ok(envelope.msg)
}

/// Request-response protocol for reunion negotiation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RrRequest {
    /// Fetch the peer's full side for a scope.
    FullView { scope: ScopeId },
    /// Confirm agreed reunion coordinates. Carries the requester's side so
    /// the responder can apply the identical reunion before acking — both
    /// parties converge in one exchange, by construction.
    ReunionAck {
        scope: ScopeId,
        round: u64,
        digest_lo: [u8; 32],
        digest_hi: [u8; 32],
        requester_side: MergeSide,
        requester_digest: [u8; 32],
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RrResponse {
    FullView(Box<FullViewResponse>),
    Ack {
        agree: bool,
    },
    /// The responder had no view for that scope.
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullViewResponse {
    pub side: MergeSide,
    pub digest: RepairDigest,
    /// The responder's current kernel round, for rendezvous derivation.
    pub round: u64,
}

/// Standing states rendered for the roster display.
pub fn state_glyph(state: BeliefState) -> &'static str {
    match state {
        BeliefState::Accepted => "●",
        BeliefState::Provisional => "◐",
        BeliefState::Witnessed => "○",
        BeliefState::Introduced => "·",
        BeliefState::Suspected => "?",
        BeliefState::Disputed => "‼",
        BeliefState::Quarantined => "◌",
        BeliefState::Removed => "✕",
        BeliefState::Unknown => " ",
    }
}
