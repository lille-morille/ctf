FROM rust:1.90-slim-bullseye AS app

WORKDIR /app

COPY ./Cargo.toml ./
COPY ./Cargo.lock ./
COPY ./src ./src

RUN cargo build --release

CMD ["./target/release/ctf"]
