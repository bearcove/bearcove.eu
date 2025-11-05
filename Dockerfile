FROM rust:1 AS builder
WORKDIR /app

# Install LLVM for WASM builds (tree-sitter C compilation)
RUN apt-get update && \
    apt-get install -y llvm clang lld && \
    rm -rf /var/lib/apt/lists/*

# Set environment variables for WASM builds
ENV CC=clang
ENV AR=llvm-ar

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli@0.7.0 --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

# Install wasm32-unknown-unknown target
RUN rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli
RUN cargo binstall wasm-bindgen-cli --root /.cargo -y --force

COPY . .

# Create the final bundle folder. Bundle always executes in release mode with optimizations enabled
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    dx bundle -p bearcove-eu --release --web && \
    cp -rfv /app/target/dx/bearcove-eu/release/web /app/

####################################################################################

FROM debian:bookworm-slim AS runtime

# Install ca-certificates for HTTPS connections
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/web/ /usr/local/app

# Create non-root user and change ownership
RUN useradd -m -u 1000 appuser && \
    chown -R appuser:appuser /usr/local/app

# set our port and make sure to listen for all connections
ENV PORT=8080
ENV IP=0.0.0.0

# expose the port 8080
EXPOSE 8080

# Switch to non-root user
USER appuser

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/bearcove-eu" ]

####################################################################################
