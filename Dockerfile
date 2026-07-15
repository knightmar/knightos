FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    bison \
    flex \
    libgmp3-dev \
    libmpc-dev \
    libmpfr-dev \
    texinfo \
    xorriso \
    grub-pc-bin \
    grub-common \
    mtools \
    git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /tmp
RUN curl -O https://ftp.gnu.org/gnu/binutils/binutils-2.42.tar.gz && \
    tar -xf binutils-2.42.tar.gz && \
    mkdir binutils-build && \
    cd binutils-build && \
    ../binutils-2.42/configure --target=i686-elf --prefix="/usr/local" --with-sysroot --disable-nls --disable-werror && \
    make -j$(nproc) && \
    make install && \
    rm -rf /tmp/*

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-2026-07-15

RUN rustup component add rust-src llvm-tools-preview && \
    cargo install cargo-binutils

WORKDIR /workspace

CMD ["/bin/bash", "./build.sh"]