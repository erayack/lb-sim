mod algorithms;
mod cli;
mod engine;
mod events;
mod models;

use crate::algorithms::build_strategy;
use crate::engine::SimulationEngine;
use crate::models::{Algorithm, SimConfig, SimError, SimResult, SimulationResult, TieBreak};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> SimResult<()> {
    let args = cli::parse_args()?;
    let servers = cli::parse_servers(&args.servers)?;
    if args.requests == 0 {
        return Err(SimError::RequestsZero);
    }

    let algo: Algorithm = args.algo.clone().into();
    let tie_break = match args.seed {
        Some(seed) => TieBreak::Seeded(seed),
        None => TieBreak::Stable,
    };
    let config = SimConfig {
        servers,
        requests: args.requests,
        tie_break,
    };
    let strategy = build_strategy(algo);
    let mut engine = SimulationEngine::new(config, strategy);
    let result = engine.run()?;

    if args.summary {
        print_summary(&result);
        return Ok(());
    }

    println!("Tie-break: {}", result.tie_break);

    for assignment in &result.assignments {
        if let Some(score) = assignment.score {
            println!(
                "Request {} -> {} (score: {}ms)",
                assignment.request_id, assignment.server_name, score
            );
        } else {
            println!(
                "Request {} -> {}",
                assignment.request_id, assignment.server_name
            );
        }
    }

    print_summary(&result);

    Ok(())
}

fn print_summary(result: &SimulationResult) {
    println!("Summary:");
    for summary in &result.totals {
        println!(
            "{}: {} requests (avg response: {}ms)",
            summary.name, summary.requests, summary.avg_response_ms
        );
    }
}
