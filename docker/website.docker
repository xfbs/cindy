FROM debian:12
ENV ZOLA_VERSION=0.17.2

# install dependencies
RUN apt update && \
    apt install -y wget && \
    rm -rf /var/lib/apt/lists/*

# install zola
RUN wget -qO- https://github.com/getzola/zola/releases/download/v${ZOLA_VERSION}/zola-v${ZOLA_VERSION}-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin
