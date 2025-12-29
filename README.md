# Load Balancer CLI

A Rust CLI that simulates load-balancing algorithms across servers with configurable latencies and weights. Features an event-driven simulation engine and pluggable algorithm architecture.

## Quick Start

Build:

```
cargo build
```

Run simulations using subcommands:

```
# Run a simulation with round-robin
cargo run -- run --algo round-robin --servers a:10,b:20 --requests 5

# Run with least-connections and seeded tie-breaking
cargo run -- run --algo least-connections --servers a:10,b:20 --requests 5 --seed 42

# Run with weighted round-robin
cargo run -- run --algo weighted-round-robin --server a:10:2 --server b:20:1 --requests 5

# List available algorithms
cargo run -- list-algorithms

# Show effective configuration
cargo run -- show-config --algo round-robin --servers a:10,b:20 --requests 5
```

## CLI Commands

The CLI uses subcommands for different operations:

- `run` - Run a load balancer simulation
- `list-algorithms` - List all available algorithms
- `show-config` - Show the effective configuration for given arguments

### Global Options (when no subcommand)

You can also run without a subcommand for backwards compatibility:

```
cargo run -- --algo round-robin --servers a:10,b:20 --requests 5
```

### Run Command Options

| Option | Description |
|--------|-------------|
| `--algo` | Algorithm to use (required) |
| `--servers` | Comma-separated server list (e.g., `a:10,b:20`) |
| `--server` | Add a single server (can be repeated, format: `name:latency[:weight]`) |
| `--requests` | Number of requests to simulate (required) |
| `--format` | Output format: `human`, `summary`, or `json` (default: `human`) |
| `--summary` | Short for `--format summary` |
| `--seed` | Seed for deterministic tie-breaking |
| `--config` | Path to config file (TOML or JSON) |

### Server Format

Servers can be specified as:

- `name:latency` - Simple server with latency
- `name:latency:weight` - Server with latency and weight (for weighted algorithms)

Example: `api:25,db:40:2`

### Output Formats

- `human` - Human-readable detailed output with per-request assignments
- `summary` - Compact summary showing only server totals
- `json` - JSON format for programmatic use

## Algorithms

1. **round-robin** - Cycles through servers sequentially
2. **weighted-round-robin** - Distributes requests proportionally to weight values
3. **least-connections** - Picks server with fewest active connections (time-based decay)
4. **least-response-time** - Picks server with lowest latency + (pick count * 10)

## Tie-Breaking

- **stable** - Uses input order for ties (default, no seed)
- **seeded** - Uses `StdRng` with provided seed for deterministic random selection

## Least-Connections Semantics

- Requests arrive one time unit apart
- A request stays "in flight" for `base_latency_ms`
- Active connections decay when in-flight requests complete (time-based, not count-based)

## Configuration Files

You can also provide configuration via TOML or JSON files:

```toml
# config.toml
algorithm = "round-robin"
requests = 10

[[servers]]
name = "api"
latency_ms = 10
weight = 1
```

```json
// config.json
{
  "algorithm": "least-connections",
  "requests": 5,
  "servers": [
    { "name": "api", "latency_ms": 10, "weight": 1 },
    { "name": "db", "latency_ms": 20, "weight": 2 }
  ]
}
```

Run with config file:

```
cargo run -- run --config config.toml --requests 15
```

CLI options override config file settings.

## Project Layout

```
src/
├── algorithms/          # Pluggable algorithm implementations
│   ├── mod.rs          # SelectionStrategy trait and factory
│   ├── round_robin.rs
│   ├── weighted_round_robin.rs
│   ├── least_connections.rs
│   └── least_response_time.rs
├── bin/
│   └── load-balancer.rs # CLI entry point
├── config.rs            # CLI parsing and config building
├── engine.rs            # Event-driven simulation engine
├── events.rs            # Event types for simulation
├── lib.rs               # Library root (exports public API)
├── models.rs            # Core types (SimConfig, ServerConfig, etc.)
├── output.rs            # Output formatters
├── state.rs             # Runtime state tracking
└── error.rs             # Error types
```

## Development

Using Cargo:

```
cargo build              # Build the project
cargo run -- run --algo round-robin --servers a:10,b:20 --requests 5
cargo test               # Run tests
cargo fmt                # Format code
cargo clippy             # Run linter
```

Using Just (recommended):

```
just                    # Show available commands
just build              # Build the project
just run-example        # Run example simulation
just test               # Run all tests
just nextest            # Run tests with nextest
just fmt                # Format code
just clippy             # Run linter
just check              # Run all checks (fmt, clippy, test, nextest)
```

## Testing

Run all tests:

```
cargo test
cargo nextest run
```

Run specific test files:

```
cargo nextest run config_cli
cargo nextest run engine
```

Run a single test by name pattern:

```
cargo nextest run least_connections_prefers_lowest_pick_count
cargo nextest run --filter-expr 'weighted'
```

Test a specific algorithm with the CLI:

```
# Test least-connections with deterministic seed
cargo run -- run --algo least-connections --servers a:10,b:20,c:15 --requests 10 --seed 42 --summary

# Test weighted round-robin with custom server weights
cargo run -- run --algo weighted-round-robin --server a:10:2 --server b:20:3 --server c:15:1 --requests 10 --summary

# Test with JSON output for validation
cargo run -- run --algo round-robin --servers a:5,b:10,c:15 --requests 5 --format json
```

Run tests with verbose output:

```
cargo test -- --nocapture
cargo nextest run -- --nocapture
```
