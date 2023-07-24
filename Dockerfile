FROM rust:1.71-bookworm
ENV TRUNK_VERSION=0.17.2

# install dependencies
RUN apt update && \
    apt install -y libsqlite3-dev libavcodec-dev clang pkg-config libavformat-dev && \
    rm -rf /var/lib/apt/lists/*

# install wasm support
RUN rustup target add wasm32-unknown-unknown

# install trunk
RUN wget -qO- https://github.com/thedodd/trunk/releases/download/v${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin
