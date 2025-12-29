# Repository Guidelines

## Project Structure & Module Organization

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

## Build, Test, and Development Commands

- `cargo build` builds the CLI binary.
- `cargo run -- run --algo round-robin --servers a:10,b:20 --requests 5` runs a sample simulation.
- `cargo run -- list-algorithms` lists all available algorithms.
- `cargo run -- show-config --algo round-robin --servers a:10,b:20 --requests 5` displays effective configuration.
- `cargo test` runs unit/integration tests.
- `cargo fmt` formats Rust code using rustfmt.
- `cargo clippy` runs lints; fix or justify warnings before PRs.

## Tooling

- `cargo nextest run` is the preferred test runner (config in `.config/nextest.toml`).
- `cargo xtest` is a local alias for `cargo nextest run`.

## Coding Style & Naming Conventions

- Rust 2021 edition with standard 4-space indentation.
- Types use `UpperCamelCase`; functions and variables use `snake_case`.
- Keep parsing and CLI concerns in `src/config.rs`, algorithm logic in `src/algorithms/`, and simulation in `src/engine.rs`.
- Prefer small, pure functions for selection logic (e.g., `pick_*` helpers in algorithm modules).

## Testing Guidelines

- Unit tests live in-module with `#[cfg(test)]` (e.g., selection helpers in algorithm files).
- Integration tests live under `tests/` and use `assert_cmd` + `predicates` with `--summary` for stable output.
- Use deterministic `--seed` values in CLI tests to keep tie-breaks reproducible.
- Use descriptive test names like `least_connections_prefers_lowest_pick_count`.

## Commit & Pull Request Guidelines

- No commit history is available in this repository; use concise, imperative messages
  (e.g., "Add least-response-time scoring").
- Run `cargo fmt` before opening a PR to keep formatting consistent.
- PRs should include a short description, how to run the change, and sample output
  when user-visible behavior changes.

## CLI & Configuration Notes

- Supports subcommands: `run`, `list-algorithms`, `show-config`.
- `--servers` accepts comma-separated `name:latency_ms[:weight]` entries, e.g. `api:25,db:40`.
- `--server` can be used repeatedly for individual servers with weight support.
- `--seed` makes tie-breaks deterministic for least-connections/response-time.
- `--summary` or `--format summary` prints compact output for testing.
- `--config` accepts TOML or JSON configuration files (CLI options override file settings).
- Duplicate server IDs are rejected by the simulator.
