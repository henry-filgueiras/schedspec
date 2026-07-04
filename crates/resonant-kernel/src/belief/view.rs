//! A scoped membership view: beliefs plus their residue, inseparably.

use crate::belief::cell::BeliefCell;
use crate::epoch::{Epoch, Round};
use crate::id::SubjectId;
use crate::residue::ResidueLedger;
use crate::scope::ScopeId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// One scope's current belief about its subjects. A view is always scoped,
/// never global (PRIMITIVES.md MembershipView), and it carries its
/// `ResidueLedger` as a field: a view without its disagreement cannot be
/// constructed or serialized.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MembershipView {
    scope: ScopeId,
    epoch: Epoch,
    beliefs: BTreeMap<SubjectId, BeliefCell>,
    residue: ResidueLedger,
}

impl MembershipView {
    pub fn new(scope: ScopeId, epoch: Epoch) -> Self {
        Self {
            scope,
            epoch,
            beliefs: BTreeMap::new(),
            residue: ResidueLedger::new(),
        }
    }

    pub fn scope(&self) -> &ScopeId {
        &self.scope
    }

    pub fn epoch(&self) -> Epoch {
        self.epoch
    }

    pub fn advance_epoch(&mut self, epoch: Epoch) {
        self.epoch = self.epoch.max(epoch);
    }

    pub fn belief(&self, subject: &SubjectId) -> Option<&BeliefCell> {
        self.beliefs.get(subject)
    }

    /// Get or create the cell for a subject. The returned cell can only be
    /// mutated through `BeliefCell::apply`, so handing out `&mut` does not
    /// bypass the state machine.
    pub fn cell_mut(&mut self, subject: SubjectId, at: (Epoch, Round)) -> &mut BeliefCell {
        self.beliefs
            .entry(subject)
            .or_insert_with(|| BeliefCell::new(at))
    }

    pub fn subjects(&self) -> impl Iterator<Item = (&SubjectId, &BeliefCell)> {
        self.beliefs.iter()
    }

    pub fn len(&self) -> usize {
        self.beliefs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.beliefs.is_empty()
    }

    pub fn residue(&self) -> &ResidueLedger {
        &self.residue
    }

    pub(crate) fn residue_mut(&mut self) -> &mut ResidueLedger {
        &mut self.residue
    }
}
