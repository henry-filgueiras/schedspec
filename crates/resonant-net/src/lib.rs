//! # resonant-net
//!
//! The transport shell that carries the sans-IO `resonant-kernel` over
//! libp2p: gossipsub for claims/digests/app-chatter, request-response for
//! reunion negotiation, ping/identify for presence, block-lists for
//! partition demos. One tokio event loop; the kernel never crosses an
//! `.await`; every stimulus is a loggable, replay-verifiable `Input`.
//!
//! Application skins (`resonant-node` chat, `resonant-blockd` federated
//! blocklists, the `resonant-tui` demo) differ only by an [`node::AppProfile`]
//! and which commands they expose — the standing semantics, evidence
//! bookkeeping, and deterministic reunion are identical.
#![forbid(unsafe_code)]

pub mod node;
pub mod wire;
