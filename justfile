#!/usr/bin/env -S just --justfile

alias b := build
alias d := dev
alias r := run
alias f := fmt
alias l := lint
alias t := test

# List available commands.
_default:
    just --list --unsorted

# Develop the app.
dev:
    cargo watch -x 'clippy --locked --all-targets --all-features'

# Build only
build:
    cargo build

# Develop the app.
run: build
    ./target/debug/js8wait -f ./tests/media/sample_441K_mono_fp_32.wav

# Format the codebase.
fmt:
    cargo fmt --all

# Check if the codebase is properly formatted.
fmt-check:
    cargo fmt --all -- --check

# Lint the codebase.
lint:
    cargo clippy --locked --all-targets --all-features

# Test the codebase.
test: build
    cargo test run --all-targets

# Tasks to make the code base comply with the rules. Mostly used in git hooks.
comply: fmt lint test

# Check if the repository complies with the rules and is ready to be pushed.
check: fmt-check lint test
