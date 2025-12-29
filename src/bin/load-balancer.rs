use load_balancer_cli::cli::{self, Command};
use load_balancer_cli::models::{AlgoConfig, SimConfig, SimResult, SimulationResult};
use load_balancer_cli::sim;

const ALGORITHMS: [&str; 4] = [
    "round-robin",
    "weighted-round-robin",
    "least-connections",
    "least-response-time",
];

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> SimResult<()> {
    match cli::parse_cli()?.command {
        Command::Run(args) => {
            let (sim_config, algo_config) = cli::build_configs(&args.sim)?;
            let result = sim::run_simulation(
                sim_config.servers,
                algo_config.algorithm,
                sim_config.requests,
                sim_config.tie_break,
            )?;

            if args.summary {
                print_summary(&result);
                return Ok(());
            }

            print_assignments(&result);
            print_summary(&result);
            Ok(())
        }
        Command::ListAlgorithms => {
            for name in ALGORITHMS {
                println!("{}", name);
            }
            Ok(())
        }
        Command::ShowConfig(args) => {
            let (sim_config, algo_config) = cli::build_configs(&args)?;
            print_config(&sim_config, &algo_config);
            Ok(())
        }
    }
}

fn print_assignments(result: &SimulationResult) {
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

fn print_config(sim_config: &SimConfig, algo_config: &AlgoConfig) {
    println!("Algorithm: {}", algo_config.algorithm);
    println!("Requests: {}", sim_config.requests);
    println!("Tie-break: {}", sim_config.tie_break);
    println!("Servers:");
    for server in &sim_config.servers {
        println!(
            "- {} (latency: {}ms, weight: {})",
            server.name, server.base_latency_ms, server.weight
        );
    }
}
