# Dockerfile

# Use the latest Rust image
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/app

# Install dependencies needed for `cargo-watch` and SQLite
RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    pkg-config \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Initialize a Cargo project if Cargo.toml doesn't exist
RUN [ ! -f "Cargo.toml" ] && cargo init --bin . || true

# Ensure the `src/main.rs` file exists
RUN mkdir -p src && \
    [ ! -f "src/main.rs" ] && echo 'fn main() { println!("Hello, world!"); }' > src/main.rs || true

# Pre-build dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build

# Install `cargo-watch` with the latest Rust version
RUN CARGO_TARGET_DIR=/tmp/cargo-watch cargo install cargo-watch

# Copy the rest of the application files (if any)
COPY . .

# Expose the default Rocket port
EXPOSE 8000

# Command to start the application with hot-reloading
CMD ["cargo", "watch", "-x", "run"]
