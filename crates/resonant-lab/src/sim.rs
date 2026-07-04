//! A seeded, fully deterministic partition/heal simulation driving the
//! kernel end to end: introduction, witness selection via permutation
//! rank, corroboration under hysteresis, a partition with divergent
//! decisions, and a deterministic reunion with visible residue.
//!
//! There is no `rand` dependency and no clock: every choice derives from
//! one root `u64` through domain-separated SplitMix64, so a report is a
//! pure function of its config — the same accountable determinism the
//! treatise asks of the protocol itself.

use resonant_kernel::belief::BeliefState;
use resonant_kernel::digest::RepairDigest;
use resonant_kernel::epoch::Epoch;
use resonant_kernel::evidence::{
    AssertedState, Claim, Diversity, Observation, ObservationMode, Provenance, Quality, Stance,
    WitnessRecord, WitnessSummary,
};
use resonant_kernel::id::{OperatorId, PeerId, SubjectId, WitnessId, WitnessRecordId};
use resonant_kernel::kernel::{Effect, Input, Kernel};
use resonant_kernel::merge::engine::MergeSide;
use resonant_kernel::merge::{BeliefFragment, OverallOutcome};
use resonant_kernel::operator::OperatorOverride;
use resonant_kernel::policy::PolicyBundle;
use resonant_kernel::rank::RankDomain;
use resonant_kernel::scope::{ScopeAuthority, ScopeId};
use resonant_kernel::transcript::Transcript;
use resonant_kernel::trust::{Confidence, TrustGrade};
use resonant_kernel::util::NonEmpty;
use serde::Serialize;
use std::collections::BTreeMap;

/// SplitMix64: the only randomness source, public-domain construction.
pub struct SplitMix64(u64);

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    pub fn pick(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound.max(1)
    }
}

/// Domain-separated sub-seed: any component of the simulation can be
/// re-derived in isolation from (root, label, parts).
pub fn derive_seed(root: u64, label: &str, parts: &[u64]) -> u64 {
    let mut acc = 0xcbf2_9ce4_8422_2325u64; // FNV offset basis
    for byte in label.as_bytes() {
        acc ^= u64::from(*byte);
        acc = acc.wrapping_mul(0x1000_0000_01b3);
    }
    for part in parts {
        acc ^= *part;
        acc = acc.wrapping_mul(0x1000_0000_01b3);
    }
    SplitMix64::new(root ^ acc).next_u64()
}

