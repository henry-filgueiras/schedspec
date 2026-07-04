//! Partition Theater: an in-process, deterministic multi-actor harness.
//!
//! Every actor is a full participant — its own `Kernel`, `Transcript`, and
//! `EvidenceBook` — and stories are scripts of protocol beats: joins,
//! witnessing, moderation, partitions, heals, overrides, expectations.
//! There is no networking and no clock; "broadcast" is a routing table
//! that honors the current partition, which makes every story replayable
//! and assertable. This is both the demo rig (`resonant theater run`) and
//! the kernel's end-to-end integration suite.

use resonant_kernel::belief::{BeliefState, QuarantineReason, RemovalBasis};
use resonant_kernel::digest::RepairDigest;
use resonant_kernel::epoch::Epoch;
use resonant_kernel::evidence::{
    AssertedState, Claim, Observation, ObservationMode, Provenance, Stance, WitnessRecord,
};
use resonant_kernel::id::{OperatorId, PeerId, SubjectId, WitnessId};
use resonant_kernel::kernel::{Input, Kernel};
use resonant_kernel::operator::OperatorOverride;
use resonant_kernel::policy::PolicyBundle;
use resonant_kernel::scope::{ScopeAuthority, ScopeId};
use resonant_kernel::transcript::Transcript;
use resonant_kernel::trust::{Confidence, TrustGrade};
use resonant_kernel::util::NonEmpty;
use resonant_kernel::witnessing::{ready_to_advance, rendezvous_round, EvidenceBook};
use std::collections::BTreeMap;

/// One story beat.
#[derive(Debug, Clone)]
pub enum Step {
    /// `who` claims presence in the room, vouched by `vouched_by`.
    Join {
        who: &'static str,
        vouched_by: &'static str,
    },
    /// `by` witnesses `subject` with the given observation mode.
    Witness {
        by: &'static str,
        subject: &'static str,
        mode: ObservationMode,
    },
    /// Moderation: suspend or remove, decided by `by`'s side of the room.
    Mute {
        by: &'static str,
        subject: &'static str,
    },
    Ban {
        by: &'static str,
        subject: &'static str,
    },
    /// Split the room into two groups (actors not listed stay in group A).
    Partition { group_b: &'static [&'static str] },
    /// Reconnect and run deterministic reunion on every actor.
    Heal,
    /// A visible operator override, broadcast to everyone reachable.
    Override {
        by: &'static str,
        subject: &'static str,
        to: BeliefState,
    },
    /// Advance every actor's round clock (lets hysteresis pass).
    Ticks(u64),
    /// Assert `subject`'s state — on one actor, or on all reachable ones.
    Expect {
        on: Option<&'static str>,
        subject: &'static str,
        state: BeliefState,
    },
    /// Assert residue counts across all actors' ledgers.
    ExpectResidue {
        min_unhandled: usize,
        min_handled: usize,
    },
    /// Assert every actor's view digest is content-identical.
    ExpectConverged,
}

pub struct Story {
    pub name: &'static str,
    pub blurb: &'static str,
    /// (actor name, trust grade its witness records carry)
    pub actors: &'static [(&'static str, u8)],
    pub creator: &'static str,
    pub steps: fn() -> Vec<Step>,
}

struct Actor {
    name: String,
    kernel: Kernel,
    transcript: Transcript,
    evidence: EvidenceBook,
    trust: TrustGrade,
    group: u8,
}

pub struct StoryReport {
    pub name: String,
    pub lines: Vec<String>,
    pub failures: Vec<String>,
}

impl StoryReport {
    pub fn passed(&self) -> bool {
        self.failures.is_empty()
    }
}

fn scope() -> ScopeId {
    ScopeId::new("room:theater")
}

pub struct Theater {
    actors: BTreeMap<String, Actor>,
    partitioned: bool,
    epoch: Epoch,
    report: StoryReport,
}

