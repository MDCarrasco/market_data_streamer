# Step 1: Build Rust in Alpine
FROM rust:1.85-alpine AS builder

RUN apk add --no-cache \
    musl-dev \
    ca-certificates \
    perl \
    make \
    pkgconfig \
    openssl-dev \
    build-base

WORKDIR /usr/src/app
COPY . .

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --release --target x86_64-unknown-linux-musl

# Step 2: Minimal Scratch Image
FROM scratch

# Copy CA certificates from Alpine builder
COPY --from=builder /etc/ssl/certs /etc/ssl/certs

# Copy Rust binary
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/rust-streamer /rust-streamer

CMD ["/rust-streamer"]
