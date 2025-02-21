# Rust
FROM rust:1.85 AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# Image
FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/rust-streamer /app/

CMD ["./rust-streamer"]
