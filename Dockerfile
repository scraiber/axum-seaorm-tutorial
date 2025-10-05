# Build stage
FROM rust:1.90-slim-bookworm AS builder

WORKDIR /app

# Install dependencies and musl target in single layer
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    musl-tools \
    && rm -rf /var/lib/apt/lists/* \
    && rustup target add x86_64-unknown-linux-musl

# Copy manifests and source in single layer
COPY Cargo.toml ./
COPY src ./src

# Build statically linked binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage - minimal scratch container
FROM scratch

# Copy CA certificates and binary
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/axum-seaorm /axum-seaorm

# Expose port
EXPOSE 3000

# Run the binary
CMD ["/axum-seaorm"]

