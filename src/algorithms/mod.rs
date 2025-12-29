mod least_connections;
mod least_response_time;
mod round_robin;
mod weighted_round_robin;

use rand::RngCore;

use crate::models::{Algorithm, ServerState};

pub use least_connections::LeastConnectionsStrategy;
pub use least_response_time::LeastResponseTimeStrategy;
pub use round_robin::RoundRobinStrategy;
pub use weighted_round_robin::WeightedRoundRobinStrategy;

pub trait SelectionStrategy {
    fn select(&mut self, ctx: &SelectionContext) -> Selection;
}

pub struct SelectionContext<'a> {
    pub servers: &'a [ServerState],
    #[allow(dead_code)]
    pub time_ms: u64,
    pub rng: &'a mut dyn RngCore,
}

pub struct Selection {
    pub server_id: usize,
    pub score: Option<u64>,
}

pub fn build_strategy(algo: Algorithm) -> Box<dyn SelectionStrategy> {
    match algo {
        Algorithm::RoundRobin => Box::new(RoundRobinStrategy::default()),
        Algorithm::WeightedRoundRobin => Box::new(WeightedRoundRobinStrategy::default()),
        Algorithm::LeastConnections => Box::new(LeastConnectionsStrategy),
        Algorithm::LeastResponseTime => Box::new(LeastResponseTimeStrategy),
    }
}
