//! # resonant-node
//!
//! A P2P group-chat node with partition-surviving moderation: libp2p
//! (gossipsub + request-response) provides transport and liveness; the
//! sans-IO `resonant-kernel` provides membership *standing* — who is a
//! member, moderator, muted, or banned — with honest deterministic merge
//! after partitions, visible residue, and a replayable decision
//! transcript.
#![forbid(unsafe_code)]

pub mod node;
pub mod wire;
