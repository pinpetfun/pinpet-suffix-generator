# PetAddr - Solana Pet Address Generator

A high-performance Rust web server that generates Solana addresses ending with lowercase letter + "Pet" suffix (e.g., aPet, bPet, nPet). Built with Axum framework and featuring automatic address pool management.

## Features

- **Pet Address Generation**: Generates Solana addresses that end with lowercase letter + "Pet" (aPet, bPet, cPet, ..., zPet)
- **Automatic Pool Management**: Maintains a pool of 100 pre-generated addresses
- **Background Generation**: Automatically generates new addresses when pool runs low
- **Persistent Storage**: Uses sled embedded database for address storage
- **REST API**: Simple HTTP endpoints for address retrieval
- **Swagger Documentation**: Built-in API documentation

## Quick Start

### Prerequisites

- Rust 1.70+ 
- Cargo

### Installation & Running

```bash
# Clone the repository
git clone <repository-url>
cd pinpet-suffix-generator

# Run the server
./run.sh
# or
cargo run
```

The server will start on `http://localhost:5057`

## Getting Pet Addresses

### Get a Pet Address

Retrieve a Solana address ending with lowercase letter + "Pet" (e.g., aPet, bPet, nPet) along with its private key:

```bash
curl http://localhost:5057/api/v1/pet/address
```

**Response:**
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": 1,
    "public_key": "AGm9DpEaQYHxLKy98WGGoqErJEML9Pf5HySA1o4skPet",
    "private_key": "24vtL5hidJdxFhXPg3M6taaETDwwscLBLBjvoHDdSk4d...",
    "address": "AGm9DpEaQYHxLKy98WGGoqErJEML9Pf5HySA1o4skPet",
    "created_at": "2025-09-18T18:22:27.560460384+00:00"
  },
  "timestamp": 1758220011
}
```

### Check Generation Status

Monitor the address pool status:

```bash
curl http://localhost:5057/api/v1/pet/status
```

**Response:**
```json
{
  "code": 200,
  "message": "success",
  "data": {
    "total_addresses": 85,
    "pool_size": 100,
    "generation_active": true
  },
  "timestamp": 1758220047
}
```

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/pet/address` | GET | Get a Pet address with private key |
| `/api/v1/pet/status` | GET | Check generator status and pool size |
| `/health` | GET | Health check |
| `/swagger-ui` | GET | API documentation |

## Configuration

Configuration is managed through `config.toml`:

```toml
[server]
host = "0.0.0.0"
port = 5057

[pet_generator]
pool_size = 100          # Target number of addresses in pool
batch_size = 10          # Addresses generated per batch
db_path = "./data/pet_addresses.db"  # Database file path

[rate_limit]
max_requests_per_minute = 10
window_seconds = 60
```

## How It Works

1. **Background Generation**: Server continuously generates Solana keypairs
2. **Pet Validation**: Only addresses ending with lowercase letter + "Pet" (e.g., aPet, bPet, zPet) are stored
3. **Pool Management**: Maintains a pool of 100 ready-to-use addresses
4. **Auto-Replenishment**: Generates new addresses when pool drops below target
5. **Atomic Retrieval**: Each address is returned once and removed from pool

## Architecture

- **Layered Design**: Clean separation of concerns
- **Async Processing**: Built on Tokio async runtime
- **Embedded Database**: Uses sled for fast, local storage
- **RESTful API**: Standard HTTP endpoints with JSON responses
- **OpenAPI Documentation**: Auto-generated Swagger UI

## Development

```bash
# Check code
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Static analysis
cargo clippy
```

## License

MIT License - see [LICENSE](LICENSE) file for details.