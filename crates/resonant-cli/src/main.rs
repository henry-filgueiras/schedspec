//! `resonant` — operator CLI for the resonant membership reference kernel.
#![forbid(unsafe_code)]

use clap::{Args, Parser, Subcommand};
use resonant_kernel::epoch::{Epoch, Round};
use resonant_kernel::id::PeerId;
use resonant_kernel::policy::MergePolicy;
use resonant_kernel::rank::{permutation_rank, reconstruct, RankDomain, RankSeed};
use resonant_kernel::scope::ScopeId;
use resonant_lab::conformance::{compare, run_kernel_reunion};
use resonant_lab::golden::{check_case, golden_cases};
use resonant_lab::oracle::{
    build_naive_comparison, compute_island_digest, run_deterministic_merge,
};
use resonant_lab::replay::materialize;
use resonant_lab::scenario::{default_scenarios_dir, load_corpus, load_index, Scenario};
use resonant_lab::sim::{run as run_sim, SimConfig};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(
    name = "resonant",
    about = "Deterministic reunion, accountable rank, and visible residue — the resonant membership treatise, executable.",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Work with the canonical scenario corpus (docs/scenarios).
    #[command(subcommand)]
    Scenario(ScenarioCommand),
    /// Demonstrate accountable permutation rank.
    Rank(RankArgs),
    /// Run the deterministic partition/heal simulation.
    #[command(subcommand)]
    Sim(SimCommand),
}

#[derive(Subcommand)]
enum ScenarioCommand {
    /// List the scenarios in the corpus.
    List(CorpusArgs),
    /// Replay a scenario and show the deterministic reunion.
    Run(ScenarioRunArgs),
    /// Check every scenario against the golden outcome table and the
    /// kernel-vs-oracle conformance sweep.
    Verify(CorpusArgs),
}

#[derive(Args)]
struct CorpusArgs {
    /// Directory holding index.json and the scenario files.
    #[arg(long)]
    scenarios_dir: Option<PathBuf>,
}

#[derive(Args)]
struct ScenarioRunArgs {
    /// Scenario id (see `resonant scenario list`).
    id: String,
    /// Replay only the first N divergence events (default: all).
    #[arg(long)]
    steps: Option<usize>,
    /// Apply the scenario's operator override, if it allows one.
    #[arg(long = "override")]
    apply_override: bool,
    /// Also show what a naive "latest or loudest wins" reunion would do.
    #[arg(long)]
    naive: bool,
    /// Emit machine-readable JSON instead of tables.
    #[arg(long)]
    json: bool,
    #[command(flatten)]
    corpus: CorpusArgs,
}

#[derive(Args)]
struct RankArgs {
    /// Seed round (part of the rank seed; vary it to watch rotation).
    #[arg(long, default_value_t = 0)]
    round: u64,
    /// Epoch component of the rank seed.
    #[arg(long, default_value_t = 1)]
    epoch: u64,
    /// Scope component of the rank seed.
    #[arg(long, default_value = "demo")]
    scope: String,
    /// Comma-separated candidate ids.
    #[arg(long, value_delimiter = ',', required = true)]
    candidates: Vec<String>,
    /// Select the first K of the ranked order.
    #[arg(long)]
    take: Option<usize>,
    /// Show the per-candidate tokens and verify reconstruction.
    #[arg(long)]
    audit: bool,
}

#[derive(Subcommand)]
enum SimCommand {
    /// Run the simulation.
    Run(SimRunArgs),
}

#[derive(Args)]
struct SimRunArgs {
    #[arg(long, default_value_t = 42)]
    seed: u64,
    #[arg(long, default_value_t = 8)]
    nodes: u32,
    #[arg(long, default_value_t = 4)]
    subjects: u32,
    /// Apply a visible operator override to the disputed subject.
    #[arg(long = "override")]
    apply_override: bool,
    #[arg(long)]
    json: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Scenario(ScenarioCommand::List(args)) => scenario_list(&args),
        Command::Scenario(ScenarioCommand::Run(args)) => scenario_run(&args),
        Command::Scenario(ScenarioCommand::Verify(args)) => scenario_verify(&args),
        Command::Rank(args) => rank(&args),
        Command::Sim(SimCommand::Run(args)) => sim_run(&args),
    }
}

fn corpus_dir(args: &CorpusArgs) -> PathBuf {
    args.scenarios_dir
        .clone()
        .unwrap_or_else(default_scenarios_dir)
}

