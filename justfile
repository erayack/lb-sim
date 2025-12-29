# Load Balancer CLI - Just commands

# Run clippy linter
clippy:
    cargo clippy --all-targets --all-features

# Run clippy with warnings as errors
clippy-strict:
    cargo clippy --all-targets --all-features -- -D warnings

# Format code with rustfmt
fmt:
    cargo fmt --all

# Check formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Run tests with cargo test
test:
    cargo test

# Run tests with nextest (preferred)
nextest:
    cargo nextest run

# Run a specific test with nextest
nextest-test TEST:
    cargo nextest run {{TEST}}

# Run all checks (fmt, clippy, test, nextest)
check: fmt clippy test nextest

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run the CLI with example arguments
run-example:
    cargo run -- --algo round-robin --servers a:10,b:20 --requests 5

# Clean build artifacts
clean:
    cargo clean

# Show available recipes
help:
    @just --list

