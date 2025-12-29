use crate::error::{SimError, SimResult};
use crate::models::{AlgoConfig, RequestProfile, SimConfig, TieBreakConfig};
use crate::state::SimulationResult;

pub fn print_summary(result: &SimulationResult) {
    println!("Summary:");
    for summary in &result.totals {
        println!(
            "{}: {} requests (avg response: {}ms)",
            summary.name, summary.requests, summary.avg_response_ms
        );
    }
}

pub fn print_full(config: &SimConfig, result: &SimulationResult) -> SimResult<()> {
    match result.tie_break {
        TieBreakConfig::Stable => println!("Tie-break: stable"),
        TieBreakConfig::Seeded => {
            let seed = result.seed.unwrap_or_default();
            println!("Tie-break: seeded({})", seed);
        }
    }

    for assignment in &result.assignments {
        let server_name = config
            .servers
            .get(assignment.server_id)
            .map(|server| server.name.as_str())
            .ok_or_else(|| {
                SimError::InvalidServerEntry(format!(
                    "missing server for id {}",
                    assignment.server_id
                ))
            })?;
        if let Some(score) = assignment.score {
            println!(
                "Request {} -> {} (score: {}ms)",
                assignment.request_id, server_name, score
            );
        } else {
            println!("Request {} -> {}", assignment.request_id, server_name);
        }
    }

    print_summary(result);

    Ok(())
}

pub fn print_config(config: &SimConfig) {
    println!("Algorithm: {}", algo_name(&config.algo));
    println!("Requests: {}", format_requests(&config.requests));
    println!("Tie-break: {}", format_tie_break(&config.tie_break, config.seed));
    println!("Servers:");
    for server in &config.servers {
        println!(
            "- {} (latency: {}ms, weight: {})",
            server.name, server.base_latency_ms, server.weight
        );
    }
}

fn algo_name(algo: &AlgoConfig) -> &'static str {
    match algo {
        AlgoConfig::RoundRobin => "round-robin",
        AlgoConfig::WeightedRoundRobin => "weighted-round-robin",
        AlgoConfig::LeastConnections => "least-connections",
        AlgoConfig::LeastResponseTime => "least-response-time",
    }
}

fn format_requests(profile: &RequestProfile) -> String {
    match profile {
        RequestProfile::FixedCount(count) => count.to_string(),
        RequestProfile::Poisson { rate, duration_ms } => {
            format!("poisson(rate={}, duration_ms={})", rate, duration_ms)
        }
    }
}

fn format_tie_break(tie_break: &TieBreakConfig, seed: Option<u64>) -> String {
    match tie_break {
        TieBreakConfig::Stable => "stable".to_string(),
        TieBreakConfig::Seeded => format!("seeded({})", seed.unwrap_or_default()),
    }
}