fn scenario_list(args: &CorpusArgs) -> ExitCode {
    let dir = corpus_dir(args);
    match load_index(&dir) {
        Ok(index) => {
            println!("scenario corpus: {} ({})", index.lab, dir.display());
            for entry in index.scenarios {
                println!("  {:44} {}", entry.id, entry.title);
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn find_scenario(dir: &std::path::Path, id: &str) -> Result<Scenario, String> {
    let corpus = load_corpus(dir).map_err(|e| e.to_string())?;
    corpus.into_iter().find(|s| s.id == id).ok_or_else(|| {
        format!(
            "no scenario '{id}' in {} (try `resonant scenario list`)",
            dir.display()
        )
    })
}

fn scenario_run(args: &ScenarioRunArgs) -> ExitCode {
    let dir = corpus_dir(&args.corpus);
    let scenario = match find_scenario(&dir, &args.id) {
        Ok(s) => s,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };

    let steps = args.steps.unwrap_or(scenario.events.len());
    let policy = MergePolicy::lab_compat();
    let current = materialize(&scenario, steps);

    let mut apply_override = args.apply_override;
    if apply_override && !scenario.allow_operator_override {
        eprintln!("note: this scenario does not allow an operator override; ignoring --override");
        apply_override = false;
    }
    let merge = run_deterministic_merge(&policy, &scenario, &current, apply_override);
    if apply_override {
        if let Some(op) = &scenario.operator_override {
            let target = merge.members.iter().find(|m| m.subject_id == op.subject_id);
            // Mirror the lab UI's gate as a warning: overrides are meant
            // for subjects the merge left disputed.
            if target.is_some_and(|t| {
                t.status != resonant_kernel::belief::BeliefState::Quarantined
                    && t.rule_summary
                        .iter()
                        .all(|r| !r.contains("OperatorOverride"))
            }) {
                eprintln!("warning: override target was not disputed before intervention");
            }
        }
    }

    if args.json {
        let naive = args
            .naive
            .then(|| build_naive_comparison(&scenario, &current, &merge));
        let kernel = run_kernel_reunion(&policy, &scenario, &current, apply_override);
        let payload = serde_json::json!({
            "scenario": scenario.id,
            "steps_replayed": current.steps_replayed,
            "merge": merge,
            "naive": naive,
            "kernel_agrees": compare(&kernel, &merge).is_empty(),
        });
        println!("{}", serde_json::to_string_pretty(&payload).unwrap());
        return ExitCode::SUCCESS;
    }

    println!("{} — {}", scenario.id, scenario.title);
    println!("{}\n", scenario.summary);
    println!(
        "replayed {} of {} divergence event(s)\n",
        current.steps_replayed,
        scenario.events.len()
    );

    for island in [&current.island_a, &current.island_b] {
        let counts = compute_island_digest(island);
        let summary: Vec<String> = counts
            .iter()
            .map(|(status, n)| format!("{n} {status}"))
            .collect();
        println!(
            "{:9} epoch {:4}  {}",
            island.label,
            island.local_epoch,
            summary.join(", ")
        );
    }

    println!("\nmerged view:");
    println!(
        "  {:10} {:12} {:9} {:22} {:5} {:5}",
        "subject", "status", "source", "stability", "epoch", "trust"
    );
    for member in &merge.members {
        println!(
            "  {:10} {:12} {:9} {:22} {:5} {:5}",
            member.subject_label,
            member.status.as_str(),
            member.source.as_str(),
            format!("{:?}", member.stability).to_lowercase(),
            member.epoch,
            member.trust_weight
        );
    }

    println!("\nvisible residue:");
    if merge.residues.is_empty() {
        println!("  (none — every path converged cleanly)");
    }
    for residue in &merge.residues {
        let mark = if residue.handled_by_override {
            " [handled by override]"
        } else {
            ""
        };
        println!("  {}{mark}\n    {}", residue.subject_label, residue.detail);
    }

    println!("\nrepair digest:");
    println!("  outcome: {:?}", merge.digest.overall_outcome);
    for input in &merge.digest.inputs {
        println!("  input:   {input}");
    }
    for rule in &merge.digest.rules_fired {
        println!("  rule:    {rule}");
    }

    let kernel = run_kernel_reunion(&policy, &scenario, &current, apply_override);
    let disagreements = compare(&kernel, &merge);
    if disagreements.is_empty() {
        println!("\nkernel cross-check: the typed merge engine agrees with the lab oracle.");
    } else {
        println!("\nkernel cross-check FAILED:");
        for d in &disagreements {
            println!("  {d}");
        }
        return ExitCode::FAILURE;
    }

    if args.naive {
        let comparison = build_naive_comparison(&scenario, &current, &merge);
        println!("\nnaive reunion (\"latest or loudest wins\"):");
        for member in &comparison.naive {
            println!(
                "  {:10} {:12} from {}",
                member.subject_label,
                member.status.as_str(),
                member.source
            );
        }
        if comparison.diffs.is_empty() {
            println!("  no differences — nothing here needed honesty.");
        } else {
            println!("  differences:");
            for diff in &comparison.diffs {
                println!(
                    "    {}: naive {} vs deterministic {} — {}",
                    diff.subject_label,
                    diff.naive_status.as_str(),
                    diff.deterministic_status.as_str(),
                    diff.note
                );
            }
        }
    }

    ExitCode::SUCCESS
}

fn scenario_verify(args: &CorpusArgs) -> ExitCode {
    let dir = corpus_dir(args);
    let corpus = match load_corpus(&dir) {
        Ok(c) => c,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };
    let policy = MergePolicy::lab_compat();
    let mut failed = 0;

    println!("golden outcome table:");
    for case in golden_cases() {
        let Some(scenario) = corpus.iter().find(|s| s.id == case.scenario_id) else {
            println!("  FAIL {} — missing from corpus", case.scenario_id);
            failed += 1;
            continue;
        };
        let report = check_case(&policy, scenario, &case);
        let label = format!(
            "{}{}",
            report.scenario_id,
            if report.apply_override {
                " (+override)"
            } else {
                ""
            }
        );
        if report.passed() {
            println!("  ok   {label}");
        } else {
            failed += 1;
            println!("  FAIL {label}");
            for failure in &report.failures {
                println!("         {failure}");
            }
        }
    }

    println!("kernel-vs-oracle conformance (every replay prefix):");
    for scenario in &corpus {
        let mut scenario_failures = 0;
        for steps in 0..=scenario.events.len() {
            let current = materialize(scenario, steps);
            let overrides: &[bool] = if scenario.allow_operator_override {
                &[false, true]
            } else {
                &[false]
            };
            for &apply_override in overrides {
                let oracle = run_deterministic_merge(&policy, scenario, &current, apply_override);
                let kernel = run_kernel_reunion(&policy, scenario, &current, apply_override);
                scenario_failures += compare(&kernel, &oracle).len();
            }
        }
        if scenario_failures == 0 {
            println!("  ok   {}", scenario.id);
        } else {
            failed += 1;
            println!(
                "  FAIL {} — {scenario_failures} disagreement(s)",
                scenario.id
            );
        }
    }

    if failed == 0 {
        println!("\nall scenarios conform.");
        ExitCode::SUCCESS
    } else {
        println!("\n{failed} check(s) failed.");
        ExitCode::FAILURE
    }
}

fn rank(args: &RankArgs) -> ExitCode {
    let seed = RankSeed {
        domain: RankDomain::WitnessSelection,
        scope: ScopeId::new(args.scope.clone()),
        subject: None,
        epoch: Epoch(args.epoch),
        round: Round(args.round),
    };
    let pool: Vec<PeerId> = args.candidates.iter().map(PeerId::new).collect();
    let take = args.take.unwrap_or(pool.len());
    let selection = permutation_rank(seed, pool, vec![], take);

    println!(
        "rank seed: domain=witness-selection scope={} epoch={} round={}",
        args.scope, args.epoch, args.round
    );
    println!(
        "selected {} of {}:",
        selection.selected.len(),
        selection.pool.len()
    );
    for (position, (peer, token)) in selection.ranked.iter().enumerate() {
        let mark = if position < take { "*" } else { " " };
        if args.audit {
            println!(
                "  {mark} {position:2}. {peer:12} token {}",
                token.short_hex()
            );
        } else {
            println!("  {mark} {position:2}. {peer}");
        }
    }
    if args.audit {
        match reconstruct(&selection) {
            Ok(()) => println!(
                "reconstruction: ok — any observer holding the seed and pool derives this exact order"
            ),
            Err(divergence) => {
                println!("reconstruction FAILED: {divergence}");
                return ExitCode::FAILURE;
            }
        }
        println!("hint: change --round to watch the order rotate (hotspot damping).");
    }
    ExitCode::SUCCESS
}

fn sim_run(args: &SimRunArgs) -> ExitCode {
    let config = SimConfig {
        seed: args.seed,
        nodes: args.nodes,
        subjects: args.subjects,
        operator_override: args.apply_override,
    };
    let report = run_sim(&config);

    if args.json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
        return ExitCode::SUCCESS;
    }

    println!("resonant sim (seed {}):", config.seed);
    for line in &report.narrative {
        println!("  {line}");
    }
    println!("\n  victim {} -> {}", report.victim, report.victim_state);
    println!(
        "  residue: {} unresolved, {} handled by override",
        report.unresolved_residue, report.handled_residue
    );
    println!("  overall: {:?}", report.overall);
    println!(
        "  islands converged: {} (content hashes {})",
        report.converged,
        if report.converged { "match" } else { "differ" },
    );
    println!(
        "  transcript heads: island A {} / island B {}",
        report.transcript_heads[0], report.transcript_heads[1]
    );
    ExitCode::SUCCESS
}
