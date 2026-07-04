//! resonant-blockd — a federated blocklist daemon on the same standing
//! machinery as the chat: servers are peers who earn membership standing
//! in a federation scope, and blocked handles are subjects introduced with
//! `AssertedState::Compromised`. A block *proposal* becomes an *accepted*
//! block only when independently-vouched servers corroborate it — the same
//! evidence gate that stops sockpuppets stops blocklist brigading, and
//! cross-federation disagreements survive partitions as visible disputes
//! instead of silently winning by arrival order.
#![forbid(unsafe_code)]

use clap::Parser;
use libp2p::identity::Keypair;
use libp2p::{Multiaddr, PeerId};
use resonant_kernel::belief::BeliefState;
use resonant_kernel::evidence::{AssertedState, ObservationMode, Stance};
use resonant_kernel::id::SubjectId;
use resonant_net::node::{AppProfile, Node, NodeConfig};
use tokio::io::{AsyncBufReadExt, BufReader};

/// Federated blocklist daemon on the resonant standing kernel.
#[derive(Parser)]
#[command(name = "resonant-blockd", version)]
struct Args {
    /// Federation to join.
    #[arg(long, default_value = "fediblock")]
    federation: String,
    /// Display name for this server.
    #[arg(long)]
    nick: Option<String>,
    /// Derive a stable identity from this seed (demo convenience).
    #[arg(long)]
    seed: Option<u8>,
    /// The federation root's peer id. Omit if you are founding it.
    #[arg(long)]
    root: Option<PeerId>,
    /// Listen addresses.
    #[arg(long, default_value = "/ip4/127.0.0.1/tcp/0")]
    listen: Vec<Multiaddr>,
    /// Peers to dial at startup.
    #[arg(long)]
    dial: Vec<Multiaddr>,
    /// Append every kernel input as JSONL here (audit + replay).
    #[arg(long)]
    input_log: Option<std::path::PathBuf>,
}

fn is_handle(subject: &str) -> bool {
    subject.parse::<PeerId>().is_err()
}

fn handle_command(node: &mut Node, line: &str) {
    let mut parts = line.split_whitespace();
    let command = parts.next().unwrap_or("");
    let args: Vec<&str> = parts.collect();
    match command {
        "/block" => {
            let Some(handle) = args.first() else {
                println!("[?] /block <handle> [reason...]");
                return;
            };
            let me = node.peer_id().to_base58();
            node.introduce_subject((*handle).to_string(), AssertedState::Compromised, me);
            // The proposer's own evidence counts as one confirmation.
            node.witness_subject(
                (*handle).to_string(),
                ObservationMode::AdminInspection,
                Stance::Corroborate,
            );
            println!("[block] proposed {handle}; awaiting corroboration from other servers");
        }
        "/confirm" => {
            let Some(handle) = args.first() else {
                println!("[?] /confirm <handle>");
                return;
            };
            node.witness_subject(
                (*handle).to_string(),
                ObservationMode::ChallengeResponse,
                Stance::Corroborate,
            );
            println!("[block] confirmed {handle}");
        }
        "/dispute" => {
            let Some(handle) = args.first() else {
                println!("[?] /dispute <handle>");
                return;
            };
            node.witness_subject(
                (*handle).to_string(),
                ObservationMode::AdminInspection,
                Stance::Dispute,
            );
            println!("[block] disputed {handle}");
        }
        "/list" => {
            println!("[list] block entries in this federation:");
            for row in node.roster() {
                if !is_handle(&row.subject) {
                    continue;
                }
                let verdict = match row.state {
                    BeliefState::Accepted => "BLOCKED",
                    BeliefState::Provisional => "blocked (provisional)",
                    BeliefState::Witnessed | BeliefState::Introduced => "proposed",
                    BeliefState::Disputed => "DISPUTED",
                    BeliefState::Quarantined => "suspended",
                    BeliefState::Removed => "retired",
                    _ => "unknown",
                };
                println!(
                    "  {:24} {:22} (witnesses: {} {:?}/{:?})",
                    row.subject,
                    verdict,
                    row.summary.count,
                    row.summary.quality,
                    row.summary.diversity
                );
            }
        }
        "/why" => {
            let Some(handle) = args.first() else {
                println!("[?] /why <handle>");
                return;
            };
            let story = node
                .transcript
                .explain(node.scope(), &SubjectId::new((*handle).to_string()));
            println!("[why] transcript for {handle} ({} events):", story.len());
            for sealed in story.iter().rev().take(12).rev() {
                println!("  #{} {:?}", sealed.seq, sealed.event);
            }
        }
        "/retire" => {
            let Some(handle) = args.first() else {
                println!("[?] /retire <handle> (federation root only)");
                return;
            };
            node.publish_override(
                (*handle).to_string(),
                BeliefState::Removed,
                "federation root retired the block entry".into(),
            );
        }
        // Everything else (status, peers, split, heal, quit-adjacent) is
        // shared machinery.
        _ => node.command(line),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let keypair = match args.seed {
        Some(seed) => {
            let mut bytes = [0u8; 32];
            bytes[0] = seed;
            bytes[31] = 0xb1;
            Keypair::ed25519_from_bytes(bytes)?
        }
        None => Keypair::generate_ed25519(),
    };
    let peer_id = keypair.public().to_peer_id();
    println!("resonant-blockd — server id {peer_id}");
    if args.root.is_none() {
        println!("(you are the federation root; others join with --root {peer_id})");
    }

    let mut node = Node::new(NodeConfig {
        profile: AppProfile::federation(),
        keypair,
        room: args.federation,
        nickname: args.nick,
        creator: args.root,
        voucher: None,
        listen: args.listen,
        dial: args.dial,
        input_log: args.input_log,
        interactive: true,
    })?;

    let mut stdin = BufReader::new(tokio::io::stdin()).lines();
    let mut ticker = tokio::time::interval(std::time::Duration::from_secs(1));

    loop {
        node.output.clear();
        tokio::select! {
            _ = node.poll() => {}
            line = stdin.next_line() => {
                match line? {
                    Some(line) if line.trim() == "/quit" => break,
                    Some(line) if !line.trim().is_empty() => handle_command(&mut node, line.trim()),
                    Some(_) => {}
                    None => break,
                }
            }
            _ = ticker.tick() => node.tick(),
        }
    }
    Ok(())
}
