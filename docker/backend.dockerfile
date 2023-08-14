FROM rust:1.71-bookworm

# dependencies needed for cindy backend
RUN apt update && \
    apt install -y libsqlite3-dev libavcodec-dev clang pkg-config libavformat-dev && \
    rm -rf /var/lib/apt/lists/*

# dependencies needed for coverage reporting
RUN rustup component add llvm-tools
RUN rustup component add clippy
RUN cargo install cargo-llvm-cov
