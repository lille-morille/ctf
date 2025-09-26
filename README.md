# CTF Application

A simple Rust web application built with Axum framework.

## Prerequisites

- Docker and Docker Compose installed on your system
- (Optional) Rust toolchain if you want to develop locally

## Quick Start with Docker

### Using Docker Compose (Recommended)

```bash
# Start the application
docker compose up --build

# Force rebuild and start the application
docker compose up --build

# Stop the application
docker-compose down
```

### Using Docker Directly

```bash
# Build the Docker image
docker build -t ctf .

# Run the container
docker run -p 3000:3000 ctf
```

## Application Details

- **Port**: The application runs on port 3000
- **Framework**: Axum (Rust web framework)
- **Runtime**: Tokio async runtime

## Development

If you want to develop locally without Docker:

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and run the application
cargo run

# Build for release
cargo build --release
```

## Environment Variables

- `RUST_LOG=info`: Controls logging level (set in docker-compose.yml)

## Troubleshooting

If you encounter issues:

1. **Docker daemon not running**: Start Docker Desktop or your Docker service
2. **Port already in use**: Change the port mapping in docker-compose.yml
3. **Build cache issues**: Use `docker compose build --no-cache`

## License

This project is part of a university security exercise.
