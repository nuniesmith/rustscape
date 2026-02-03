#!/bin/bash
# run.sh - Start the server in development mode

set -e

# Create data directories if they don't exist
mkdir -p data/players

# Set log level (debug, info, warn, error)
export RUST_LOG="${RUST_LOG:-rustscape=debug}"

# Run the server
cargo run