#[derive(Debug, Clone, Serialize)]
pub struct SimConfig {
    pub seed: u64,
    /// Peer pool size for witness selection.
    pub nodes: u32,
    /// Number of subjects introduced into the cluster.
    pub subjects: u32,
    /// Apply a visible operator override to the disputed subject at
    /// reunion time.
    pub operator_override: bool,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            seed: 42,
            nodes: 8,
            subjects: 4,
            operator_override: false,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SimReport {
    pub config: SimConfig,
    pub narrative: Vec<String>,
    /// The subject the islands disagreed about during the partition.
    pub victim: String,
    pub victim_state: BeliefState,
    pub unresolved_residue: usize,
    pub handled_residue: usize,
    pub overall: OverallOutcome,
    /// Do both islands hold identical views after applying the reunion?
    pub converged: bool,
    /// Transcript chain heads (hex), island A and B.
    pub transcript_heads: [String; 2],
    pub island_digests: [RepairDigest; 2],
}

fn hex(bytes: [u8; 32]) -> String {
    bytes[..8].iter().map(|b| format!("{b:02x}")).collect()
}

fn scope() -> ScopeId {
    ScopeId::new("cluster")
}

fn subject_name(i: u32) -> SubjectId {
    SubjectId::new(format!("s{i:02}"))
}

/// Run the simulation. Pure: `run(c) == run(c)` bit for bit.
pub fn run(config: &SimConfig) -> SimReport {
    let root = config.seed;
    let policy = PolicyBundle::default();
    let mut narrative = Vec::new();

    let subjects: Vec<SubjectId> = (0..config.subjects).map(subject_name).collect();
    let pool: Vec<PeerId> = (0..config.nodes)
        .map(|i| PeerId::new(format!("n{i:02}")))
        .collect();
    let trust: BTreeMap<SubjectId, TrustGrade> = subjects
        .iter()
        .map(|s| {
            let grade = 70
                + (derive_seed(root, "trust", &[u64::from(s.as_str().len() as u32)])
                    ^ u64::from(s.as_str().as_bytes()[1]))
                    % 20;
            (s.clone(), TrustGrade::new(grade as u8))
        })
        .collect();

    // Phase 1: shared history. Every subject is introduced, witnessed by a
    // rank-selected witness set, and corroborated to accepted under
    // hysteresis.
    let mut kernel = Kernel::new(policy.clone());
    let mut prehistory = Transcript::new();
    let mut summaries: BTreeMap<SubjectId, WitnessSummary> = BTreeMap::new();

    kernel.handle(
        Input::EpochAdvanced {
            scope: scope(),
            epoch: Epoch(1),
        },
        &mut prehistory,
    );
    for subject in &subjects {
        kernel.handle(
            Input::Introduce(Claim {
                subject: subject.clone(),
                asserted: AssertedState::Present,
                scope: scope(),
                provenance: Provenance {
                    introducer: PeerId::new("gateway"),
                },
                epoch: Epoch(1),
                evidence: vec![],
            }),
            &mut prehistory,
        );

        let effects = kernel.handle(
            Input::RequestWitnessSelection {
                domain: RankDomain::WitnessSelection,
                scope: scope(),
                subject: Some(subject.clone()),
                pool: pool.clone(),
                excluded: vec![],
                take: 3,
            },
            &mut prehistory,
        );
        let [Effect::WitnessSetSelected(selection)] = &effects[..] else {
            unreachable!("witness selection always yields a selection effect");
        };

        let mut records: Vec<WitnessRecordId> = Vec::new();
        for peer in &selection.selected {
            let observation = Observation {
                observer: WitnessId::new(peer.as_str()),
                subject: subject.clone(),
                mode: ObservationMode::DirectContact,
                epoch: Epoch(1),
            };
            let record = WitnessRecord {
                witness: WitnessId::new(peer.as_str()),
                subject: subject.clone(),
                about: None,
                stance: Stance::Corroborate,
                observation: observation.id(),
                scope: scope(),
                epoch: Epoch(1),
                trust_context: trust[subject],
            };
            records.push(record.id());
            kernel.handle(Input::WitnessRecordReceived(record), &mut prehistory);
        }
        summaries.insert(
            subject.clone(),
            WitnessSummary {
                count: records.len() as u32,
                quality: Quality::Strong,
                diversity: Diversity::CrossScope,
            },
        );

        // Strengthen: witnessed -> provisional -> accepted, waiting out
        // hysteresis between steps.
        for _ in 0..policy.strengthen_hysteresis_rounds {
            kernel.handle(Input::Tick, &mut prehistory);
        }
        kernel.handle(
            Input::CorroborationAssessed {
                scope: scope(),
                subject: subject.clone(),
                records: NonEmpty::from_vec(records.clone()).expect("witness set is non-empty"),
                confidence: Confidence::Bounded,
            },
            &mut prehistory,
        );
        for _ in 0..policy.strengthen_hysteresis_rounds {
            kernel.handle(Input::Tick, &mut prehistory);
        }
        kernel.handle(
            Input::CorroborationAssessed {
                scope: scope(),
                subject: subject.clone(),
                records: NonEmpty::from_vec(records).expect("witness set is non-empty"),
                confidence: Confidence::Strong,
            },
            &mut prehistory,
        );
    }
    narrative.push(format!(
        "phase 1: {} subjects accepted in scope 'cluster' under {}-round hysteresis ({} transcript events)",
        subjects.len(),
        policy.strengthen_hysteresis_rounds,
        prehistory.len(),
    ));

    // Phase 2: partition. Both islands inherit the shared history, then
    // diverge: island B removes a seeded victim at the same epoch island A
    // keeps it accepted — the canonical same-epoch conflict.
    let mut island_a = kernel.clone();
    let mut island_b = kernel;
    let mut transcript_a = Transcript::new();
    let mut transcript_b = Transcript::new();

    let victim = subjects[derive_seed(root, "victim", &[]) as usize % subjects.len()].clone();
    island_a.handle(
        Input::EpochAdvanced {
            scope: scope(),
            epoch: Epoch(2),
        },
        &mut transcript_a,
    );
    island_b.handle(
        Input::EpochAdvanced {
            scope: scope(),
            epoch: Epoch(2),
        },
        &mut transcript_b,
    );

    // Island B: an opposing witness record, then an organic removal
    // decision (accepted -> removed is a canonical edge).
    for input in [
        Input::WitnessRecordReceived(WitnessRecord {
            witness: WitnessId::new("n-watchdog"),
            subject: victim.clone(),
            about: None,
            stance: Stance::SupportRevocation,
            observation: Observation {
                observer: WitnessId::new("n-watchdog"),
                subject: victim.clone(),
                mode: ObservationMode::Timeout,
                epoch: Epoch(2),
            }
            .id(),
            scope: scope(),
            epoch: Epoch(2),
            trust_context: trust[&victim],
        }),
        Input::Tick,
        Input::RemovalAssessed {
            scope: scope(),
            subject: victim.clone(),
            basis: resonant_kernel::belief::RemovalBasis::PolicyViolation,
        },
    ] {
        island_b.handle(input, &mut transcript_b);
    }
    debug_assert_eq!(
        island_b
            .view(&scope())
            .unwrap()
            .belief(&victim)
            .unwrap()
            .state(),
        BeliefState::Removed
    );

    narrative.push(format!(
        "phase 2: partition; island B revokes {} at epoch 2 while island A keeps it accepted",
        victim
    ));

    // Phase 3: reunion. Sides are read straight from the island views;
    // witness summaries ride along from the shared history.
    let side = |label: &str, island: &Kernel| MergeSide {
        label: label.into(),
        epoch: Epoch(2),
        fragments: subjects
            .iter()
            .map(|s| {
                let state = island.view(&scope()).unwrap().belief(s).unwrap().state();
                let fragment = BeliefFragment {
                    state,
                    epoch: Epoch(2),
                    trust: trust[s],
                    authority: ScopeAuthority::Global,
                    witness: summaries[s],
                };
                (s.clone(), fragment)
            })
            .collect(),
    };
    let side_a = side("island-a", &island_a);
    let side_b = side("island-b", &island_b);

    let operator_override = config.operator_override.then(|| OperatorOverride {
        operator: OperatorId::new("sim-operator"),
        subject: victim.clone(),
        forced: BeliefState::Quarantined,
        reason: "same-epoch removal vs acceptance: quarantine is the least dishonest intervention"
            .into(),
        epoch: Epoch(2),
    });

    // The rendezvous round is agreed as part of the reunion, so both
    // islands mint identical residue ids and genuinely converge.
    let rendezvous_round = resonant_kernel::epoch::Round(1000);
    let reunion_input = |ov: Option<OperatorOverride>| Input::ReunionRequested {
        scope: scope(),
        round: rendezvous_round,
        subjects: subjects.clone(),
        side_a: side_a.clone(),
        side_b: side_b.clone(),
        operator_override: ov,
    };

    let effects_a = island_a.handle(reunion_input(operator_override.clone()), &mut transcript_a);
    let _effects_b = island_b.handle(reunion_input(operator_override), &mut transcript_b);

    let digest_a = RepairDigest::of(island_a.view(&scope()).unwrap());
    let digest_b = RepairDigest::of(island_b.view(&scope()).unwrap());
    let converged = digest_a.content_hash == digest_b.content_hash;

    let victim_state = island_a
        .view(&scope())
        .unwrap()
        .belief(&victim)
        .unwrap()
        .state();
    let unresolved_residue = island_a
        .view(&scope())
        .unwrap()
        .residue()
        .unhandled()
        .count();
    let handled_residue = island_a.view(&scope()).unwrap().residue().len() - unresolved_residue;
    let overall = match &effects_a[..] {
        [Effect::ShareDigest(_)] => {
            // Recover the overall outcome from the transcript.
            transcript_a
                .events()
                .iter()
                .rev()
                .find_map(|e| match &e.event {
                    resonant_kernel::transcript::TranscriptEvent::ReunionCompleted {
                        overall,
                        ..
                    } => Some(*overall),
                    _ => None,
                })
                .expect("reunion completed event exists")
        }
        _ => unreachable!("reunion always shares a digest"),
    };

    narrative.push(format!(
        "phase 3: deterministic reunion -> {} is {}, {} unresolved residue, islands {}",
        victim,
        victim_state,
        unresolved_residue,
        if converged { "converged" } else { "DIVERGED" },
    ));

    SimReport {
        config: config.clone(),
        narrative,
        victim: victim.as_str().to_string(),
        victim_state,
        unresolved_residue,
        handled_residue,
        overall,
        converged,
        transcript_heads: [
            transcript_a.head().map(hex).unwrap_or_default(),
            transcript_b.head().map(hex).unwrap_or_default(),
        ],
        island_digests: [digest_a, digest_b],
    }
}