impl Theater {
    fn new(story: &Story) -> Self {
        let mut actors = BTreeMap::new();
        for (name, trust) in story.actors {
            let mut evidence = EvidenceBook::new();
            evidence.mark_trust_root(&scope(), WitnessId::new(story.creator));
            actors.insert(
                name.to_string(),
                Actor {
                    name: name.to_string(),
                    kernel: Kernel::new(PolicyBundle::default()),
                    transcript: Transcript::new(),
                    evidence,
                    trust: TrustGrade::new(*trust),
                    group: 0,
                },
            );
        }
        Self {
            actors,
            partitioned: false,
            epoch: Epoch(1),
            report: StoryReport {
                name: story.name.to_string(),
                lines: Vec::new(),
                failures: Vec::new(),
            },
        }
    }

    fn narrate(&mut self, line: impl Into<String>) {
        self.report.lines.push(line.into());
    }

    fn fail(&mut self, line: impl Into<String>) {
        self.report.failures.push(line.into());
    }

    /// Deliver an input to every actor reachable from `origin` (same
    /// group while partitioned; everyone otherwise), including the origin.
    fn broadcast(&mut self, origin: &str, input: &Input) {
        let origin_group = self.actors.get(origin).map_or(0, |a| a.group);
        let partitioned = self.partitioned;
        for actor in self.actors.values_mut() {
            if partitioned && actor.group != origin_group {
                continue;
            }
            if let Input::Introduce(claim) = input {
                actor.evidence.record_claim(claim);
            }
            if let Input::WitnessRecordReceived(record) = input {
                actor.evidence.record_witness(record.clone());
            }
            actor.kernel.handle(input.clone(), &mut actor.transcript);
        }
    }

    /// The standing policy beat every actor runs after each step:
    /// strengthen belief where the derived summary clears the gate.
    fn settle(&mut self) {
        for actor in self.actors.values_mut() {
            let ready: Vec<(SubjectId, Vec<_>, Confidence)> = {
                let Some(view) = actor.kernel.view(&scope()) else {
                    continue;
                };
                ready_to_advance(&actor.evidence, view)
                    .into_iter()
                    .map(|(subject, records)| {
                        let confidence = match view.belief(&subject).map(|c| c.state()) {
                            Some(BeliefState::Provisional) => Confidence::Strong,
                            _ => Confidence::Bounded,
                        };
                        (subject, records, confidence)
                    })
                    .collect()
            };
            for (subject, records, confidence) in ready {
                let Some(records) = NonEmpty::from_vec(records) else {
                    continue;
                };
                actor.kernel.handle(
                    Input::CorroborationAssessed {
                        scope: scope(),
                        subject,
                        records,
                        confidence,
                    },
                    &mut actor.transcript,
                );
            }
        }
    }

