FROM debian:bookworm-slim

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    build-essential \
    grub-pc-bin \
    xorriso \
    mtools \
    binutils \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup component add llvm-tools-preview rust-src

RUN cargo install cargo-binutils

WORKDIR /usr/src/knightos

CMD ["/bin/bash", "-c", "chmod +x ./build.sh && ./build.sh"]
