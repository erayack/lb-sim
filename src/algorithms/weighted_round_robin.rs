use crate::algorithms::{Selection, SelectionContext, SelectionStrategy};

#[derive(Default)]
pub struct WeightedRoundRobinStrategy {
    cursor: u64,
}

impl SelectionStrategy for WeightedRoundRobinStrategy {
    fn select(&mut self, ctx: &SelectionContext) -> Selection {
        let total_weight: u64 = ctx.servers.iter().map(|server| server.weight as u64).sum();
        let target = self.cursor % total_weight;
        self.cursor = (self.cursor + 1) % total_weight;

        let mut cursor = 0u64;
        let mut selected = 0usize;
        for (idx, server) in ctx.servers.iter().enumerate() {
            cursor += server.weight as u64;
            if target < cursor {
                selected = idx;
                break;
            }
        }

        Selection {
            server_id: selected,
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
    fn weighted_round_robin_respects_weights() {
        let servers = vec![
            Server::test_at(0, "a", 10, 2, 0, 0),
            Server::test_at(1, "b", 10, 1, 0, 0),
        ];
        let mut rng = rand::rngs::StdRng::seed_from_u64(1);
        let mut strategy = WeightedRoundRobinStrategy::default();
        let ctx = SelectionContext {
            servers: &servers,
            time_ms: 0,
            rng: &mut rng,
        };

        let picks: Vec<usize> = (0..6).map(|_| strategy.select(&ctx).server_id).collect();
        assert_eq!(picks, vec![0, 0, 1, 0, 0, 1]);
    }
}
