# syntax=docker/dockerfile:1

FROM rust:1.99-slim@sha256:aab42391a986424e22d6a3ec14f37bc6dd3dacff4d457ca7cccd781e02297cc6 AS builder
WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release && rm -rf src

# Copy actual source
COPY src ./src

# Build application
RUN touch src/main.rs && cargo build --release

# Runtime
FROM debian:bookworm-slim AS runner
WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/template-poem ./app

ENV RUST_LOG=info
EXPOSE 3000

CMD ["./app"]
