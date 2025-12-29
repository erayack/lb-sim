# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build, Test, and Lint Commands

```bash
cargo build                # Build the project
cargo run -- run --algo round-robin --servers a:10,b:20 --requests 5  # Run simulation
cargo run -- list-algorithms                                           # List algorithms
cargo run -- show-config --algo round-robin --servers a:10,b:20       # Show config
cargo test                 # Run all tests
cargo nextest run          # Run all tests (preferred over cargo test)
cargo nextest run <test>   # Run a specific test
cargo fmt                  # Format code
cargo clippy               # Run linter
just                       # Show available just recipes
```

## Architecture

A Rust CLI that simulates load-balancing algorithms across servers with configurable latencies and weights. Features an event-driven simulation engine and pluggable algorithm architecture.

### Module Structure

- **`src/bin/load-balancer.rs`** - CLI entry point. Handles subcommand routing and output formatting.
- **`src/config.rs`** - Defines clap arguments (`CliArgs`, `Command`, `RunArgs`, `AlgoArg`) and parses server specs (`name:latency_ms[:weight]`). Supports TOML/JSON config files.
- **`src/engine.rs`** - Event-driven simulation engine. Uses `BinaryHeap<Reverse<ScheduledEvent>>` for time-based event processing.
- **`src/algorithms/`** - Pluggable algorithm architecture
  - **`mod.rs`** - Defines `SelectionStrategy` trait and `SelectionContext`. Contains `build_strategy()` factory.
  - **`round_robin.rs`** - Sequential server cycling
  - **`weighted_round_robin.rs`** - Weight-proportional distribution
  - **`least_connections.rs`** - Fewest active connections (time-based decay via in-flight tracking)
  - **`least_response_time.rs`** - Lowest `base_latency_ms + (pick_count * 10)` score
- **`src/models.rs`** - Core types: `ServerConfig`, `SimConfig`, `AlgoConfig`, `RequestProfile`, `TieBreakConfig`. Uses serde for TOML/JSON serialization.
- **`src/events.rs`** - Event types for simulation: `Event`, `Request`, `ScheduledEvent`. Implements `Ord` for event prioritization.
- **`src/state.rs`** - Runtime state: `ServerState`, `EngineState`, `SimulationResult`, `Assignment`, `ServerSummary`.
- **`src/output.rs`** - Output formatters: `Formatter` trait, `HumanFormatter`, `JsonFormatter`, `SummaryFormatter`.
- **`src/error.rs`** - Error types: `Error` enum with `Result<T>` alias.
- **`src/lib.rs`** - Library root, exports public modules.

### Algorithms

1. **RoundRobin** - Cycles through servers sequentially.
2. **WeightedRoundRobin** - Distributes proportionally to weight values.
3. **LeastConnections** - Picks server with fewest `active_connections`. Uses `BinaryHeap<Reverse<InFlight>>` for time-based decay (requests complete after `base_latency_ms`).
4. **LeastResponseTime** - Picks server with lowest `base_latency_ms + (pick_count * 10)` score.

### Tie-Breaking

- **Stable** - Uses input order for ties (default, no seed).
- **Seeded** - Uses `StdRng` with provided seed for deterministic random selection.

### Least-Connections Semantics

Requests arrive one time unit apart. A request stays "in flight" for `base_latency_ms`. Active connections decay when in-flight requests complete (time-based, not count-based).

### Event-Driven Engine

The simulation engine uses a priority queue of scheduled events:

1. All requests are pre-scheduled as `RequestArrival` events
2. When a request arrives, a `RequestComplete` event is scheduled for `arrival + latency`
3. The event loop processes events in time order
4. `RequestComplete` events decrement active/in-flight counts

### Request Profiles

- `FixedCount(n)` - Simulate exactly n requests arriving one time unit apart
- `Poisson { rate, duration_ms }` - Random arrivals following Poisson process

### Output Formats

- `human` - Human-readable with tie-break header and per-request assignments
- `summary` - Compact, test-friendly format showing only server totals
- `json` - Structured JSON for programmatic consumption

### CLI Subcommands

- `run` - Execute a simulation
- `list-algorithms` - Print all available algorithm names (one per line)
- `show-config` - Display effective configuration from args/config file
