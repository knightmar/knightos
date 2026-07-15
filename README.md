# KnightOS

[![RepoGrade](https://www.repo-grade.com/api/badge/knightmar/knightos)](https://www.repo-grade.com/report/knightmar/knightos)
[![Build Status](https://github.com/knightmar/knightos/actions/workflows/build.yml/badge.svg)](https://github.com/knightmar/knightos/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust: Nightly](https://img.shields.io/badge/Rust-Nightly-orange.svg)](https://www.rust-lang.org/)

A handcrafted x86 Rust kernel.

The main goal of this project is learning, so I'm trying to write as much code as possible myself. 
It might not be highly optimized, and some parts may seem unusual or strangely implemented, but that's because I choose to design things myself rather than copying existing OS implementations.

## Design
I try to separate all features into dedicated modules organized logically (e.g., an `interrupts` module, a `memory` module, etc.) under the kernel core.

*I might create a proper architecture diagram at some point.*

## Features
Here is a non-exhaustive list of currently implemented features:
- [x] Assembly bootloader & Multiboot compliance
- [x] Basic booting sequence
- [x] Memory management (PMM, VMM, custom allocator, paging)
- [x] Hardware Interrupts (IDT, PIC)
- [x] Cooperative Task Scheduler (Round Robin for now)
- [x] Serial logging (COM1)
- [x] Integration testsuite using `cargo test`
- [x] Pixel-level Graphics Driver (supporting FullHD)
- [x] Text rendering (using a custom bitmap font)
- [x] Keyboard input handling
- [x] Unified build system (using `make`)
- [x] Automated CI builds via GitHub Actions

## TODO
- [ ] Proper UI / Window Library
- [ ] System IDLE task
- [ ] Advanced scheduler algorithm (with better task state management)
- [ ] Mouse input handling
- [ ] Basic shell interface & commands
- [ ] Simple File System
- [ ] BMP Image decoding & display
- [ ] Github action testing

## How to Try (No Compilation Required)
If you just want to run KnightOS without setting up a build environment:
1. Go to the [GitHub Actions page](https://github.com/knightmar/knightos/actions) of this repository.
2. Click on the latest successful build.
3. Scroll down to the **Artifacts** section and download the `knightos-iso` zip file.
4. Extract the `.iso` file.
5. Boot it using a virtual machine (QEMU, VirtualBox, VMware) or flash it onto a USB drive to test it on real x86 hardware.

## How to Build

If you want to compile and modify KnightOS yourself:

### 1. Install System Dependencies

#### Debian / Ubuntu
```bash
sudo apt update && sudo apt upgrade -y
sudo apt install -y build-essential make binutils grub-pc-bin xorriso mtools wget curl git qemu-system-x86
```

#### Arch Linux
```bash
sudo pacman -Syu
sudo pacman -S --needed base-devel make grub libisoburn mtools wget curl git qemu-desktop
```

### 2. Install Rust Nightly & Toolchain Components
```bash
# Install Rustup (press 1 to proceed with default installation, then restart your terminal)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install the Nightly toolchain and components needed for bare-metal compilation
rustup toolchain install nightly
rustup component add llvm-tools-preview rust-src --toolchain nightly
cargo install cargo-binutils
```

### 3. Clone and Build
```bash
# Clone the repository
git clone https://github.com/knightmar/knightos
cd knightos

# Build the bootable ISO
make build
```

The build is complete! Your bootable image is located at the root of the project: **`knightos.iso`**. 

*If you have QEMU installed, you can boot it instantly with:*
```bash
make run
```

# AI usage
No any code has been written by AI, I've used AI only for thoses tasks:
- Debugging (logs analyses, etc..) 
- General architecte planning (no code)
- Theorical explanations 
- Github actions setup
