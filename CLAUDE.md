# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PetAddr is a modern Rust web server project based on the Axum framework, designed with layered architecture, providing time service APIs and complete Swagger documentation.

**Important**: All code comments, documentation, and user-facing text in this project are written in English.

## Quick Start

### Simplest startup method
```bash
# Use startup script (recommended)
./run.sh

# Or run directly
cargo run
```

### Service Access URLs
- **Main Service**: http://localhost:5057
- **API Documentation**: http://localhost:5057/swagger-ui
- **Health Check**: http://localhost:5057/health
- **Time API**: http://localhost:5057/api/v1/time

## Project Architecture

### Layered Structure
```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library file, application creation and configuration
├── config/              # Configuration management
│   └── mod.rs          # Configuration structure and loading logic
├── handlers/            # Request handlers
│   ├── mod.rs
│   ├── health.rs       # Health check handlers
│   └── time.rs         # Time service handlers
├── middleware/          # Middleware
│   ├── mod.rs
│   ├── cors.rs         # CORS handling
│   └── logging.rs      # Logging middleware
├── models/              # Data models
│   ├── mod.rs
│   ├── response.rs     # Response models
│   └── time.rs         # Time-related models
├── routes/              # Route definitions
│   └── mod.rs          # Route organization and configuration
└── utils/               # Utility modules
    ├── mod.rs
    ├── env.rs          # Environment variable handling
    └── validation.rs   # Data validation
```

### Configuration Files
- **config.toml**: Main configuration file (port, API paths, etc.)
- **.env**: Environment variable configuration (optional)
- **.env.example**: Environment variable examples

## Development Commands

### Build and Run
```bash
# Run in development mode
cargo run

# Build and run in release mode
cargo build --release
cargo run --release

# Use startup script
./run.sh
```

### Code Quality
```bash
# Code check
cargo check

# Format code
cargo fmt

# Clippy static analysis
cargo clippy

# Run all checks
cargo clippy -- -D warnings
```

### Testing
```bash
# Run tests
cargo test

# Test with output
cargo test -- --nocapture

# Specific test
cargo test <test_name>
```

### Dependency Management
```bash
# Update dependencies
cargo update

# Add new dependency
cargo add <package_name>

# Clean cache
cargo clean
```

## API Endpoints

### Health Check
- `GET /health` - Basic health check
- `GET /health/detailed` - Detailed health information (including memory usage, etc.)

### Time Service
- `GET /api/v1/time` - Get server time
  - Parameters: `format` (iso8601|timestamp|formatted)
  - Parameters: `timezone` (e.g. +08:00)
- `GET /api/v1/time/zones` - Get multi-timezone time

### Documentation
- `GET /swagger-ui` - Swagger UI interface
- `GET /swagger-ui/openapi.json` - OpenAPI specification

## Configuration Management

### Configuration Priority
1. Environment variables (highest priority)
2. .env file
3. config.{RUST_ENV}.toml
4. config.toml (default)

### Main Configuration Items
```toml
[server]
host = "0.0.0.0"    # Service listening address
port = 5057         # Service port

[api]
base_path = "/api"  # API base path
version = "v1"      # API version

[swagger]
enabled = true      # Whether to enable Swagger
path = "/swagger-ui" # Swagger path
```

## Environment Variables

### Supported Environment Variables
- `SERVER_HOST` - Server host address
- `SERVER_PORT` - Server port
- `LOG_LEVEL` - Log level (trace|debug|info|warn|error)
- `RUST_ENV` - Runtime environment (development|production)

## Development Notes

### CORS Handling
- Development environment automatically configures CORS, allowing all origins
- Production environment needs to configure specific allowed domains

### Logging Configuration
- Uses tracing for structured logging
- Default log level is info
- Can be adjusted via RUST_LOG environment variable

### Error Handling
- Unified API response format
- Error messages include status codes and descriptions
- All responses have timestamps

### Language Convention
- **All code comments, documentation, and user-facing text must be in English**
- API error messages are in English
- Log messages are in English
- Configuration descriptions are in English

## Extension Development

### Adding New API Endpoints
1. Define data models in `src/models/`
2. Implement handler functions in `src/handlers/`
3. Add routes in `src/routes/`
4. Use `#[utoipa::path]` annotations to generate documentation

### Adding Middleware
1. Create middleware in `src/middleware/`
2. Apply in `create_app` in `src/lib.rs`

### Modifying Configuration
1. Update configuration structure in `src/config/mod.rs`
2. Update default values in `config.toml`
3. Update environment variable mapping