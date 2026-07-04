//! The node: one tokio event loop driving a libp2p swarm below and the
//! sans-IO resonant kernel above. Every stimulus becomes a kernel `Input`
//! (logged as JSONL for audit and replay verification), every kernel
//! `Effect` becomes a publish or a request — the kernel never crosses an
//! `.await`.

use crate::wire::{self, GossipMsg, Moderation, RrRequest, RrResponse};
use libp2p::allow_block_list::{self, BlockedPeers};
use libp2p::futures::StreamExt;
use libp2p::gossipsub::{self, IdentTopic, MessageAuthenticity, ValidationMode};
use libp2p::identity::Keypair;
use libp2p::request_response::{self, ProtocolSupport};
use libp2p::swarm::{NetworkBehaviour, SwarmEvent};
use libp2p::{identify, ping, Multiaddr, PeerId, StreamProtocol, Swarm};
use resonant_kernel::belief::{BeliefState, QuarantineReason, RemovalBasis};
use resonant_kernel::digest::RepairDigest;
use resonant_kernel::epoch::Epoch;
use resonant_kernel::evidence::{
    AssertedState, Claim, Observation, ObservationMode, Provenance, Stance, WitnessRecord,
};
use resonant_kernel::id::{OperatorId, SubjectId, WitnessId};
use resonant_kernel::kernel::{Effect, Input, Kernel};
use resonant_kernel::operator::OperatorOverride;
use resonant_kernel::policy::PolicyBundle;
use resonant_kernel::scope::{ScopeAuthority, ScopeId};
use resonant_kernel::transcript::Transcript;
use resonant_kernel::trust::{Confidence, TrustGrade};
use resonant_kernel::util::NonEmpty;
use resonant_kernel::witnessing::{ready_to_advance, rendezvous_round, EvidenceBook};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write as _;
use std::time::Duration;

#[derive(NetworkBehaviour)]
pub struct Behaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub identify: identify::Behaviour,
    pub ping: ping::Behaviour,
    pub rr: request_response::json::Behaviour<RrRequest, RrResponse>,
    pub blocklist: allow_block_list::Behaviour<BlockedPeers>,
}

/// How an application skin interprets the shared machinery. The standing
/// semantics, evidence bookkeeping, and deterministic reunion are
/// identical across skins; the profile only decides what a *subject* is.
#[derive(Debug, Clone, Copy)]
pub struct AppProfile {
    pub kind: &'static str,
    /// Chat: subjects must be addressable peer identities. Federation
    /// blocklists: subjects are arbitrary handles (the things being
    /// blocked), while the witnesses remain peers.
    pub subjects_are_peers: bool,
}

impl AppProfile {
    pub fn chat() -> Self {
        Self {
            kind: "chat",
            subjects_are_peers: true,
        }
    }

    pub fn federation() -> Self {
        Self {
            kind: "federation",
            subjects_are_peers: false,
        }
    }
}

/// One row of the standing roster, for display layers.
pub struct RosterRow {
    pub subject: String,
    pub state: BeliefState,
    pub summary: resonant_kernel::evidence::WitnessSummary,
}

/// One live residue entry, for display layers.
pub struct ResidueRow {
    pub subject: String,
    pub handled_by_override: bool,
    pub detail: String,
}

pub struct NodeConfig {
    pub keypair: Keypair,
    pub profile: AppProfile,
    pub room: String,
    pub nickname: Option<String>,
    /// The room creator's peer id; `None` means "I am the creator".
    pub creator: Option<PeerId>,
    /// Who vouches my join claim (defaults to the creator).
    pub voucher: Option<PeerId>,
    pub listen: Vec<Multiaddr>,
    pub dial: Vec<Multiaddr>,
    pub input_log: Option<std::path::PathBuf>,
    /// Print chat/status lines (off in tests).
    pub interactive: bool,
}

struct Presence {
    connected: bool,
    last_contact_round: u64,
    last_observation_round: u64,
}

pub struct Node {
    pub swarm: Swarm<Behaviour>,
    kernel: Kernel,
    pub transcript: Transcript,
    evidence: EvidenceBook,
    profile: AppProfile,
    scope: ScopeId,
    room: String,
    me: PeerId,
    creator: PeerId,
    voucher: PeerId,
    nickname: BTreeMap<PeerId, String>,
    address_book: BTreeMap<PeerId, BTreeSet<Multiaddr>>,
    presence: BTreeMap<PeerId, Presence>,
    blocked: BTreeSet<PeerId>,
    /// Peers we owe a reconnect after a heal; redialed on ticks until a
    /// connection lands (unblocking never reconnects by itself).
    heal_pending: BTreeSet<PeerId>,
    round: u64,
    /// Reunions already performed, keyed on the sorted digest-hash pair.
    reunions_done: BTreeSet<([u8; 32], [u8; 32])>,
    /// Reunion negotiations awaiting a FullView response, keyed by peer.
    pending_fetch: BTreeMap<PeerId, [u8; 32]>,
    /// Agreed coordinates awaiting an Ack, keyed by peer.
    pending_ack: BTreeMap<PeerId, PendingReunion>,
    /// Claims this node introduced, re-shared on the refresh beat so a
    /// lossy mesh or late joiner still converges.
    my_claims: Vec<Claim>,
    input_log: Option<std::fs::File>,
    interactive: bool,
    /// Lines produced for the user this poll (drained by the runner).
    pub output: Vec<String>,
    /// Addresses this node is actually listening on.
    pub listen_addrs: Vec<Multiaddr>,
}

