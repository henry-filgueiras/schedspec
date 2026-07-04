//! The blocklist skin over the same standing machinery: a proposed block
//! becomes an accepted block only under independently-vouched
//! corroboration, and all servers converge on the same view.

use libp2p::identity::Keypair;
use resonant_kernel::belief::BeliefState;
use resonant_kernel::evidence::{AssertedState, ObservationMode, Stance};
use resonant_net::node::{AppProfile, Node, NodeConfig};
use std::time::Duration;

fn keypair(seed: u8) -> Keypair {
    let mut bytes = [0u8; 32];
    bytes[0] = seed;
    bytes[31] = 0xb1;
    Keypair::ed25519_from_bytes(bytes).expect("valid ed25519 seed")
}

fn config(seed: u8, root: Option<libp2p::PeerId>, dial: Vec<libp2p::Multiaddr>) -> NodeConfig {
    NodeConfig {
        profile: AppProfile::federation(),
        keypair: keypair(seed),
        room: "testfed".into(),
        nickname: None,
        creator: root,
        voucher: None,
        listen: vec!["/ip4/127.0.0.1/tcp/0".parse().unwrap()],
        dial,
        input_log: None,
        interactive: false,
    }
}

async fn drive_until(
    nodes: &mut [&mut Node],
    deadline: Duration,
    label: &str,
    mut check: impl FnMut(&[&mut Node]) -> bool,
) {
    let start = tokio::time::Instant::now();
    let mut ticker = tokio::time::interval(Duration::from_millis(60));
    loop {
        if check(nodes) {
            return;
        }
        assert!(start.elapsed() < deadline, "timed out waiting for: {label}");
        for node in nodes.iter_mut() {
            while let std::task::Poll::Ready(Some(event)) =
                futures::poll!(std::pin::pin!(futures::StreamExt::next(&mut node.swarm)))
            {
                node.on_swarm_event(event);
            }
        }
        ticker.tick().await;
        for node in nodes.iter_mut() {
            node.tick();
        }
    }
}

#[tokio::test(flavor = "current_thread")]
async fn proposed_block_needs_independent_corroboration() {
    let root_peer = keypair(11).public().to_peer_id();

    let mut root = Node::new(config(11, None, vec![])).expect("root");
    drive_until(
        &mut [&mut root],
        Duration::from_secs(10),
        "root listening",
        |nodes| !nodes[0].listen_addrs.is_empty(),
    )
    .await;
    let root_addr = root.listen_addrs[0].clone();

    let mut south = Node::new(config(12, Some(root_peer), vec![root_addr.clone()])).expect("south");
    let mut west = Node::new(config(13, Some(root_peer), vec![root_addr.clone()])).expect("west");
    drive_until(
        &mut [&mut root, &mut south, &mut west],
        Duration::from_secs(10),
        "all listening",
        |nodes| nodes.iter().all(|n| !n.listen_addrs.is_empty()),
    )
    .await;
    let west_addr = west.listen_addrs[0].clone();
    south.swarm.dial(west_addr).expect("south dials west");

    // Let the federation mesh form: every server sees every other's join.
    drive_until(
        &mut [&mut root, &mut south, &mut west],
        Duration::from_secs(20),
        "federation mesh formed",
        |nodes| {
            nodes
                .iter()
                .all(|n| n.belief_of(&root_peer.to_base58()).is_some() && n.roster().len() == 3)
        },
    )
    .await;

    let handle = "spam-network.example";

    // Root proposes the block and supplies its own evidence.
    root.introduce_subject(
        handle.to_string(),
        AssertedState::Compromised,
        root_peer.to_base58(),
    );
    root.witness_subject(
        handle.to_string(),
        ObservationMode::AdminInspection,
        Stance::Corroborate,
    );

    // The proposal alone — one witness, one lineage — must not clear the
    // advancement gate: it propagates everywhere but stays a proposal.
    drive_until(
        &mut [&mut root, &mut south, &mut west],
        Duration::from_secs(10),
        "proposal propagates but stays short of acceptance",
        |nodes| {
            nodes
                .iter()
                .all(|n| n.belief_of(handle) == Some(BeliefState::Witnessed))
        },
    )
    .await;

    // A second, independently-vouched server confirms.
    south.witness_subject(
        handle.to_string(),
        ObservationMode::ChallengeResponse,
        Stance::Corroborate,
    );

    drive_until(
        &mut [&mut root, &mut south, &mut west],
        Duration::from_secs(20),
        "block accepted everywhere after corroboration",
        |nodes| {
            let states = nodes
                .iter()
                .all(|n| n.belief_of(handle) == Some(BeliefState::Accepted));
            let hashes: Vec<_> = nodes
                .iter()
                .map(|n| n.view_digest().map(|d| d.content_hash))
                .collect();
            let converged = hashes
                .first()
                .is_some_and(|f| f.is_some() && hashes.iter().all(|h| h == f));
            states && converged
        },
    )
    .await;

    // The block entry has a causal story on every server.
    let story = west
        .transcript
        .explain(west.scope(), &resonant_kernel::id::SubjectId::new(handle));
    assert!(!story.is_empty(), "the block has provenance");
}