    fn run_step(&mut self, step: &Step) {
        match step {
            Step::Join { who, vouched_by } => {
                self.narrate(format!("• {who} joins, vouched by {vouched_by}"));
                let claim = Claim {
                    subject: SubjectId::new(*who),
                    asserted: AssertedState::Present,
                    scope: scope(),
                    provenance: Provenance {
                        introducer: PeerId::new(*vouched_by),
                    },
                    epoch: self.epoch,
                    evidence: vec![],
                };
                self.broadcast(who, &Input::Introduce(claim));
            }
            Step::Witness { by, subject, mode } => {
                self.narrate(format!("• {by} witnesses {subject} ({mode:?})"));
                let trust = self
                    .actors
                    .get(*by)
                    .map_or(TrustGrade::new(30), |a| a.trust);
                let observation = Observation {
                    observer: WitnessId::new(*by),
                    subject: SubjectId::new(*subject),
                    mode: *mode,
                    epoch: self.epoch,
                };
                let record = WitnessRecord {
                    witness: WitnessId::new(*by),
                    subject: SubjectId::new(*subject),
                    about: None,
                    stance: Stance::Corroborate,
                    observation: observation.id(),
                    mode: *mode,
                    scope: scope(),
                    epoch: self.epoch,
                    trust_context: trust,
                };
                self.broadcast(by, &Input::WitnessRecordReceived(record));
            }
            Step::Mute { by, subject } => {
                self.narrate(format!("• {by}'s side mutes {subject}"));
                self.broadcast(
                    by,
                    &Input::QuarantineAssessed {
                        scope: scope(),
                        subject: SubjectId::new(*subject),
                        reason: QuarantineReason::ConflictPressure,
                    },
                );
            }
            Step::Ban { by, subject } => {
                self.narrate(format!("• {by}'s side bans {subject}"));
                self.broadcast(
                    by,
                    &Input::RemovalAssessed {
                        scope: scope(),
                        subject: SubjectId::new(*subject),
                        basis: RemovalBasis::PolicyViolation,
                    },
                );
            }
            Step::Partition { group_b } => {
                self.narrate(format!(
                    "— partition: {{{}}} split off —",
                    group_b.join(", ")
                ));
                self.partitioned = true;
                for actor in self.actors.values_mut() {
                    actor.group = u8::from(group_b.contains(&actor.name.as_str()));
                }
            }
            Step::Heal => {
                self.heal();
            }
            Step::Override { by, subject, to } => {
                self.narrate(format!("• operator {by} overrides {subject} -> {to}"));
                let op = OperatorOverride {
                    operator: OperatorId::new(*by),
                    subject: SubjectId::new(*subject),
                    forced: *to,
                    reason: format!("operator {by} takes visible responsibility"),
                    epoch: self.epoch,
                };
                self.broadcast(by, &Input::Override(op));
            }
            Step::Ticks(n) => {
                for _ in 0..*n {
                    for actor in self.actors.values_mut() {
                        actor.kernel.handle(Input::Tick, &mut actor.transcript);
                    }
                }
            }
            Step::Expect { on, subject, state } => {
                let subject_id = SubjectId::new(*subject);
                let mut checks: Vec<(String, Option<BeliefState>)> = Vec::new();
                for actor in self.actors.values() {
                    if on.is_some_and(|name| name != actor.name) {
                        continue;
                    }
                    let got = actor
                        .kernel
                        .view(&scope())
                        .and_then(|v| v.belief(&subject_id))
                        .map(|c| c.state());
                    checks.push((actor.name.clone(), got));
                }
                for (actor, got) in checks {
                    if got != Some(*state) {
                        self.fail(format!(
                            "{actor} believes {subject} is {got:?}, expected {state}"
                        ));
                    }
                }
                self.narrate(format!(
                    "✓ {} believe {subject} is {state}",
                    on.map_or("all reachable actors".to_string(), |n| n.to_string())
                ));
            }
            Step::ExpectResidue {
                min_unhandled,
                min_handled,
            } => {
                let mut oks = Vec::new();
                let mut fails = Vec::new();
                for actor in self.actors.values() {
                    let Some(view) = actor.kernel.view(&scope()) else {
                        continue;
                    };
                    let unhandled = view.residue().unhandled().count();
                    let handled = view.residue().len() - unhandled;
                    if unhandled < *min_unhandled || handled < *min_handled {
                        fails.push(format!(
                            "{}: residue unhandled={unhandled} handled={handled}, expected >= {min_unhandled}/{min_handled}",
                            actor.name
                        ));
                    } else {
                        oks.push(format!(
                            "{}: {unhandled} unhandled / {handled} handled",
                            actor.name
                        ));
                    }
                }
                for f in fails {
                    self.fail(f);
                }
                if let Some(sample) = oks.first() {
                    self.narrate(format!("✓ residue visible ({sample})"));
                }
            }
            Step::ExpectConverged => {
                let hashes: Vec<(String, Option<[u8; 32]>)> = self
                    .actors
                    .values()
                    .map(|a| {
                        (
                            a.name.clone(),
                            a.kernel
                                .view(&scope())
                                .map(|v| RepairDigest::of(v).content_hash),
                        )
                    })
                    .collect();
                let first = hashes.first().and_then(|(_, h)| *h);
                if hashes.iter().all(|(_, h)| *h == first) && first.is_some() {
                    self.narrate(format!(
                        "✓ all {} actors converged (digest {})",
                        hashes.len(),
                        first
                            .map(|h| h[..6]
                                .iter()
                                .map(|b| format!("{b:02x}"))
                                .collect::<String>())
                            .unwrap_or_default()
                    ));
                } else {
                    self.fail(format!("views diverged: {hashes:?}"));
                }
            }
        }
        self.settle();
    }

