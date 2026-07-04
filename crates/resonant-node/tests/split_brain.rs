//! End-to-end split-brain over real libp2p transport (TCP loopback):
//! three nodes reach accepted standing, partition 2-vs-1, the majority
//! side bans the isolated node while it keeps believing in itself, then
//! the partition heals and deterministic reunion converges every node on
//! the same honest view — the ban survives as a visible dispute, and the
//! creator's override quarantines it without erasing the scar.
//!
//! Swarm interleaving is nondeterministic, so the test drives to
//! *quiescent convergence* (digest equality with a timeout) rather than
//! asserting step-lockstep; per-node determinism is covered by the
//! kernel's own replay verification.

use libp2p::identity::Keypair;
use resonant_kernel::belief::BeliefState;
use resonant_node::node::{Node, NodeConfig};
use std::time::Duration;

fn keypair(seed: u8) -> Keypair {
    let mut bytes = [0u8; 32];
    bytes[0] = seed;
    bytes[31] = 0x5e;
    Keypair::ed25519_from_bytes(bytes).expect("valid ed25519 seed")
}

fn config(seed: u8, creator: Option<libp2p::PeerId>, dial: Vec<libp2p::Multiaddr>) -> NodeConfig {
    NodeConfig {
        profile: resonant_node::node::AppProfile::chat(),
        keypair: keypair(seed),
        room: "testroom".into(),
        nickname: None,
        creator,
        voucher: None,
        listen: vec!["/ip4/127.0.0.1/tcp/0".parse().unwrap()],
        dial,
        input_log: None,
        interactive: false,
    }
}

/// Drive all nodes concurrently until `check` passes or the deadline hits.
/// Ticks every ~60ms so hysteresis rounds pass quickly.
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
        assert!(
            start.elapsed() < deadline,
            "timed out waiting for: {label}\n{}",
            nodes
                .iter()
                .map(|n| {
                    let roster: Vec<String> = n
                        .roster()
                        .iter()
                        .map(|r| {
                            format!(
                                "{}={} res{}",
                                &r.subject[r.subject.len() - 4..],
                                r.state,
                                n.residues()
                                    .iter()
                                    .filter(|x| x.subject == r.subject)
                                    .count()
                            )
                        })
                        .collect();
                    let tail: Vec<&String> = n.output.iter().rev().take(6).collect();
                    format!(
                        "  {}: digest {:?} [{}] log_tail={:?}",
                        n.peer_id(),
                        n.view_digest().map(|d| d.content_hash[..4].to_vec()),
                        roster.join(", "),
                        tail
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        );
        // Poll every node's swarm without blocking on any single one.
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

fn all_believe(nodes: &[&mut Node], subject: libp2p::PeerId, state: BeliefState) -> bool {
    nodes.iter().all(|n| n.belief(&subject) == Some(state))
}

fn converged(nodes: &[&mut Node]) -> bool {
    let hashes: Vec<_> = nodes
        .iter()
        .map(|n| n.view_digest().map(|d| d.content_hash))
        .collect();
    hashes
        .first()
        .is_some_and(|first| first.is_some() && hashes.iter().all(|h| h == first))
}

#[tokio::test(flavor = "current_thread")]
async fn split_brain_ban_survives_reunion() {
    let creator_peer = keypair(1).public().to_peer_id();

    let mut alice = Node::new(config(1, None, vec![])).expect("alice");
    // Get alice's listen address before the others dial.
    drive_until(
        &mut [&mut alice],
        Duration::from_secs(10),
        "alice listening",
        |nodes| !nodes[0].listen_addrs.is_empty(),
    )
    .await;
    let alice_addr = alice.listen_addrs[0].clone();

    let mut bob = Node::new(config(2, Some(creator_peer), vec![alice_addr.clone()])).expect("bob");
    let mut carol =
        Node::new(config(3, Some(creator_peer), vec![alice_addr.clone()])).expect("carol");
    let bob_peer = bob.peer_id();
    let carol_peer = carol.peer_id();

    // Bob and carol also need to find each other; dial via alice's mesh
    // won't connect them directly, so exchange addresses once known.
    drive_until(
        &mut [&mut alice, &mut bob, &mut carol],
        Duration::from_secs(10),
        "bob and carol listening",
        |nodes| nodes.iter().all(|n| !n.listen_addrs.is_empty()),
    )
    .await;
    let carol_addr = carol.listen_addrs[0].clone();
    bob.swarm.dial(carol_addr).expect("bob dials carol");

    // Everyone reaches accepted standing: each subject is witnessed by
    // the other two (independent lineages under the creator).
    drive_until(
        &mut [&mut alice, &mut bob, &mut carol],
        Duration::from_secs(30),
        "all members accepted",
        |nodes| {
            all_believe(nodes, bob_peer, BeliefState::Accepted)
                && all_believe(nodes, carol_peer, BeliefState::Accepted)
        },
    )
    .await;

    // Partition: carol alone on one side. Both sides block, so no path
    // survives.
    let carol_b58 = carol_peer.to_base58();
    let alice_b58 = alice.peer_id().to_base58();
    let bob_b58 = bob_peer.to_base58();
    alice.command(&format!("/split {carol_b58}"));
    bob.command(&format!("/split {carol_b58}"));
    carol.command(&format!("/split {alice_b58} {bob_b58}"));

    // The majority side bans carol; carol's island keeps her accepted.
    alice.command(&format!("/ban {carol_b58} testing split-brain"));
    drive_until(
        &mut [&mut alice, &mut bob],
        Duration::from_secs(15),
        "majority side bans carol",
        |nodes| all_believe(nodes, carol_peer, BeliefState::Removed),
    )
    .await;
    assert_eq!(
        carol.belief(&carol_peer),
        Some(BeliefState::Accepted),
        "carol's island disagrees"
    );

    // Heal and converge: the conflict must survive as a visible dispute
    // with residue on every node, and all digests must match.
    alice.command("/heal");
    bob.command("/heal");
    carol.command("/heal");
    drive_until(
        &mut [&mut alice, &mut bob, &mut carol],
        Duration::from_secs(30),
        "reunion converges with carol disputed",
        |nodes| all_believe(nodes, carol_peer, BeliefState::Disputed) && converged(nodes),
    )
    .await;
    for node in [&alice, &bob, &carol] {
        let digest = node.view_digest().unwrap();
        assert!(
            digest.unhandled_residue >= 1,
            "the dispute must leave visible residue"
        );
    }

    // The creator resolves it visibly: quarantine, scar marked handled.
    alice.command(&format!("/override {carol_b58} quarantined"));
    drive_until(
        &mut [&mut alice, &mut bob, &mut carol],
        Duration::from_secs(20),
        "override quarantines carol everywhere",
        |nodes| all_believe(nodes, carol_peer, BeliefState::Quarantined) && converged(nodes),
    )
    .await;
    for node in [&alice, &bob, &carol] {
        let digest = node.view_digest().unwrap();
        assert_eq!(digest.unhandled_residue, 0, "override takes responsibility");
        assert!(!digest.residue_ids.is_empty(), "the scar is never erased");
    }

    // The whole story is explainable from any node's transcript.
    alice
        .transcript
        .verify_chain()
        .expect("transcript chain intact");
    let story = alice.transcript.explain(
        alice.scope(),
        &resonant_kernel::id::SubjectId::new(carol_b58),
    );
    assert!(!story.is_empty(), "the ban has a causal story");
}
