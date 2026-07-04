//! Serde model for the scenario corpus in `docs/scenarios/`.
//!
//! The six deterministic-reunion scenarios are the conformance corpus
//! shared with the in-browser Deterministic Reunion Lab. The model is
//! deliberately closed: `deny_unknown_fields` everywhere and closed enums,
//! so any drift between the JSON files, the JS lab, and this crate fails
//! loudly at parse time. (lab.js silently scores unknown scopes as 0 and
//! defaults a missing witness summary; the closed types make both
//! unrepresentable — a deliberate tightening.)

use resonant_kernel::belief::BeliefState;
use resonant_kernel::evidence::{Diversity, Quality};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The three scopes the lab's score table knows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Scope {
    Global,
    RegionalA,
    RegionalB,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WitnessSummary {
    pub count: u32,
    pub quality: Quality,
    pub diversity: Diversity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemberEntry {
    pub status: BeliefState,
    pub epoch: u64,
    pub trust_weight: i64,
    pub scope: Scope,
    pub witness_summary: WitnessSummary,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IslandView {
    pub label: String,
    pub scope: Scope,
    pub local_epoch: u64,
    pub members: BTreeMap<String, MemberEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InitialState {
    pub policy_name: String,
    pub policy_summary: String,
    pub island_a: IslandView,
    pub island_b: IslandView,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IslandPatch {
    #[serde(default)]
    pub local_epoch: Option<u64>,
    #[serde(default)]
    pub members: Option<BTreeMap<String, MemberEntry>>,
    #[serde(default)]
    pub log: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Patches {
    #[serde(default)]
    pub island_a: Option<IslandPatch>,
    #[serde(default)]
    pub island_b: Option<IslandPatch>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub id: String,
    pub label: String,
    pub description: String,
    pub patches: Patches,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatorOverride {
    pub subject_id: String,
    pub status: BeliefState,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Subject {
    pub id: String,
    pub label: String,
    pub role: String,
    pub domain: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub difference_from_quorum_lab: String,
    pub notes: Vec<String>,
    pub allow_operator_override: bool,
    pub operator_override: Option<OperatorOverride>,
    pub expected_merge_tensions: Vec<String>,
    pub subjects: Vec<Subject>,
    pub initial: InitialState,
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IndexEntry {
    pub id: String,
    pub title: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioIndex {
    pub lab: String,
    pub scenarios: Vec<IndexEntry>,
}

/// Default location of the canonical corpus: `docs/scenarios/` in this
/// repository, resolved relative to this crate. Override with the
/// `RESONANT_SCENARIOS_DIR` environment variable for installed binaries.
pub fn default_scenarios_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("RESONANT_SCENARIOS_DIR") {
        return PathBuf::from(dir);
    }
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/scenarios")
}

#[derive(Debug, thiserror::Error)]
pub enum CorpusError {
    #[error("failed to read {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to parse {path}: {source}")]
    Parse {
        path: PathBuf,
        source: serde_json::Error,
    },
}

pub fn load_index(dir: &Path) -> Result<ScenarioIndex, CorpusError> {
    load_json(&dir.join("index.json"))
}

pub fn load_scenario(dir: &Path, path: &str) -> Result<Scenario, CorpusError> {
    load_json(&dir.join(path))
}

/// Load the whole corpus in index order.
pub fn load_corpus(dir: &Path) -> Result<Vec<Scenario>, CorpusError> {
    let index = load_index(dir)?;
    index
        .scenarios
        .iter()
        .map(|entry| load_scenario(dir, &entry.path))
        .collect()
}

fn load_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, CorpusError> {
    let text = std::fs::read_to_string(path).map_err(|source| CorpusError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| CorpusError::Parse {
        path: path.to_path_buf(),
        source,
    })
}
