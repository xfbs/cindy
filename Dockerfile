FROM rust:1.71-bookworm

RUN apt update
RUN apt install -y libsqlite3-dev libavcodec-dev libgtk-4-dev clang pkg-config libavformat-dev
