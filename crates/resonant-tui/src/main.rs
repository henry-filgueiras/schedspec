//! resonant-tui — the whole split-brain story in one window.
//!
//! Spawns three in-process chat nodes (alice the creator, bob, carol) over
//! real TCP loopback and renders them side by side: log pane, standing
//! roster, and residue panel per node, with a convergence strip showing
//! each node's digest hash. Function keys drive the story beats; the input
//! line talks as whichever node has focus.
#![forbid(unsafe_code)]

use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use futures::StreamExt;
use libp2p::identity::Keypair;
use libp2p::PeerId;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;
use resonant_net::node::{AppProfile, Node, NodeConfig};
use resonant_net::wire::state_glyph;
use std::time::Duration;

const NAMES: [&str; 3] = ["alice", "bob", "carol"];
const LOG_KEEP: usize = 300;

struct Pane {
    name: &'static str,
    peer: PeerId,
    log: Vec<String>,
}

struct App {
    nodes: Vec<Node>,
    panes: Vec<Pane>,
    focus: usize,
    input: String,
    partitioned: bool,
    banner: String,
}

fn keypair(seed: u8) -> Keypair {
    let mut bytes = [0u8; 32];
    bytes[0] = seed;
    bytes[31] = 0x70;
    Keypair::ed25519_from_bytes(bytes).expect("valid seed")
}

impl App {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let creator = keypair(1).public().to_peer_id();
        let config = |seed: u8, name: &str, creator_id: Option<PeerId>, dial| NodeConfig {
            profile: AppProfile::chat(),
            keypair: keypair(seed),
            room: "demo".into(),
            nickname: Some(name.to_string()),
            creator: creator_id,
            voucher: None,
            listen: vec!["/ip4/127.0.0.1/tcp/0".parse().unwrap()],
            dial,
            input_log: None,
            interactive: false,
        };

        let mut alice = Node::new(config(1, "alice", None, vec![]))?;
        while alice.listen_addrs.is_empty() {
            alice.poll().await;
        }
        let alice_addr = alice.listen_addrs[0].clone();

        let mut bob = Node::new(config(2, "bob", Some(creator), vec![alice_addr.clone()]))?;
        while bob.listen_addrs.is_empty() {
            bob.poll().await;
        }
        let bob_addr = bob.listen_addrs[0].clone();
        let carol = Node::new(config(
            3,
            "carol",
            Some(creator),
            vec![alice_addr, bob_addr],
        ))?;

        let mut nodes = vec![alice, bob, carol];
        let peers: Vec<PeerId> = nodes.iter().map(|n| n.peer_id()).collect();
        // Everyone knows everyone's display name.
        for node in &mut nodes {
            for (peer, name) in peers.iter().zip(NAMES) {
                node.set_nickname(*peer, name.to_string());
            }
        }
        let panes = peers
            .iter()
            .zip(NAMES)
            .map(|(peer, name)| Pane {
                name,
                peer: *peer,
                log: vec![format!("~ {name} is online")],
            })
            .collect();

        Ok(Self {
            nodes,
            panes,
            focus: 0,
            input: String::new(),
            partitioned: false,
            banner: "F2 partition · F3 ban carol · F4 heal · F5 override · Tab focus · Esc quit"
                .into(),
        })
    }

    fn drain_output(&mut self) {
        for (node, pane) in self.nodes.iter_mut().zip(&mut self.panes) {
            for line in node.output.drain(..) {
                pane.log.push(line);
            }
            if pane.log.len() > LOG_KEEP {
                let cut = pane.log.len() - LOG_KEEP;
                pane.log.drain(..cut);
            }
        }
    }

    fn beat(&mut self, label: &str) {
        for pane in &mut self.panes {
            pane.log.push(format!("== {label} =="));
        }
    }

    fn story_partition(&mut self) {
        if self.partitioned {
            return;
        }
        self.partitioned = true;
        self.beat("PARTITION: carol splits off");
        let carol = self.panes[2].peer.to_base58();
        let alice = self.panes[0].peer.to_base58();
        let bob = self.panes[1].peer.to_base58();
        self.nodes[0].command(&format!("/split {carol}"));
        self.nodes[1].command(&format!("/split {carol}"));
        self.nodes[2].command(&format!("/split {alice} {bob}"));
    }

    fn story_ban(&mut self) {
        self.beat("alice's island bans carol");
        let carol = self.panes[2].peer.to_base58();
        self.nodes[0].command(&format!("/ban {carol} posting spam during the split"));
    }

    fn story_heal(&mut self) {
        if !self.partitioned {
            return;
        }
        self.partitioned = false;
        self.beat("HEAL: islands reconnect, deterministic reunion");
        for node in &mut self.nodes {
            node.command("/heal");
        }
    }

    fn story_override(&mut self) {
        self.beat("creator override: carol -> quarantined");
        let carol = self.panes[2].peer.to_base58();
        self.nodes[0].command(&format!("/override {carol} quarantined"));
    }

    fn submit_input(&mut self) {
        let line = std::mem::take(&mut self.input);
        if line.trim().is_empty() {
            return;
        }
        self.nodes[self.focus].command(line.trim());
    }
}

fn digest_short(node: &Node) -> String {
    node.view_digest()
        .map(|d| {
            d.content_hash[..3]
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect()
        })
        .unwrap_or_else(|| "------".into())
}

fn draw(frame: &mut Frame, app: &App) {
    let [columns_area, status_area, input_area, help_area] = Layout::vertical([
        Constraint::Min(10),
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    let column_areas = Layout::horizontal([Constraint::Ratio(1, 3); 3]).split(columns_area);
    for (index, area) in column_areas.iter().enumerate() {
        draw_node_column(frame, *area, app, index);
    }

    // Convergence strip: digest per node, highlighted when all match.
    let digests: Vec<String> = app.nodes.iter().map(digest_short).collect();
    let converged = digests.iter().all(|d| d == &digests[0]);
    let status_style = if converged {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Yellow)
    };
    let status = Line::from(vec![
        Span::styled(
            if converged {
                " CONVERGED "
            } else {
                " DIVERGED "
            },
            status_style.add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(
            " digests: alice={} bob={} carol={}   partition: {}",
            digests[0],
            digests[1],
            digests[2],
            if app.partitioned { "ACTIVE" } else { "none" },
        )),
    ]);
    frame.render_widget(Paragraph::new(status), status_area);

    let input = Paragraph::new(app.input.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" say as {} ", app.panes[app.focus].name)),
    );
    frame.render_widget(input, input_area);

    frame.render_widget(
        Paragraph::new(app.banner.as_str()).style(Style::default().fg(Color::DarkGray)),
        help_area,
    );
}

