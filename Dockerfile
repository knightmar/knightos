FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    binutils \
    xorriso \
    grub-pc-bin \
    mtools \
    git \
    && rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-2026-07-15

RUN rustup component add rust-src llvm-tools-preview rustfmt clippy && \
    cargo install cargo-binutils

WORKDIR /workspace

CMD ["/bin/bash", "./build.sh"]