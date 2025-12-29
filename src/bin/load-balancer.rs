use load_balancer_cli::cli::{self, FormatArg};
use load_balancer_cli::error::{Error, Result};
use load_balancer_cli::models::{Algorithm, TieBreak};
use load_balancer_cli::output::{Formatter, HumanFormatter, JsonFormatter, SummaryFormatter};
use load_balancer_cli::sim;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = cli::parse_args()?;
    let servers = cli::parse_servers(&args.servers)?;
    if args.requests == 0 {
        return Err(Error::RequestsZero);
    }

    let algo: Algorithm = args.algo.clone().into();
    let tie_break = match args.seed {
        Some(seed) => TieBreak::Seeded(seed),
        None => TieBreak::Stable,
    };
    let result = sim::run_simulation(servers, algo, args.requests, tie_break)?;

    let formatter = formatter_for(&args);
    let output = formatter.write(&result);
    print!("{}", output);

    Ok(())
}

fn formatter_for(args: &cli::Args) -> Box<dyn Formatter> {
    let format = if args.summary {
        FormatArg::Summary
    } else {
        args.format.clone()
    };

    match format {
        FormatArg::Human => Box::new(HumanFormatter),
        FormatArg::Summary => Box::new(SummaryFormatter),
        FormatArg::Json => Box::new(JsonFormatter),
    }
}
