FROM rust:latest

# Install system dependencies from flake.nix
RUN apt-get update && apt-get install -y \
    pkg-config \
    openssl \
    libssl-dev \
    nodejs \
    npm \
    lld \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Install nightly Rust and wasm target
RUN rustup default nightly && \
    rustup component add rust-analyzer rust-src && \
    rustup target add wasm32-unknown-unknown

# Install tailwindcss via npm globally
RUN npm install -g tailwindcss

# Install cargo-leptos
RUN cargo install cargo-leptos --locked

WORKDIR /app

# Copy entire project
COPY . .

# Build with leptos
RUN cargo leptos build --release

# Expose port
EXPOSE 3000

# Set environment for logging
ENV RUST_LOG=gitcoda_web=debug,gitcoda=debug

# Run server
CMD ["cargo", "leptos", "serve", "--release"]
