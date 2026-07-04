//! Epochs, rounds, and freshness.
//!
//! PINNED (P9, closes SEMANTICS.md "exact clocking model — intentionally
//! open"): the kernel has no wall clock. Time is a caller-supplied per-scope
//! `Epoch` generation counter plus a kernel-step `Round` counter. Freshness
//! is epoch arithmetic against policy, so ordering stays "visible enough to
//! explain" — every record and transcript event carries its epoch.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Per-scope generation counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Epoch(pub u64);

impl Epoch {
    pub fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for Epoch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "e{}", self.0)
    }
}

/// Kernel step counter used for hysteresis and repair rounds.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Round(pub u64);

impl Round {
    pub fn get(self) -> u64 {
        self.0
    }

    pub fn next(self) -> Round {
        Round(self.0 + 1)
    }
}

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}

/// Freshness classification of evidence relative to the merging scope's
/// current epoch. `Superseded` (a newer record from the same source exists)
/// is decided structurally during merge-input assembly, not here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FreshnessClass {
    Stale,
    Fresh,
}

impl FreshnessClass {
    pub fn classify(record: Epoch, now: Epoch, stale_after_epochs: u64) -> FreshnessClass {
        if now.get().saturating_sub(record.get()) > stale_after_epochs {
            FreshnessClass::Stale
        } else {
            FreshnessClass::Fresh
        }
    }
}
