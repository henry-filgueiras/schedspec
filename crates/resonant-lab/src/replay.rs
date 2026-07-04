//! Divergence replay: `materializeCurrentState` from lab.js, ported.
//!
//! Patches replace whole member entries (never partial fields) and may
//! bump an island's local epoch; `log` strings are operator narration and
//! ride along for display.

use crate::scenario::{IslandView, Scenario};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ReplayLog {
    pub island: &'static str,
    pub event_label: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CurrentState {
    pub island_a: IslandView,
    pub island_b: IslandView,
    pub logs: Vec<ReplayLog>,
    /// How many divergence events were replayed to produce this state.
    pub steps_replayed: usize,
}

/// Replay the first `steps` divergence events over the initial islands.
/// `steps` is clamped to the scenario's event count.
pub fn materialize(scenario: &Scenario, steps: usize) -> CurrentState {
    let mut island_a = scenario.initial.island_a.clone();
    let mut island_b = scenario.initial.island_b.clone();
    let mut logs = Vec::new();
    let steps = steps.min(scenario.events.len());

    for event in &scenario.events[..steps] {
        for (island_name, patch, island) in [
            ("island_a", &event.patches.island_a, &mut island_a),
            ("island_b", &event.patches.island_b, &mut island_b),
        ] {
            let Some(patch) = patch else { continue };
            if let Some(epoch) = patch.local_epoch {
                island.local_epoch = epoch;
            }
            if let Some(members) = &patch.members {
                for (id, entry) in members {
                    island.members.insert(id.clone(), entry.clone());
                }
            }
            if let Some(log) = &patch.log {
                logs.push(ReplayLog {
                    island: island_name,
                    event_label: event.label.clone(),
                    detail: log.clone(),
                });
            }
        }
    }

    CurrentState {
        island_a,
        island_b,
        logs,
        steps_replayed: steps,
    }
}
