# Halt.rs Dockerfile

FROM rust:1.75-slim AS rust-builder

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Node.js stage for MCP server
FROM node:18-slim AS node-builder

WORKDIR /app/mcp-server

# Copy package files
COPY mcp-server/package*.json ./

# Install dependencies
RUN npm ci --only=production

# Copy source
COPY mcp-server/src ./src

# Build MCP server
RUN npm run build

# Python stage for bindings
FROM python:3.11-slim AS python-builder

WORKDIR /app/bindings/python

# Copy Python requirements
COPY bindings/python/setup.py bindings/python/requirements.txt ./

# Install dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Copy Python source
COPY bindings/python/halt ./halt

# Build Python package
RUN python setup.py build_ext --inplace

# Final stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    openjdk-17-jre-headless \
    python3 \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

# Create application user
RUN useradd -r -s /bin/false halt

# Create directories
RUN mkdir -p /app/bin /app/lib /app/config /app/logs /app/data
WORKDIR /app

# Copy built binaries and libraries
COPY --from=rust-builder /app/target/release/halt ./bin/
COPY --from=node-builder /app/mcp-server/dist ./lib/mcp/
COPY --from=python-builder /app/bindings/python/build/lib* ./lib/python/

# Copy configuration and documentation
COPY config/*.env ./config/
COPY README.md docs/

# Copy Java bindings
COPY HaltProxy.java ./

# Compile Java bindings
RUN javac -d lib/java HaltProxy.java

# Set permissions
RUN chown -R halt:halt /app
USER halt

# Expose ports
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Default command
CMD ["./bin/halt", "serve", "--host", "0.0.0.0", "--port", "8080"]

# Labels
LABEL org.opencontainers.image.title="Halt.rs" \
      org.opencontainers.image.description="Multi-Agent Proxy for Swarm Control" \
      org.opencontainers.image.version="0.1.0" \
      org.opencontainers.image.authors="Halt.rs Team" \
      org.opencontainers.image.source="https://github.com/halt-rs/halt" \
      org.opencontainers.image.licenses="MIT"
