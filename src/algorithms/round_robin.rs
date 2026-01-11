use crate::algorithms::{Selection, SelectionContext, SelectionStrategy};
use crate::state::ServerId;

#[derive(Default)]
pub struct RoundRobinStrategy {
    next_idx: usize,
}

impl SelectionStrategy for RoundRobinStrategy {
    fn select(&mut self, ctx: &mut SelectionContext) -> Selection {
        let idx = self.next_idx % ctx.servers.len();
        self.next_idx = (self.next_idx + 1) % ctx.servers.len();
        Selection {
            server_id: ServerId::from(idx),
            score: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ServerId, ServerState};
    use rand::SeedableRng;

    #[test]
    fn round_robin_cycles_indices() {
        let servers = vec![
            ServerState {
                id: ServerId::from(0),
                name: "a".to_string(),
                base_latency_ms: 10,
                weight: 1,
                active_connections: 0,
                pick_count: 0,
                in_flight: 0,
                next_available_ms: 0,
            },
            ServerState {
                id: ServerId::from(1),
                name: "b".to_string(),
                base_latency_ms: 10,
                weight: 1,
                active_connections: 0,
                pick_count: 0,
                in_flight: 0,
                next_available_ms: 0,
            },
            ServerState {
                id: ServerId::from(2),
                name: "c".to_string(),
                base_latency_ms: 10,
                weight: 1,
                active_connections: 0,
                pick_count: 0,
                in_flight: 0,
                next_available_ms: 0,
            },
        ];
        let mut rng = rand::rngs::StdRng::seed_from_u64(1);
        let mut strategy = RoundRobinStrategy::default();
        let mut ctx = SelectionContext {
            servers: &servers,
            time_ms: 0,
            rng: &mut rng,
        };

        assert_eq!(strategy.select(&mut ctx).server_id, ServerId::from(0));
        assert_eq!(strategy.select(&mut ctx).server_id, ServerId::from(1));
        assert_eq!(strategy.select(&mut ctx).server_id, ServerId::from(2));
        assert_eq!(strategy.select(&mut ctx).server_id, ServerId::from(0));
    }
}
