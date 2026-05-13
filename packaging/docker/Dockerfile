# Multi-stage Docker build for binderd
# Supports linux/amd64 and linux/arm64

# Stage 1: Build
FROM rust:1.78-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy workspace manifest
COPY Cargo.toml Cargo.lock* ./
COPY crates/binder-schemas/Cargo.toml crates/binder-schemas/
COPY crates/binder-rpc/Cargo.toml crates/binder-rpc/
COPY crates/binder-core/Cargo.toml crates/binder-core/
COPY crates/binder-llm/Cargo.toml crates/binder-llm/
COPY crates/binder-host-linux/Cargo.toml crates/binder-host-linux/
COPY crates/binder-host-windows/Cargo.toml crates/binder-host-windows/
COPY crates/binder-host-darwin/Cargo.toml crates/binder-host-darwin/
COPY crates/binder-host-browser/Cargo.toml crates/binder-host-browser/
COPY bin/binderd/Cargo.toml bin/binderd/
COPY bin/binderctl/Cargo.toml bin/binderctl/

# Build dummy main.rs to cache dependencies
RUN mkdir -p crates/binder-schemas/src && echo "fn main() {}" > crates/binder-schemas/src/lib.rs
RUN mkdir -p crates/binder-rpc/src && echo "fn main() {}" > crates/binder-rpc/src/lib.rs
RUN mkdir -p crates/binder-core/src && echo "fn main() {}" > crates/binder-core/src/lib.rs
RUN mkdir -p crates/binder-llm/src && echo "fn main() {}" > crates/binder-llm/src/lib.rs
RUN mkdir -p crates/binder-host-linux/src && echo "fn main() {}" > crates/binder-host-linux/src/lib.rs
RUN mkdir -p crates/binder-host-windows/src && echo "fn main() {}" > crates/binder-host-windows/src/lib.rs
RUN mkdir -p crates/binder-host-darwin/src && echo "fn main() {}" > crates/binder-host-darwin/src/lib.rs
RUN mkdir -p crates/binder-host-browser/src && echo "fn main() {}" > crates/binder-host-browser/src/lib.rs
RUN mkdir -p bin/binderd/src && echo "fn main() {}" > bin/binderd/src/main.rs
RUN mkdir -p bin/binderctl/src && echo "fn main() {}" > bin/binderctl/src/main.rs

RUN cargo build --release 2>/dev/null || true

# Copy actual source
COPY . .

# Build release
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -s /bin/bash binder

RUN mkdir -p /var/lib/binder /var/run/binder /etc/binder/capabilities \
    && chown -R binder:binder /var/lib/binder /var/run/binder /etc/binder

COPY --from=builder /build/target/release/binderd /usr/local/bin/
COPY --from=builder /build/target/release/binderctl /usr/local/bin/
COPY capabilities/ /etc/binder/capabilities/

ENV BINDER_AUDIT_DB=/var/lib/binder/audit.db
ENV BINDER_CAPABILITIES_DIR=/etc/binder/capabilities

USER binder
EXPOSE 9242

ENTRYPOINT ["/usr/local/bin/binderd"]