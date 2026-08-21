FROM rust:latest AS builder
LABEL authors="mikhail_zagoruiko"

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY hello_world_service ./hello_world_service

RUN cargo build --release --package hello_world_service

FROM debian:trixie-slim

COPY --from=builder \
    /app/target/release/hello_world_service \
    /usr/local/bin/hello_world_service

ENTRYPOINT ["/usr/local/bin/hello_world_service"]