#!/bin/bash

# PetAddr Server Start Script

echo "🚀 Starting PetAddr Server..."

# Set environment variables
export RUST_LOG=info
export RUST_ENV=development

# Check configuration file
if [ ! -f "config.toml" ]; then
    echo "⚠️  Warning: config.toml file not found, using default configuration"
fi

# Run the project
cargo run

echo "👋 Server stopped"