    /// Reconnect and run the same deterministic reunion on every actor:
    /// sides come from one representative view per group, oriented by
    /// digest hash, at a rendezvous round derived from both digests (P18).
    fn heal(&mut self) {
        self.narrate("— heal: islands reconnect —".to_string());
        self.partitioned = false;

        let rep = |theater: &Theater, group: u8| -> Option<String> {
            theater
                .actors
                .values()
                .find(|a| a.group == group)
                .map(|a| a.name.clone())
        };
        let (Some(rep_a), Some(rep_b)) = (rep(self, 0), rep(self, 1)) else {
            // Never partitioned into two groups: nothing to reunite.
            return;
        };

        let side_of = |theater: &Theater, name: &str| {
            let actor = &theater.actors[name];
            let view = actor.kernel.view(&scope()).expect("actor has a room view");
            (
                actor
                    .evidence
                    .merge_side(name, view, ScopeAuthority::Global),
                RepairDigest::of(view).content_hash,
            )
        };
        let (side_x, hash_x) = side_of(self, &rep_a);
        let (side_y, hash_y) = side_of(self, &rep_b);
        // Orientation by digest hash: lower hash is side_a, so every
        // participant computes the identical reunion input.
        let ((side_a, hash_a), (side_b, hash_b)) = if hash_x <= hash_y {
            ((side_x, hash_x), (side_y, hash_y))
        } else {
            ((side_y, hash_y), (side_x, hash_x))
        };

        let rounds: Vec<u64> = self
            .actors
            .values()
            .map(|a| a.kernel.round().get())
            .collect();
        let round = rendezvous_round(&rounds, &hash_a, &hash_b);

        let mut subjects: Vec<SubjectId> = side_a
            .fragments
            .keys()
            .chain(side_b.fragments.keys())
            .cloned()
            .collect();
        subjects.sort();
        subjects.dedup();

        self.narrate(format!(
            "• deterministic reunion at {round} between {} and {}",
            side_a.label, side_b.label
        ));

        let input = Input::ReunionRequested {
            scope: scope(),
            round,
            subjects,
            side_a,
            side_b,
            operator_override: None,
        };
        for actor in self.actors.values_mut() {
            actor.group = 0;
            actor.kernel.handle(input.clone(), &mut actor.transcript);
        }
    }
}

/// Run a story to completion.
pub fn run_story(story: &Story) -> StoryReport {
    let mut theater = Theater::new(story);
    theater.narrate(format!("=== {} ===", story.name));
    theater.narrate(story.blurb.to_string());
    for actor in theater.actors.values_mut() {
        actor.kernel.handle(
            Input::EpochAdvanced {
                scope: scope(),
                epoch: Epoch(1),
            },
            &mut actor.transcript,
        );
    }
    for step in (story.steps)() {
        theater.run_step(&step);
    }
    let verdict = if theater.report.passed() {
        "PASSED"
    } else {
        "FAILED"
    };
    theater.narrate(format!("=== {} {} ===", story.name, verdict));
    theater.report
}

