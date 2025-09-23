# CTF Application

A simple Rust web application built with Axum framework.

## Prerequisites

- Docker and Docker Compose installed on your system
- (Optional) Rust toolchain if you want to develop locally

## Quick Start with Docker

### Using Docker Compose (Recommended)

```bash
# Build and start the application
docker-compose up --build

# Run in detached mode
docker-compose up -d --build

# View logs
docker-compose logs -f

# Stop the application
docker-compose down
```

### Using Docker Directly

```bash
# Build the Docker image
docker build -t ctf-app .

# Run the container
docker run -p 3000:3000 ctf-app
```

## Application Details

- **Port**: The application runs on port 3000
- **Endpoint**: `GET /` returns "Hello, World!"
- **Framework**: Axum (Rust web framework)
- **Runtime**: Tokio async runtime

## Health Check

The application includes a health check endpoint. You can test it with:

```bash
curl http://localhost:3000
```

Expected response: `Hello, World!`

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

## Docker Image Optimization

The Dockerfile uses a multi-stage build to create a minimal final image:
- Builder stage: Compiles the Rust application
- Runtime stage: Uses a slim Debian base with only the compiled binary

## Environment Variables

- `RUST_LOG=info`: Controls logging level (set in docker-compose.yml)

## Troubleshooting

If you encounter issues:

1. **Docker daemon not running**: Start Docker Desktop or your Docker service
2. **Port already in use**: Change the port mapping in docker-compose.yml
3. **Build cache issues**: Use `docker-compose build --no-cache`

## License

This project is part of a university security exercise.