# Use the official Rust image as the base
FROM rust:1.78-slim-bullseye AS builder

# Create a new empty shell project
RUN USER=root cargo new --bin ctf
WORKDIR /ctf

# Copy over the manifests
COPY ./Cargo.toml ./Cargo.toml

# Build the dependencies only (this layer will be cached if Cargo.toml doesn't change)
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build the application
RUN cargo build --release

# Start a new stage with a minimal runtime image
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /ctf/target/release/ctf /usr/local/bin/ctf

# Expose the port the app runs on
EXPOSE 3000

# Set the startup command to run the binary
CMD ["ctf"]