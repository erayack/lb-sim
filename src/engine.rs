use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use crate::algorithms::{SelectionContext, SelectionStrategy};
use crate::events::{Event, Request, ScheduledEvent};
use crate::models::{
    Assignment, EngineState, ServerSummary, SimConfig, SimError, SimResult, SimulationResult,
    TieBreak,
};

pub struct SimulationEngine {
    pub config: SimConfig,
    pub state: EngineState,
    pub strategy: Box<dyn SelectionStrategy>,
    pub rng: StdRng,
}

impl SimulationEngine {
    pub fn new(config: SimConfig, strategy: Box<dyn SelectionStrategy>) -> Self {
        let seed = match config.tie_break {
            TieBreak::Seeded(seed) => seed,
            TieBreak::Stable => 0,
        };
        let rng = StdRng::seed_from_u64(seed);
        let state = EngineState {
            time_ms: 0,
            servers: Vec::new(),
            assignments: Vec::new(),
        };

        Self {
            config,
            state,
            strategy,
            rng,
        }
    }

    pub fn run(&mut self) -> SimResult<SimulationResult> {
        if self.config.servers.is_empty() {
            return Err(SimError::EmptyServers);
        }
        if self.config.requests == 0 {
            return Err(SimError::RequestsZero);
        }

        let mut id_to_index = HashMap::new();
        for (idx, server) in self.config.servers.iter().enumerate() {
            if id_to_index.insert(server.id, idx).is_some() {
                return Err(SimError::DuplicateServerId(server.id));
            }
        }

        self.state.servers = self.config.servers.clone();
        for server in &mut self.state.servers {
            server.active_connections = 0;
            server.pick_count = 0;
        }
        self.state.assignments = Vec::with_capacity(self.config.requests);

        let mut events: BinaryHeap<Reverse<ScheduledEvent>> = BinaryHeap::new();
        for (offset, request_id) in (1..=self.config.requests).enumerate() {
            let arrival_time_ms = offset as u64;
            let request = Request {
                id: request_id,
                arrival_time_ms,
            };
            events.push(Reverse(ScheduledEvent::new(
                arrival_time_ms,
                Event::RequestArrival(request),
            )));
        }

        let mut stable_rng = StableRng;

        while let Some(Reverse(scheduled)) = events.pop() {
            self.state.time_ms = scheduled.time_ms;
            match scheduled.event {
                Event::RequestComplete { server_id, .. } => {
                    if let Some(server) = self.state.servers.get_mut(server_id) {
                        server.active_connections = server.active_connections.saturating_sub(1);
                    }
                }
                Event::RequestArrival(request) => {
                    let rng: &mut dyn RngCore = match self.config.tie_break {
                        TieBreak::Stable => &mut stable_rng,
                        TieBreak::Seeded(_) => &mut self.rng,
                    };
                    let ctx = SelectionContext {
                        servers: &self.state.servers,
                        time_ms: self.state.time_ms,
                        rng,
                    };
                    let selection = self.strategy.select(&ctx);
                    let server_idx = selection.server_id;

                    let server = &mut self.state.servers[server_idx];
                    server.active_connections += 1;
                    server.pick_count += 1;

                    let started_at = self.state.time_ms;
                    let completed_at = started_at + server.base_latency_ms;
                    events.push(Reverse(ScheduledEvent::new(
                        completed_at,
                        Event::RequestComplete {
                            server_id: server_idx,
                            request_id: request.id,
                        },
                    )));

                    self.state.assignments.push(Assignment {
                        request_id: request.id,
                        server_id: server.id,
                        server_name: server.name.clone(),
                        score: selection.score,
                        started_at,
                        completed_at,
                    });
                }
            }
        }

        let mut counts = vec![0u32; self.state.servers.len()];
        let mut total_response_ms = vec![0u64; self.state.servers.len()];
        for assignment in &self.state.assignments {
            let idx = id_to_index[&assignment.server_id];
            counts[idx] += 1;
            total_response_ms[idx] += assignment.completed_at - assignment.started_at;
        }

        let totals = self
            .state
            .servers
            .iter()
            .enumerate()
            .map(|(idx, server)| {
                let count = counts[idx];
                let avg_response_ms = if count == 0 {
                    0
                } else {
                    total_response_ms[idx] / count as u64
                };
                ServerSummary {
                    name: server.name.clone(),
                    requests: count,
                    avg_response_ms,
                }
            })
            .collect();

        Ok(SimulationResult {
            assignments: self.state.assignments.clone(),
            totals,
            tie_break: self.config.tie_break.clone(),
        })
    }
}

struct StableRng;

impl RngCore for StableRng {
    fn next_u32(&mut self) -> u32 {
        0
    }

    fn next_u64(&mut self) -> u64 {
        0
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            *byte = 0;
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::{build_strategy, LeastConnectionsStrategy};
    use crate::models::{Algorithm, Server};

    #[test]
    fn least_connections_accounts_for_completed_requests() {
        let servers = vec![
            Server::test_at(0, "fast", 1, 1, 0, 0),
            Server::test_at(1, "slow", 100, 1, 0, 0),
        ];
        let config = SimConfig {
            servers,
            requests: 2,
            tie_break: TieBreak::Stable,
        };
        let mut engine = SimulationEngine::new(config, Box::new(LeastConnectionsStrategy));
        let result = engine.run().expect("simulation should succeed");
        let assigned = result
            .assignments
            .iter()
            .map(|assignment| assignment.server_name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(assigned, vec!["fast", "fast"]);
    }

    #[test]
    fn seeded_tiebreak_is_deterministic_in_engine() {
        let servers = vec![
            Server::test_at(0, "a", 1, 1, 0, 0),
            Server::test_at(1, "b", 1, 1, 0, 0),
            Server::test_at(2, "c", 1, 1, 0, 0),
        ];
        let config = SimConfig {
            servers: servers.clone(),
            requests: 3,
            tie_break: TieBreak::Seeded(42),
        };
        let mut engine_a =
            SimulationEngine::new(config, build_strategy(Algorithm::LeastConnections));
        let result_a = engine_a.run().expect("simulation should succeed");

        let config = SimConfig {
            servers,
            requests: 3,
            tie_break: TieBreak::Seeded(42),
        };
        let mut engine_b =
            SimulationEngine::new(config, build_strategy(Algorithm::LeastConnections));
        let result_b = engine_b.run().expect("simulation should succeed");

        let actual = result_a
            .assignments
            .iter()
            .map(|assignment| assignment.server_name.as_str())
            .collect::<Vec<_>>();
        let expected = result_b
            .assignments
            .iter()
            .map(|assignment| assignment.server_name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);
    }
}
