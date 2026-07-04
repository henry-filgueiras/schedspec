#![forbid(unsafe_code)]

use clap::Parser;
use libp2p::identity::Keypair;
use libp2p::{Multiaddr, PeerId};
use resonant_node::node::{Node, NodeConfig};
use tokio::io::{AsyncBufReadExt, BufReader};

/// P2P group chat where moderation standing survives partitions.
#[derive(Parser)]
#[command(name = "resonant-chat", version)]
struct Args {
    /// Room to join.
    #[arg(long, default_value = "lobby")]
    room: String,
    /// Display name for your own messages.
    #[arg(long)]
    nick: Option<String>,
    /// Derive a stable identity from this seed (demo convenience).
    #[arg(long)]
    seed: Option<u8>,
    /// The room creator's peer id. Omit if you are creating the room.
    #[arg(long)]
    creator: Option<PeerId>,
    /// Peer vouching your join (defaults to the creator).
    #[arg(long)]
    voucher: Option<PeerId>,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let keypair = match args.seed {
        Some(seed) => {
            let mut bytes = [0u8; 32];
            bytes[0] = seed;
            bytes[31] = 0x5e;
            Keypair::ed25519_from_bytes(bytes)?
        }
        None => Keypair::generate_ed25519(),
    };
    let peer_id = keypair.public().to_peer_id();
    println!("resonant-chat — peer id {peer_id}");
    if args.creator.is_none() {
        println!("(you are the room creator; others join with --creator {peer_id})");
    }

    let mut node = Node::new(NodeConfig {
        profile: resonant_node::node::AppProfile::chat(),
        keypair,
        room: args.room,
        nickname: args.nick,
        creator: args.creator,
        voucher: args.voucher,
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
                    Some(line) => node.command(&line),
                    None => break,
                }
            }
            _ = ticker.tick() => node.tick(),
        }
    }
    Ok(())
}
