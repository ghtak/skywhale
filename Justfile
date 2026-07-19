# Safe, repeatable project commands for humans, CI, and AI agents.
# Run `just --list` to see the available recipes.

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

default:
  @just --list

# Run every non-mutating source-quality check.
check: format-check lint test

# Verify Rust formatting without rewriting source files.
format-check:
  cargo fmt --manifest-path skywhale/Cargo.toml -- --check

# Run lints and treat warnings as failures.
lint:
  cargo clippy --manifest-path skywhale/Cargo.toml --all-targets -- -D warnings

# Run the workspace test suite.
test:
  cargo test --manifest-path skywhale/Cargo.toml

# Compile the workspace without running it.
build:
  cargo build --manifest-path skywhale/Cargo.toml
