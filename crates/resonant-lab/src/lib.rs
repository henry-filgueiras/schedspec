//! # resonant-lab
//!
//! The conformance and demonstration layer around `resonant-kernel`:
//! - `scenario`: serde model for the canonical corpus in `docs/scenarios/`;
//! - `replay`: divergence-event replay (`materializeCurrentState`);
//! - `oracle`: a string-exact port of the in-browser Deterministic Reunion
//!   Lab's merge engine (docs/deterministic-reunion-lab/lab.js), used as
//!   the conformance oracle for the kernel's typed merge engine;
//! - `golden`: hand-derived expected outcomes per scenario, shared by the
//!   test suite and `resonant scenario verify`;
//! - `sim`: a seeded, fully deterministic partition/heal simulator.
#![forbid(unsafe_code)]

pub mod conformance;
pub mod golden;
pub mod oracle;
pub mod replay;
pub mod scenario;
pub mod sim;
