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

# Check .env file
if [ ! -f ".env" ]; then
    echo "💡 Info: .env file not found, create .env file to customize environment variables"
    echo "💡 You can copy from .env.example: cp .env.example .env"
fi

# Run the project
cargo run

echo "👋 Server stopped"