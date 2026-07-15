FROM rust:1.80-slim AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    binutils \
    grub-pc-bin \
    xorriso \
    mtools \
    curl \
    git \
    make \
    && rm -rf /var/lib/apt/lists/*

RUN rustup toolchain install nightly \
    && rustup default nightly \
    && rustup component add llvm-tools-preview rust-src --toolchain nightly \
    && cargo install cargo-binutils

WORKDIR /usr/src/knightos

COPY . .

RUN make build

FROM scratch AS exporter
COPY --from=builder /usr/src/knightos/knightos.iso /
