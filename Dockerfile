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

ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
RUN rustup component add rust-src llvm-tools-preview rustfmt clippy
RUN cargo install cargo-binutils

WORKDIR /workspace
COPY . .

RUN find /workspace -name "*.sh" -exec sed -i 's/\r$//' {} \; \
    && chmod +x /workspace/build.sh /workspace/docker-entrypoint.sh

ENTRYPOINT ["/workspace/docker-entrypoint.sh"]
