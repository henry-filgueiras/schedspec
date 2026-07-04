//! # resonant-node
//!
//! The chat skin over [`resonant_net`]: a P2P group-chat node where
//! moderation standing (member / mod / muted / banned) survives partitions
//! with honest deterministic merge, visible residue, and a replayable
//! decision transcript. All the machinery lives in `resonant-net`; this
//! crate re-exports it and ships the `resonant-chat` binary.
#![forbid(unsafe_code)]

pub use resonant_net::{node, wire};
