use crate::algorithms::{Selection, SelectionContext, SelectionStrategy};

#[derive(Default)]
pub struct RoundRobinStrategy {
    next_idx: usize,
}

impl SelectionStrategy for RoundRobinStrategy {
    fn select(&mut self, ctx: &SelectionContext) -> Selection {
        let idx = self.next_idx % ctx.servers.len();
        self.next_idx = (self.next_idx + 1) % ctx.servers.len();
        Selection {
            server_id: idx,
            score: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Server;
    use rand::SeedableRng;

    #[test]
    fn round_robin_cycles_indices() {
        let servers = vec![
            Server::test_at(0, "a", 10, 1, 0, 0),
            Server::test_at(1, "b", 10, 1, 0, 0),
            Server::test_at(2, "c", 10, 1, 0, 0),
        ];
        let mut rng = rand::rngs::StdRng::seed_from_u64(1);
        let mut strategy = RoundRobinStrategy::default();
        let ctx = SelectionContext {
            servers: &servers,
            time_ms: 0,
            rng: &mut rng,
        };

        assert_eq!(strategy.select(&ctx).server_id, 0);
        assert_eq!(strategy.select(&ctx).server_id, 1);
        assert_eq!(strategy.select(&ctx).server_id, 2);
        assert_eq!(strategy.select(&ctx).server_id, 0);
    }
}