fn draw_node_column(frame: &mut Frame, area: Rect, app: &App, index: usize) {
    let pane = &app.panes[index];
    let node = &app.nodes[index];
    let focused = app.focus == index;

    let roster = node.roster();
    let residues = node.residues();
    let roster_height = (roster.len() as u16 + 2).min(7);
    let residue_height = (residues.len() as u16 + 2).clamp(3, 6);

    let [log_area, roster_area, residue_area] = Layout::vertical([
        Constraint::Min(5),
        Constraint::Length(roster_height),
        Constraint::Length(residue_height),
    ])
    .areas(area);

    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let title = if pane.name == "alice" {
        format!(" {} (creator) ", pane.name)
    } else {
        format!(" {} ", pane.name)
    };

    let visible = log_area.rows().count().saturating_sub(2);
    let lines: Vec<Line> = pane
        .log
        .iter()
        .rev()
        .take(visible)
        .rev()
        .map(|l| style_log_line(l))
        .collect();
    frame.render_widget(
        Paragraph::new(lines).wrap(Wrap { trim: true }).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(title),
        ),
        log_area,
    );

    let roster_lines: Vec<Line> = roster
        .iter()
        .map(|row| {
            let name = node.display_name(&row.subject);
            let color = match row.state {
                resonant_kernel::belief::BeliefState::Accepted => Color::Green,
                resonant_kernel::belief::BeliefState::Disputed => Color::Red,
                resonant_kernel::belief::BeliefState::Quarantined => Color::Magenta,
                resonant_kernel::belief::BeliefState::Removed => Color::DarkGray,
                _ => Color::Yellow,
            };
            Line::from(vec![
                Span::styled(
                    format!("{} {:11}", state_glyph(row.state), row.state),
                    Style::default().fg(color),
                ),
                Span::raw(format!(
                    " {name} ({} {:?}/{:?})",
                    row.summary.count, row.summary.quality, row.summary.diversity
                )),
            ])
        })
        .collect();
    frame.render_widget(
        Paragraph::new(roster_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(" standing "),
        ),
        roster_area,
    );

    let residue_lines: Vec<Line> = if residues.is_empty() {
        vec![Line::from(Span::styled(
            "(none)",
            Style::default().fg(Color::DarkGray),
        ))]
    } else {
        residues
            .iter()
            .map(|r| {
                let mark = if r.handled_by_override {
                    " [handled]"
                } else {
                    ""
                };
                Line::from(Span::styled(
                    format!("‼ {}{}: {}", node.display_name(&r.subject), mark, r.detail),
                    Style::default().fg(if r.handled_by_override {
                        Color::Magenta
                    } else {
                        Color::Red
                    }),
                ))
            })
            .collect()
    };
    frame.render_widget(
        Paragraph::new(residue_lines)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title(" residue "),
            ),
        residue_area,
    );
}

fn style_log_line(line: &str) -> Line<'_> {
    let style = if line.starts_with("==") {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else if line.starts_with("[reunion]") {
        Style::default().fg(Color::Green)
    } else if line.starts_with("[residue]") {
        Style::default().fg(Color::Red)
    } else if line.starts_with("[mod]") {
        Style::default().fg(Color::Magenta)
    } else if line.starts_with("[net]") || line.starts_with('~') {
        Style::default().fg(Color::DarkGray)
    } else {
        Style::default()
    };
    Line::from(Span::styled(line.to_string(), style))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new().await?;

    let mut terminal = ratatui::init();
    let mut events = EventStream::new();
    let mut ticker = tokio::time::interval(Duration::from_millis(500));
    let mut frame_timer = tokio::time::interval(Duration::from_millis(120));

    let result: Result<(), Box<dyn std::error::Error>> = loop {
        // Drive all three swarms without blocking on any single one.
        for node in &mut app.nodes {
            while let std::task::Poll::Ready(Some(event)) =
                futures::poll!(std::pin::pin!(node.swarm.next()))
            {
                node.on_swarm_event(event);
            }
        }
        app.drain_output();
        if let Err(e) = terminal.draw(|frame| draw(frame, &app)) {
            break Err(e.into());
        }

        tokio::select! {
            _ = ticker.tick() => {
                for node in &mut app.nodes {
                    node.tick();
                }
            }
            _ = frame_timer.tick() => {}
            maybe_event = events.next() => {
                let Some(Ok(event)) = maybe_event else { continue };
                let Event::Key(key) = event else { continue };
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Esc => break Ok(()),
                    KeyCode::F(2) => app.story_partition(),
                    KeyCode::F(3) => app.story_ban(),
                    KeyCode::F(4) => app.story_heal(),
                    KeyCode::F(5) => app.story_override(),
                    KeyCode::Tab => app.focus = (app.focus + 1) % app.nodes.len(),
                    KeyCode::Enter => app.submit_input(),
                    KeyCode::Backspace => { app.input.pop(); }
                    KeyCode::Char(c) => app.input.push(c),
                    _ => {}
                }
            }
        }
    };

    ratatui::restore();
    result
}