struct PendingReunion {
    round: u64,
    side_a: resonant_kernel::merge::engine::MergeSide,
    side_b: resonant_kernel::merge::engine::MergeSide,
    digest_lo: [u8; 32],
    digest_hi: [u8; 32],
}

const PRESENCE_DEBOUNCE_ROUNDS: u64 = 5;
const CLAIM_REFRESH_ROUNDS: u64 = 15;
const DIGEST_SHARE_ROUNDS: u64 = 10;

impl Node {
    pub fn new(config: NodeConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let me = config.keypair.public().to_peer_id();
        let creator = config.creator.unwrap_or(me);
        let voucher = config.voucher.unwrap_or(creator);
        let room = config.room.clone();
        let scope = ScopeId::new(format!("room:{room}"));

        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(config.keypair)
            .with_tokio()
            .with_tcp(
                libp2p::tcp::Config::default(),
                libp2p::noise::Config::new,
                libp2p::yamux::Config::default,
            )?
            .with_behaviour(|key| Behaviour {
                gossipsub: gossipsub::Behaviour::new(
                    MessageAuthenticity::Signed(key.clone()),
                    gossipsub::ConfigBuilder::default()
                        .validation_mode(ValidationMode::Strict)
                        .build()
                        .expect("valid gossipsub config"),
                )
                .expect("valid gossipsub behaviour"),
                identify: identify::Behaviour::new(identify::Config::new(
                    "/resonant/1.0.0".into(),
                    key.public(),
                )),
                ping: ping::Behaviour::new(
                    ping::Config::new().with_interval(Duration::from_secs(5)),
                ),
                rr: request_response::json::Behaviour::new(
                    [(StreamProtocol::new("/resonant/rr/1"), ProtocolSupport::Full)],
                    request_response::Config::default(),
                ),
                blocklist: allow_block_list::Behaviour::default(),
            })?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(120)))
            .build();

        for topic in ["chat", "claims", "digests"] {
            swarm
                .behaviour_mut()
                .gossipsub
                .subscribe(&IdentTopic::new(format!("room:{room}/{topic}")))?;
        }
        for addr in config.listen {
            swarm.listen_on(addr)?;
        }
        for addr in config.dial {
            swarm.dial(addr)?;
        }

        let mut evidence = EvidenceBook::new();
        evidence.mark_trust_root(&scope, WitnessId::new(creator.to_base58()));

        let input_log = config
            .input_log
            .as_ref()
            .map(std::fs::File::create)
            .transpose()?;

        let mut nickname = BTreeMap::new();
        if let Some(nick) = config.nickname {
            nickname.insert(me, nick);
        }

        let mut node = Self {
            swarm,
            kernel: Kernel::new(PolicyBundle::default()),
            transcript: Transcript::new(),
            evidence,
            profile: config.profile,
            scope,
            room,
            me,
            creator,
            voucher,
            nickname,
            address_book: BTreeMap::new(),
            presence: BTreeMap::new(),
            blocked: BTreeSet::new(),
            heal_pending: BTreeSet::new(),
            round: 0,
            reunions_done: BTreeSet::new(),
            pending_fetch: BTreeMap::new(),
            pending_ack: BTreeMap::new(),
            my_claims: Vec::new(),
            input_log,
            interactive: config.interactive,
            output: Vec::new(),
            listen_addrs: Vec::new(),
        };

        node.feed(Input::EpochAdvanced {
            scope: node.scope.clone(),
            epoch: Epoch(1),
        });
        node.publish_join_claim();
        Ok(node)
    }

    pub fn peer_id(&self) -> PeerId {
        self.me
    }

    pub fn scope(&self) -> &ScopeId {
        &self.scope
    }

    pub fn view_digest(&self) -> Option<RepairDigest> {
        self.kernel.view(&self.scope).map(RepairDigest::of)
    }

    pub fn belief(&self, peer: &PeerId) -> Option<BeliefState> {
        self.belief_of(&peer.to_base58())
    }

    /// Standing of an arbitrary subject id string.
    pub fn belief_of(&self, subject: &str) -> Option<BeliefState> {
        self.kernel
            .view(&self.scope)
            .and_then(|v| v.belief(&SubjectId::new(subject)))
            .map(|c| c.state())
    }

    pub fn round(&self) -> u64 {
        self.round
    }

    pub fn is_partitioned(&self) -> bool {
        !self.blocked.is_empty()
    }

    /// Register a display name for a peer (used by embedding UIs).
    pub fn set_nickname(&mut self, peer: PeerId, nick: String) {
        self.nickname.insert(peer, nick);
    }

    /// Human-readable name for a subject id string.
    pub fn display_name(&self, subject: &str) -> String {
        self.display(subject)
    }

    /// The standing roster, for display layers.
    pub fn roster(&self) -> Vec<RosterRow> {
        let Some(view) = self.kernel.view(&self.scope) else {
            return Vec::new();
        };
        view.subjects()
            .map(|(subject, cell)| RosterRow {
                subject: subject.as_str().to_string(),
                state: cell.state(),
                summary: self.evidence.summarize(&self.scope, subject),
            })
            .collect()
    }

    /// Live residue entries, for display layers.
    pub fn residues(&self) -> Vec<ResidueRow> {
        let Some(view) = self.kernel.view(&self.scope) else {
            return Vec::new();
        };
        view.residue()
            .iter()
            .map(|r| ResidueRow {
                subject: r.key().subject.as_str().to_string(),
                handled_by_override: r.handled_by().is_some(),
                detail: r.tension().detail.clone(),
            })
            .collect()
    }

    fn say(&mut self, line: impl Into<String>) {
        let line = line.into();
        if self.interactive {
            println!("{line}");
        }
        self.output.push(line);
    }

    fn display(&self, peer_str: &str) -> String {
        if let Ok(peer) = peer_str.parse::<PeerId>() {
            if let Some(nick) = self.nickname.get(&peer) {
                return nick.clone();
            }
        }
        let tail: String = peer_str
            .chars()
            .rev()
            .take(6)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        format!("…{tail}")
    }

    /// Feed one input to the kernel: log it, handle it, dispatch effects.
    fn feed(&mut self, input: Input) {
        if let Some(log) = &mut self.input_log {
            let line = serde_json::to_string(&input).expect("inputs serialize");
            let _ = writeln!(log, "{line}");
        }
        let effects = self.kernel.handle(input, &mut self.transcript);
        for effect in effects {
            self.dispatch(effect);
        }
    }

    fn dispatch(&mut self, effect: Effect) {
        match effect {
            Effect::ShareDigest(digest) => {
                self.publish("digests", GossipMsg::Digest(digest));
            }
            Effect::FetchDetail { .. } | Effect::HoldForRepair { .. } => {
                // Divergence detected by digest comparison. The reunion
                // negotiation is driven where the digest arrived, because
                // the effect does not carry the sender.
            }
            Effect::WitnessSetSelected(_) => {}
        }
    }

    fn publish(&mut self, topic: &str, msg: GossipMsg) {
        let topic = IdentTopic::new(format!("room:{}/{topic}", self.room));
        let data = wire::encode(msg);
        // Duplicate publishes within the gossip cache window are fine to drop.
        let _ = self.swarm.behaviour_mut().gossipsub.publish(topic, data);
    }

    fn publish_join_claim(&mut self) {
        self.introduce_subject(
            self.me.to_base58(),
            AssertedState::Present,
            self.voucher.to_base58(),
        );
    }

    /// Introduce an arbitrary subject into the scope, vouched by
    /// `introducer` (an id string, usually a peer). The chat skin uses
    /// this for self-joins; the federation skin uses it to propose
    /// blocked handles (`AssertedState::Compromised`).
    pub fn introduce_subject(
        &mut self,
        subject: String,
        asserted: AssertedState,
        introducer: String,
    ) {
        let claim = Claim {
            subject: SubjectId::new(subject),
            asserted,
            scope: self.scope.clone(),
            provenance: Provenance {
                introducer: resonant_kernel::id::PeerId::new(introducer),
            },
            epoch: Epoch(1),
            evidence: vec![],
        };
        // Apply locally, then share; remember it for refresh re-shares.
        self.evidence.record_claim(&claim);
        self.feed(Input::Introduce(claim.clone()));
        if !self.my_claims.contains(&claim) {
            self.my_claims.push(claim.clone());
        }
        self.publish("claims", GossipMsg::Claim(claim));
    }

    /// My witness records carry the trust my own standing has earned.
    fn my_trust(&self) -> TrustGrade {
        if self.me == self.creator {
            return TrustGrade::new(90);
        }
        match self.belief(&self.me) {
            Some(BeliefState::Accepted) => TrustGrade::new(70),
            Some(BeliefState::Provisional) => TrustGrade::new(50),
            _ => TrustGrade::new(30),
        }
    }

    fn witness(&mut self, subject: PeerId, mode: ObservationMode) {
        self.witness_subject(subject.to_base58(), mode, Stance::Corroborate);
    }

    /// Publish a witness record about any subject in the scope. The
    /// federation skin uses this for `/confirm` and `/dispute` on blocked
    /// handles.
    pub fn witness_subject(&mut self, subject: String, mode: ObservationMode, stance: Stance) {
        let observation = Observation {
            observer: WitnessId::new(self.me.to_base58()),
            subject: SubjectId::new(subject.clone()),
            mode,
            epoch: Epoch(1),
        };
        let record = WitnessRecord {
            witness: WitnessId::new(self.me.to_base58()),
            subject: SubjectId::new(subject),
            about: None,
            stance,
            observation: observation.id(),
            mode,
            scope: self.scope.clone(),
            epoch: Epoch(1),
            trust_context: self.my_trust(),
        };
        self.evidence.record_witness(record.clone());
        self.feed(Input::WitnessRecordReceived(record.clone()));
        self.publish("claims", GossipMsg::Witness(record));
    }

    /// Is `peer` allowed to moderate, in my view? Creator always; accepted
    /// members otherwise.
    fn may_moderate(&self, peer: &PeerId) -> bool {
        *peer == self.creator || self.belief(peer) == Some(BeliefState::Accepted)
    }

    // ------------------------------------------------------------------
    // Inbound gossip
    // ------------------------------------------------------------------

    fn on_gossip(&mut self, source: Option<PeerId>, data: &[u8]) {
        let msg = match wire::decode(data) {
            Ok(msg) => msg,
            Err(err) => {
                self.say(format!("[wire] {err}"));
                return;
            }
        };
        match msg {
            GossipMsg::Chat { text } => {
                let who = source.map_or("???".into(), |p| self.display(&p.to_base58()));
                // Muted or banned speakers are shown as such, not hidden:
                // moderation is visible standing, not silent filtering.
                let standing = source
                    .and_then(|p| self.belief(&p))
                    .map(wire::state_glyph)
                    .unwrap_or(" ");
                self.say(format!("{standing} {who}: {text}"));
            }
            GossipMsg::Claim(claim) => {
                if claim.scope != self.scope {
                    return;
                }
                // Chat: only addressable identities may become subjects.
                // Federation skins accept arbitrary handles as subjects.
                if self.profile.subjects_are_peers
                    && claim.subject.as_str().parse::<PeerId>().is_err()
                {
                    self.say("[claims] rejected claim for non-peer subject".to_string());
                    return;
                }
                self.evidence.record_claim(&claim);
                let subject = claim.subject.clone();
                self.feed(Input::Introduce(claim));
                // Auto-witness: if I can see this peer alive, say so.
                if let Ok(peer) = subject.as_str().parse::<PeerId>() {
                    let alive = self.presence.get(&peer).is_some_and(|p| p.connected);
                    if alive && peer != self.me {
                        self.witness(peer, ObservationMode::DirectContact);
                    }
                }
            }
            GossipMsg::Witness(record) => {
                if record.scope != self.scope {
                    return;
                }
                self.evidence.record_witness(record.clone());
                self.feed(Input::WitnessRecordReceived(record));
            }
            GossipMsg::Digest(digest) => {
                if digest.scope != self.scope {
                    return;
                }
                let Some(sender) = source else { return };
                self.on_remote_digest(sender, digest);
            }
            GossipMsg::Moderation {
                subject,
                action,
                reason,
            } => {
                let Some(sender) = source else { return };
                if !self.may_moderate(&sender) {
                    self.say(format!(
                        "[mod] ignored {action:?} from {} (not standing to moderate)",
                        self.display(&sender.to_base58())
                    ));
                    return;
                }
                if self.profile.subjects_are_peers && subject.parse::<PeerId>().is_err() {
                    return;
                }
                let subject_id = SubjectId::new(subject.clone());
                let who = self.display(&subject);
                match action {
                    Moderation::Mute => {
                        self.say(format!("[mod] {who} muted: {reason}"));
                        self.feed(Input::QuarantineAssessed {
                            scope: self.scope.clone(),
                            subject: subject_id,
                            reason: QuarantineReason::ConflictPressure,
                        });
                    }
                    Moderation::Ban => {
                        self.say(format!("[mod] {who} banned: {reason}"));
                        self.feed(Input::RemovalAssessed {
                            scope: self.scope.clone(),
                            subject: subject_id,
                            basis: RemovalBasis::PolicyViolation,
                        });
                    }
                }
            }
            GossipMsg::Override(op) => {
                let Some(sender) = source else { return };
                if sender != self.creator {
                    self.say("[mod] ignored override from non-creator".to_string());
                    return;
                }
                let who = self.display(op.subject.as_str());
                self.say(format!(
                    "[mod] OPERATOR OVERRIDE: {who} -> {} ({})",
                    op.forced, op.reason
                ));
                self.feed(Input::Override(op));
            }
        }
    }

    // ------------------------------------------------------------------
    // Reunion negotiation (P18 on the wire)
    // ------------------------------------------------------------------

    fn on_remote_digest(&mut self, sender: PeerId, remote: RepairDigest) {
        let Some(local) = self.view_digest() else {
            return;
        };
        if local.content_hash == remote.content_hash {
            return;
        }
        let pair = hash_pair(local.content_hash, remote.content_hash);
        if self.reunions_done.contains(&pair) {
            return;
        }
        // Feed the kernel for the transcript/effect record.
        self.feed(Input::DigestReceived(remote.clone()));
        // Reunion is a repair surface, not a ramp-up mechanism: negotiate
        // only on *material* divergence — a semantic class conflict on a
        // subject, or live unresolved disagreement on either side. Plain
        // strengthening drift (witnessed vs provisional vs accepted)
        // converges through ordinary evidence gossip.
        if !material_divergence(&local, &remote) {
            return;
        }
        if self.pending_fetch.contains_key(&sender) || self.pending_ack.contains_key(&sender) {
            return;
        }
        self.pending_fetch.insert(sender, remote.content_hash);
        self.swarm.behaviour_mut().rr.send_request(
            &sender,
            RrRequest::FullView {
                scope: self.scope.clone(),
            },
        );
    }

    fn my_side(&self) -> Option<(resonant_kernel::merge::engine::MergeSide, RepairDigest)> {
        let view = self.kernel.view(&self.scope)?;
        let side = self
            .evidence
            .merge_side(self.me.to_base58(), view, ScopeAuthority::Global);
        Some((side, RepairDigest::of(view)))
    }

    fn on_rr_request(
        &mut self,
        _peer: PeerId,
        request: RrRequest,
        channel: request_response::ResponseChannel<RrResponse>,
    ) {
        let response = match request {
            RrRequest::FullView { scope } if scope == self.scope => match self.my_side() {
                Some((side, digest)) => {
                    RrResponse::FullView(Box::new(crate::wire::FullViewResponse {
                        side,
                        digest,
                        round: self.round,
                    }))
                }
                None => RrResponse::Unknown,
            },
            RrRequest::FullView { .. } => RrResponse::Unknown,
            RrRequest::ReunionAck {
                scope,
                round,
                digest_lo,
                digest_hi,
                requester_side,
                requester_digest,
            } => {
                let my_digest = self.view_digest().map(|d| d.content_hash);
                let agree = scope == self.scope
                    && my_digest.is_some_and(|h| h == digest_lo || h == digest_hi)
                    && (requester_digest == digest_lo || requester_digest == digest_hi)
                    && self.expected_round(digest_lo, digest_hi, round);
                if agree {
                    // Apply the identical reunion on this side too, so both
                    // parties converge in one exchange.
                    if let Some((my_side, _)) = self.my_side() {
                        let (side_a, side_b) = if my_digest == Some(digest_lo) {
                            (my_side, requester_side)
                        } else {
                            (requester_side, my_side)
                        };
                        self.perform_reunion(round, side_a, side_b, digest_lo, digest_hi);
                    }
                }
                RrResponse::Ack { agree }
            }
        };
        let _ = self
            .swarm
            .behaviour_mut()
            .rr
            .send_response(channel, response);
    }

    /// Apply an agreed reunion (idempotent per digest pair).
    fn perform_reunion(
        &mut self,
        round: u64,
        side_a: resonant_kernel::merge::engine::MergeSide,
        side_b: resonant_kernel::merge::engine::MergeSide,
        digest_lo: [u8; 32],
        digest_hi: [u8; 32],
    ) {
        let pair = hash_pair(digest_lo, digest_hi);
        if !self.reunions_done.insert(pair) {
            return;
        }
        let mut subjects: Vec<SubjectId> = side_a
            .fragments
            .keys()
            .chain(side_b.fragments.keys())
            .cloned()
            .collect();
        subjects.sort();
        subjects.dedup();
        self.say(format!(
            "[reunion] deterministic reunion at r{round} ({} subjects)",
            subjects.len()
        ));
        self.feed(Input::ReunionRequested {
            scope: self.scope.clone(),
            round: resonant_kernel::epoch::Round(round),
            subjects,
            side_a,
            side_b,
            operator_override: None,
        });
        self.report_residue();
    }

    /// An acked round is acceptable if it is at least what we would derive
    /// ourselves (the peer may have advertised a higher local round).
    fn expected_round(&self, lo: [u8; 32], hi: [u8; 32], round: u64) -> bool {
        let floor = rendezvous_round(&[self.round], &lo, &hi);
        round >= floor.get().saturating_sub(64) && round <= floor.get() + 64
    }

    fn on_rr_response(&mut self, peer: PeerId, response: RrResponse) {
        match response {
            RrResponse::FullView(full) => {
                let crate::wire::FullViewResponse {
                    side: their_side,
                    digest: their_digest,
                    round: their_round,
                } = *full;
                if self.pending_fetch.remove(&peer).is_none() {
                    return;
                }
                let Some((my_side, my_digest)) = self.my_side() else {
                    return;
                };
                if my_digest.content_hash == their_digest.content_hash {
                    return;
                }
                // Deterministic orientation: lower digest hash is side A.
                let (side_a, side_b, lo, hi) =
                    if my_digest.content_hash <= their_digest.content_hash {
                        (
                            my_side,
                            their_side,
                            my_digest.content_hash,
                            their_digest.content_hash,
                        )
                    } else {
                        (
                            their_side,
                            my_side,
                            their_digest.content_hash,
                            my_digest.content_hash,
                        )
                    };
                let round = rendezvous_round(&[self.round, their_round], &lo, &hi);
                self.pending_ack.insert(
                    peer,
                    PendingReunion {
                        round: round.get(),
                        side_a,
                        side_b,
                        digest_lo: lo,
                        digest_hi: hi,
                    },
                );
                let requester_digest = my_digest.content_hash;
                let requester_side = self
                    .pending_ack
                    .get(&peer)
                    .map(|p| {
                        if requester_digest == p.digest_lo {
                            p.side_a.clone()
                        } else {
                            p.side_b.clone()
                        }
                    })
                    .expect("pending reunion just inserted");
                self.swarm.behaviour_mut().rr.send_request(
                    &peer,
                    RrRequest::ReunionAck {
                        scope: self.scope.clone(),
                        round: round.get(),
                        digest_lo: lo,
                        digest_hi: hi,
                        requester_side,
                        requester_digest,
                    },
                );
            }
            RrResponse::Ack { agree } => {
                let Some(pending) = self.pending_ack.remove(&peer) else {
                    return;
                };
                if !agree {
                    self.say(
                        "[reunion] peer disagreed on coordinates; will retry on next digest"
                            .to_string(),
                    );
                    return;
                }
                self.perform_reunion(
                    pending.round,
                    pending.side_a,
                    pending.side_b,
                    pending.digest_lo,
                    pending.digest_hi,
                );
            }
            RrResponse::Unknown => {
                self.pending_fetch.remove(&peer);
            }
        }
    }

    fn report_residue(&mut self) {
        let Some(view) = self.kernel.view(&self.scope) else {
            return;
        };
        let lines: Vec<String> = view
            .residue()
            .iter()
            .map(|r| {
                let handled = if r.handled_by().is_some() {
                    " [handled by override]"
                } else {
                    ""
                };
                format!(
                    "[residue] {}{}: {}",
                    self.display(r.key().subject.as_str()),
                    handled,
                    r.tension().detail
                )
            })
            .collect();
        for line in lines {
            self.say(line);
        }
    }

    // ------------------------------------------------------------------
    // Presence
    // ------------------------------------------------------------------

    fn on_connected(&mut self, peer: PeerId, addr: Option<Multiaddr>) {
        if let Some(addr) = addr {
            self.address_book.entry(peer).or_default().insert(addr);
        }
        self.heal_pending.remove(&peer);
        let entry = self.presence.entry(peer).or_insert(Presence {
            connected: false,
            last_contact_round: self.round,
            last_observation_round: 0,
        });
        entry.connected = true;
        entry.last_contact_round = self.round;
        // On (re)contact, immediately share our digest so divergence
        // surfaces without waiting for the periodic beat.
        if let Some(digest) = self.view_digest() {
            self.publish("digests", GossipMsg::Digest(digest));
        }
        // And re-share our asserted state for peers that missed it.
        self.reshare_my_state();
    }

    fn on_disconnected(&mut self, peer: PeerId) {
        if let Some(entry) = self.presence.get_mut(&peer) {
            entry.connected = false;
        }
    }

    fn on_ping(&mut self, peer: PeerId, ok: bool) {
        let round = self.round;
        let entry = self.presence.entry(peer).or_insert(Presence {
            connected: true,
            last_contact_round: round,
            last_observation_round: 0,
        });
        if ok {
            entry.connected = true;
            entry.last_contact_round = round;
            // Debounced liveness witnessing keeps standing evidence fresh
            // without flooding the evidence layer.
            if round.saturating_sub(entry.last_observation_round) >= PRESENCE_DEBOUNCE_ROUNDS {
                entry.last_observation_round = round;
                if self.belief(&peer).is_some() {
                    self.witness(peer, ObservationMode::DirectContact);
                }
            }
        }
    }

    // ------------------------------------------------------------------
    // Tick: the standing policy beat
    // ------------------------------------------------------------------

    pub fn tick(&mut self) {
        self.round += 1;
        self.feed(Input::Tick);

        // Advancement: strengthen standing where evidence clears the gate.
        let ready: Vec<(SubjectId, Vec<_>, Confidence)> = {
            let Some(view) = self.kernel.view(&self.scope) else {
                return;
            };
            ready_to_advance(&self.evidence, view)
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
            self.feed(Input::CorroborationAssessed {
                scope: self.scope.clone(),
                subject,
                records,
                confidence,
            });
        }

        if self.round.is_multiple_of(DIGEST_SHARE_ROUNDS) {
            if let Some(digest) = self.view_digest() {
                self.publish("digests", GossipMsg::Digest(digest));
            }
        }
        if self.round.is_multiple_of(CLAIM_REFRESH_ROUNDS) {
            self.reshare_my_state();
        }
        // Post-heal reconnection: keep dialing until connections land.
        if self.round.is_multiple_of(3) && !self.heal_pending.is_empty() {
            let pending: Vec<PeerId> = self.heal_pending.iter().copied().collect();
            for peer in pending {
                if self.presence.get(&peer).is_some_and(|p| p.connected) {
                    self.heal_pending.remove(&peer);
                } else {
                    self.redial(peer);
                }
            }
        }
    }

    /// Re-share everything this node has asserted — its claims and its
    /// witness records — so lossy meshes and late joiners converge without
    /// needing a reunion for ordinary strengthening state.
    fn reshare_my_state(&mut self) {
        self.publish_join_claim();
        for claim in self.my_claims.clone() {
            self.publish("claims", GossipMsg::Claim(claim));
        }
        let mine = self
            .evidence
            .records_by_witness(&self.scope, &WitnessId::new(self.me.to_base58()));
        for record in mine {
            self.publish("claims", GossipMsg::Witness(record));
        }
    }

    // ------------------------------------------------------------------
    // Swarm event pump
    // ------------------------------------------------------------------

    pub fn on_swarm_event(&mut self, event: SwarmEvent<BehaviourEvent>) {
        match event {
            SwarmEvent::NewListenAddr { address, .. } => {
                let me = self.me;
                self.say(format!("[net] listening on {address}/p2p/{me}"));
                self.listen_addrs.push(address);
            }
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                // Only outbound remote addresses are dialable listen
                // addresses; inbound ones are ephemeral ports. Identify
                // supplies the real listen addresses either way.
                let addr = endpoint
                    .is_dialer()
                    .then(|| endpoint.get_remote_address().clone());
                self.on_connected(peer_id, addr);
            }
            SwarmEvent::ConnectionClosed {
                peer_id,
                num_established: 0,
                ..
            } => {
                self.on_disconnected(peer_id);
            }
            SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(gossipsub::Event::Message {
                message,
                ..
            })) => {
                self.on_gossip(message.source, &message.data);
            }
            SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(gossipsub::Event::Subscribed {
                topic,
                ..
            }))
                // A peer joined one of our room topics: the mesh is now
                // real, so (re)share the state that startup publishes may
                // have dropped as InsufficientPeers.
                if topic.as_str().starts_with(&format!("room:{}/", self.room)) => {
                    self.reshare_my_state();
                    if let Some(digest) = self.view_digest() {
                        self.publish("digests", GossipMsg::Digest(digest));
                    }
                }
            SwarmEvent::Behaviour(BehaviourEvent::Ping(ping::Event { peer, result, .. })) => {
                self.on_ping(peer, result.is_ok());
            }
            SwarmEvent::Behaviour(BehaviourEvent::Identify(identify::Event::Received {
                peer_id,
                info,
                ..
            })) => {
                for addr in info.listen_addrs {
                    self.address_book.entry(peer_id).or_default().insert(addr);
                }
            }
            SwarmEvent::Behaviour(BehaviourEvent::Rr(request_response::Event::Message {
                peer,
                message,
                ..
            })) => match message {
                request_response::Message::Request {
                    request, channel, ..
                } => {
                    self.on_rr_request(peer, request, channel);
                }
                request_response::Message::Response { response, .. } => {
                    self.on_rr_response(peer, response);
                }
            },
            SwarmEvent::Behaviour(BehaviourEvent::Rr(
                request_response::Event::OutboundFailure { peer, .. },
            )) => {
                self.pending_fetch.remove(&peer);
                self.pending_ack.remove(&peer);
            }
            _ => {}
        }
    }

    // ------------------------------------------------------------------
    // User commands
    // ------------------------------------------------------------------

    pub fn command(&mut self, line: &str) {
        let line = line.trim();
        if line.is_empty() {
            return;
        }
        if !line.starts_with('/') {
            self.publish(
                "chat",
                GossipMsg::Chat {
                    text: line.to_string(),
                },
            );
            let me = self.display(&self.me.to_base58());
            self.say(format!("● {me}: {line}"));
            return;
        }
        let mut parts = line.split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();
        match command {
            "/roster" => self.cmd_roster(),
            "/status" => self.cmd_status(),
            "/vouch" => self.cmd_target(&args, |node, peer| {
                node.witness(peer, ObservationMode::ChallengeResponse);
                let who = node.display(&peer.to_base58());
                node.say(format!("[you] vouched for {who}"));
            }),
            "/mute" => self.cmd_moderate(&args, Moderation::Mute),
            "/ban" => self.cmd_moderate(&args, Moderation::Ban),
            "/override" => self.cmd_override(&args),
            "/split" => self.cmd_split(&args),
            "/heal" => self.cmd_heal(),
            "/why" => self.cmd_why(&args),
            "/peers" => {
                let lines: Vec<String> = self
                    .presence
                    .iter()
                    .map(|(peer, p)| {
                        format!(
                            "  {} {}",
                            if p.connected { "up  " } else { "down" },
                            self.display(&peer.to_base58())
                        )
                    })
                    .collect();
                for line in lines {
                    self.say(line);
                }
            }
            "/help" => {
                self.say("commands: /roster /status /vouch <peer> /mute <peer> /ban <peer> /override <peer> <state> /split <peer...> /heal /why <peer> /peers /quit".to_string());
            }
            other => self.say(format!("[?] unknown command {other} (try /help)")),
        }
    }

    fn resolve_peer(&self, needle: &str) -> Option<PeerId> {
        // Exact peer id, nickname, or unique suffix match.
        if let Ok(peer) = needle.parse::<PeerId>() {
            return Some(peer);
        }
        if let Some((peer, _)) = self.nickname.iter().find(|(_, nick)| *nick == needle) {
            return Some(*peer);
        }
        let known: BTreeSet<PeerId> = self
            .presence
            .keys()
            .copied()
            .chain(self.address_book.keys().copied())
            .collect();
        let matches: Vec<PeerId> = known
            .into_iter()
            .filter(|p| p.to_base58().ends_with(needle))
            .collect();
        (matches.len() == 1).then(|| matches[0])
    }

    fn cmd_target(&mut self, args: &[&str], act: impl FnOnce(&mut Self, PeerId)) {
        let Some(peer) = args.first().and_then(|a| self.resolve_peer(a)) else {
            self.say("[?] unknown peer (use a peer id or unique suffix)".to_string());
            return;
        };
        act(self, peer);
    }

    fn cmd_moderate(&mut self, args: &[&str], action: Moderation) {
        let reason = args.get(1..).map_or(String::new(), |r| r.join(" "));
        self.cmd_target(&args[..args.len().min(1)], move |node, peer| {
            let msg = GossipMsg::Moderation {
                subject: peer.to_base58(),
                action,
                reason: if reason.is_empty() {
                    format!("{action:?} by moderator")
                } else {
                    reason
                },
            };
            // Apply locally through the same inbound path, then share.
            let data = wire::encode(msg.clone());
            node.on_gossip(Some(node.me), &data);
            node.publish("claims", msg);
        });
    }

    /// Publish a visible operator override on any subject. Receivers honor
    /// it only from the room/federation creator; locally it applies through
    /// the same inbound path as everyone else's copy.
    pub fn publish_override(&mut self, subject: String, forced: BeliefState, reason: String) {
        // The override is fresh authoritative evidence: it advances the
        // subject's epoch past everything currently known, so views that
        // apply it late still dominate stale conflict through freshness
        // instead of re-disputing the operator's visible decision.
        let current = self
            .view_digest()
            .and_then(|d| {
                d.subject_summaries
                    .get(&SubjectId::new(subject.clone()))
                    .copied()
            })
            .map_or(1, |(_, epoch)| epoch.get());
        let op = OperatorOverride {
            operator: OperatorId::new(self.me.to_base58()),
            subject: SubjectId::new(subject),
            forced,
            reason,
            epoch: Epoch(current + 1),
        };
        let msg = GossipMsg::Override(op);
        let data = wire::encode(msg.clone());
        self.on_gossip(Some(self.me), &data);
        self.publish("claims", msg);
    }

    fn cmd_override(&mut self, args: &[&str]) {
        if self.me != self.creator {
            self.say("[?] only the room creator may override".to_string());
            return;
        }
        let Some(peer) = args.first().and_then(|a| self.resolve_peer(a)) else {
            self.say("[?] unknown peer".to_string());
            return;
        };
        let state = match args.get(1).copied() {
            Some("quarantined") | None => BeliefState::Quarantined,
            Some("removed") => BeliefState::Removed,
            Some("accepted") => BeliefState::Accepted,
            Some(other) => {
                self.say(format!("[?] cannot override to {other}"));
                return;
            }
        };
        self.publish_override(
            peer.to_base58(),
            state,
            "creator override after visible dispute".into(),
        );
    }

    fn cmd_split(&mut self, args: &[&str]) {
        let peers: Vec<PeerId> = args.iter().filter_map(|a| self.resolve_peer(a)).collect();
        if peers.is_empty() {
            self.say(
                "[?] /split <peer...> — blocks those peers to simulate a partition".to_string(),
            );
            return;
        }
        for peer in peers {
            self.blocked.insert(peer);
            self.swarm.behaviour_mut().blocklist.block_peer(peer);
            let who = self.display(&peer.to_base58());
            self.say(format!("[net] partitioned away from {who}"));
        }
    }

    fn cmd_heal(&mut self) {
        let blocked: Vec<PeerId> = self.blocked.iter().copied().collect();
        for peer in blocked {
            self.swarm.behaviour_mut().blocklist.unblock_peer(peer);
            // Unblocking does not reconnect: redial until a connection
            // lands (retried on ticks — a single dial can lose the race
            // against the peer's own heal).
            self.heal_pending.insert(peer);
            self.redial(peer);
        }
        self.blocked.clear();
        self.say("[net] partition healed; redialing".to_string());
    }

    fn redial(&mut self, peer: PeerId) {
        let addrs: Vec<Multiaddr> = self
            .address_book
            .get(&peer)
            .map(|a| a.iter().cloned().collect())
            .unwrap_or_default();
        for addr in addrs {
            let _ = self.swarm.dial(addr);
        }
    }

    fn cmd_roster(&mut self) {
        let Some(view) = self.kernel.view(&self.scope) else {
            self.say("[roster] no room view yet".to_string());
            return;
        };
        let mut lines = vec!["[roster] standing in this room:".to_string()];
        for (subject, cell) in view.subjects() {
            let summary = self.evidence.summarize(&self.scope, subject);
            lines.push(format!(
                "  {} {:12} {} (witnesses: {} {:?}/{:?})",
                wire::state_glyph(cell.state()),
                cell.state(),
                self.display(subject.as_str()),
                summary.count,
                summary.quality,
                summary.diversity,
            ));
        }
        for line in lines {
            self.say(line);
        }
    }

    fn cmd_status(&mut self) {
        let Some(digest) = self.view_digest() else {
            self.say("[status] no room view yet".to_string());
            return;
        };
        let hash: String = digest.content_hash[..6]
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        self.say(format!(
            "[status] round {} | digest {hash} | residue: {} live ({} unhandled) | disputes: {}",
            self.round,
            digest.residue_ids.len(),
            digest.unhandled_residue,
            digest.live_disputes,
        ));
        self.report_residue();
    }

    fn cmd_why(&mut self, args: &[&str]) {
        let Some(peer) = args.first().and_then(|a| self.resolve_peer(a)) else {
            self.say("[?] unknown peer".to_string());
            return;
        };
        let subject = SubjectId::new(peer.to_base58());
        let story: Vec<String> = self
            .transcript
            .explain(&self.scope, &subject)
            .iter()
            .map(|sealed| format!("  #{} {:?}", sealed.seq, sealed.event))
            .collect();
        let who = self.display(&peer.to_base58());
        self.say(format!(
            "[why] transcript for {who} ({} events):",
            story.len()
        ));
        for line in story.iter().rev().take(12).rev() {
            self.say(line.clone());
        }
    }

    /// Drive the node until the next natural pause. Used by main and tests.
    pub async fn poll(&mut self) {
        let event = self.swarm.select_next_some().await;
        self.on_swarm_event(event);
    }
}

fn hash_pair(a: [u8; 32], b: [u8; 32]) -> ([u8; 32], [u8; 32]) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Divergence worth a deterministic reunion: a restrictive-vs-permissive
/// class conflict on any subject, or live unresolved disagreement on
/// either side.
fn material_divergence(local: &RepairDigest, remote: &RepairDigest) -> bool {
    if local.has_unresolved_disagreement() || remote.has_unresolved_disagreement() {
        return true;
    }
    let restrictive = |s: BeliefState| matches!(s, BeliefState::Removed | BeliefState::Quarantined);
    let permissive = |s: BeliefState| matches!(s, BeliefState::Accepted | BeliefState::Provisional);
    local
        .subject_summaries
        .iter()
        .any(|(subject, (local_state, _))| {
            remote
                .subject_summaries
                .get(subject)
                .is_some_and(|(remote_state, _)| {
                    (restrictive(*local_state) && permissive(*remote_state))
                        || (restrictive(*remote_state) && permissive(*local_state))
                })
        })
}