/// The story corpus.
pub fn stories() -> Vec<Story> {
    vec![
        Story {
            name: "sockpuppet-vouch",
            blurb: "A loud clique of puppets vouched through one member cannot buy standing; \
                    two independently-vouched witnesses can.",
            actors: &[
                ("alice", 90),
                ("bob", 70),
                ("carol", 70),
                ("mallory", 60),
                ("eve", 40),
                ("p1", 30),
                ("p2", 30),
                ("p3", 30),
                ("p4", 30),
            ],
            creator: "alice",
            steps: || {
                use ObservationMode::*;
                vec![
                    Step::Join {
                        who: "bob",
                        vouched_by: "alice",
                    },
                    Step::Join {
                        who: "carol",
                        vouched_by: "alice",
                    },
                    Step::Join {
                        who: "mallory",
                        vouched_by: "alice",
                    },
                    Step::Join {
                        who: "eve",
                        vouched_by: "mallory",
                    },
                    Step::Join {
                        who: "p1",
                        vouched_by: "mallory",
                    },
                    Step::Join {
                        who: "p2",
                        vouched_by: "p1",
                    },
                    Step::Join {
                        who: "p3",
                        vouched_by: "p1",
                    },
                    Step::Join {
                        who: "p4",
                        vouched_by: "p2",
                    },
                    // Honest corroboration for bob: two independent lineages,
                    // direct contact.
                    Step::Witness {
                        by: "alice",
                        subject: "bob",
                        mode: DirectContact,
                    },
                    Step::Witness {
                        by: "carol",
                        subject: "bob",
                        mode: ChallengeResponse,
                    },
                    // The puppet cluster shouts for eve with weak evidence.
                    Step::Witness {
                        by: "p1",
                        subject: "eve",
                        mode: Timeout,
                    },
                    Step::Witness {
                        by: "p2",
                        subject: "eve",
                        mode: TopologyEvidence,
                    },
                    Step::Witness {
                        by: "p3",
                        subject: "eve",
                        mode: Timeout,
                    },
                    Step::Witness {
                        by: "p4",
                        subject: "eve",
                        mode: TopologyEvidence,
                    },
                    Step::Ticks(3),
                    Step::Expect {
                        on: None,
                        subject: "bob",
                        state: BeliefState::Provisional,
                    },
                    Step::Ticks(3),
                    Step::Expect {
                        on: None,
                        subject: "bob",
                        state: BeliefState::Accepted,
                    },
                    // Four loud puppets bought nothing.
                    Step::Expect {
                        on: None,
                        subject: "eve",
                        state: BeliefState::Witnessed,
                    },
                ]
            },
        },
        Story {
            name: "split-brain-ban",
            blurb: "During a partition one side bans dave while the other keeps him accepted. \
                    Reunion refuses to counterfeit certainty: dave is disputed, the conflict \
                    survives as residue, and every island converges on the same honest view.",
            actors: &[("alice", 90), ("bob", 75), ("carol", 75), ("dave", 70)],
            creator: "alice",
            steps: split_brain_steps,
        },
        Story {
            name: "admin-override",
            blurb: "Same split-brain ban, but the room creator resolves the dispute with a \
                    visible override: dave is quarantined, and the scar is marked handled — \
                    never erased.",
            actors: &[("alice", 90), ("bob", 75), ("carol", 75), ("dave", 70)],
            creator: "alice",
            steps: || {
                let mut steps = split_brain_steps();
                steps.extend([
                    Step::Override {
                        by: "alice",
                        subject: "dave",
                        to: BeliefState::Quarantined,
                    },
                    Step::Expect {
                        on: None,
                        subject: "dave",
                        state: BeliefState::Quarantined,
                    },
                    Step::ExpectResidue {
                        min_unhandled: 0,
                        min_handled: 1,
                    },
                    Step::ExpectConverged,
                ]);
                steps
            },
        },
    ]
}

fn split_brain_steps() -> Vec<Step> {
    use ObservationMode::*;
    vec![
        Step::Join {
            who: "bob",
            vouched_by: "alice",
        },
        Step::Join {
            who: "carol",
            vouched_by: "alice",
        },
        Step::Join {
            who: "dave",
            vouched_by: "alice",
        },
        // Everyone gets honest, independent corroboration.
        Step::Witness {
            by: "alice",
            subject: "bob",
            mode: DirectContact,
        },
        Step::Witness {
            by: "carol",
            subject: "bob",
            mode: DirectContact,
        },
        Step::Witness {
            by: "alice",
            subject: "carol",
            mode: DirectContact,
        },
        Step::Witness {
            by: "bob",
            subject: "carol",
            mode: DirectContact,
        },
        Step::Witness {
            by: "bob",
            subject: "dave",
            mode: DirectContact,
        },
        Step::Witness {
            by: "carol",
            subject: "dave",
            mode: ChallengeResponse,
        },
        Step::Ticks(3),
        Step::Ticks(3),
        Step::Expect {
            on: None,
            subject: "dave",
            state: BeliefState::Accepted,
        },
        // The split: alice+bob on one side, carol+dave on the other.
        Step::Partition {
            group_b: &["carol", "dave"],
        },
        Step::Ban {
            by: "alice",
            subject: "dave",
        },
        Step::Expect {
            on: Some("alice"),
            subject: "dave",
            state: BeliefState::Removed,
        },
        Step::Expect {
            on: Some("carol"),
            subject: "dave",
            state: BeliefState::Accepted,
        },
        Step::Heal,
        Step::Expect {
            on: None,
            subject: "dave",
            state: BeliefState::Disputed,
        },
        Step::ExpectResidue {
            min_unhandled: 1,
            min_handled: 0,
        },
        Step::ExpectConverged,
    ]
}
