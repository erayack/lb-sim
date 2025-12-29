use load_balancer_cli::config::{self, Command};
use load_balancer_cli::engine;
use load_balancer_cli::error::SimResult;
use load_balancer_cli::output;

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
    match config::parse_cli()?.command {
        Command::Run(args) => {
            let config = config::build_config(&args.sim)?;
            let result = engine::run_simulation(&config)?;

            if args.summary {
                output::print_summary(&result);
                return Ok(());
            }

            output::print_full(&config, &result)?;
            Ok(())
        }
        Command::ListAlgorithms => {
            for name in ALGORITHMS {
                println!("{}", name);
            }
            Ok(())
        }
        Command::ShowConfig(args) => {
            let config = config::build_config(&args)?;
            output::print_config(&config);
            Ok(())
        }
    }
}
