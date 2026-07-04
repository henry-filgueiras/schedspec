//! Operator overrides: extraordinary, constrained, and permanently visible.

use crate::belief::BeliefState;
use crate::epoch::Epoch;
use crate::id::{write_str, CanonicalBytes, OperatorId, OverrideId, SubjectId};
use serde::{Deserialize, Serialize};

/// A visible operator intervention on one subject's merged belief. The
/// justification surface is required (PRIMITIVES.md OperatorOverride), and
/// applying one produces an `Overridden` resolution plus a residue scar —
/// an override can never masquerade as organic convergence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorOverride {
    pub operator: OperatorId,
    pub subject: SubjectId,
    pub forced: BeliefState,
    pub reason: String,
    pub epoch: Epoch,
}

impl OperatorOverride {
    pub fn id(&self) -> OverrideId {
        OverrideId::from_bytes(self.digest("resonant/override/v1"))
    }
}

impl CanonicalBytes for OperatorOverride {
    fn write_canonical(&self, out: &mut Vec<u8>) {
        self.operator.write_canonical(out);
        self.subject.write_canonical(out);
        write_str(out, self.forced.as_str());
        write_str(out, &self.reason);
        self.epoch.get().write_canonical(out);
    }
}
