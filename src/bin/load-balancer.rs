use lb_sim::config::{self, format_config, Command, FormatArg, RunArgs};
use lb_sim::engine;
use lb_sim::error::Result;
use lb_sim::output::formatter_from_format;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let command = config::parse_command()?;

    match command {
        Command::Run(run_args) => run_simulation(run_args),
        Command::ListAlgorithms => list_algorithms(),
        Command::ShowConfig(run_args) => show_config(run_args),
    }
}

fn run_simulation(run_args: RunArgs) -> Result<()> {
    let (config, format_for_engine) = config::build_config_from_run_args(run_args)?;
    let result = if matches!(format_for_engine, FormatArg::Summary) {
        engine::run_simulation_summary(&config)?
    } else {
        engine::run_simulation(&config)?
    };
    let formatter = formatter_from_format(&format_for_engine);
    let output = formatter.write(&result);
    print!("{}", output);

    Ok(())
}

fn list_algorithms() -> Result<()> {
    println!("round-robin");
    println!("weighted-round-robin");
    println!("least-connections");
    println!("least-response-time");
    Ok(())
}

fn show_config(run_args: RunArgs) -> Result<()> {
    let (config, _) = config::build_config_from_run_args(run_args)?;
    let output = format_config(&config);
    print!("{}", output);
    Ok(())
}
