//! # resonant-kernel
//!
//! Sans-IO deterministic reference kernel for the resonant membership
//! treatise (the `docs/` tree of this repository). Membership here is
//! *converging scoped belief*: what does this scope currently believe
//! about this subject, on what evidence, with what confidence, and under
//! what right to spread that belief further?
//!
//! Ground rules, enforced throughout:
//! - no networking, no async, no wall clock, no randomness, no floats;
//! - `BTreeMap`/`BTreeSet` only — iteration order is part of correctness;
//! - all hashing is domain-separated BLAKE3;
//! - every doc-flagged open question is pinned to one concrete reference
//!   decision, marked `PINNED` in the owning module and cataloged in
//!   `PINNED_DECISIONS.md`.
#![forbid(unsafe_code)]

pub mod belief;
pub mod digest;
pub mod epoch;
pub mod evidence;
pub mod id;
pub mod kernel;
pub mod merge;
pub mod operator;
pub mod policy;
pub mod rank;
pub mod residue;
pub mod scope;
pub mod transcript;
pub mod trust;
pub mod util;
pub mod witnessing;
