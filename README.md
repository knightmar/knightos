# KnightOS
A x86 rust kernel written by hand.
The main idea of the project is to learn, so I'm trying to write the most code possible myself.
It is surely not efficient, a lot of parts might seem silly or strangely implemented, that's because I try to do the design myself and not look to much on what have been done elsewhere. 

## Design
I try to split all the features in dedicated modules, and organize them in logical places. For exemple, an interrupt module, a memory module, etc.. all in the backend global module.

I'll maybe do a proper mapping of the archi i'm building at some point.

## Features
For now, here is a non exhaustive list of the features implemented:
- [x] ASM bootloader
- [x] basic booting
- [x] memory module (pmm, vmm, allocator, paging)
- [x] interrupts
- [x] scheduler (RR for now, but will change)
- [x] serial logging
- [x] testsuite using cargo test
- [x] UI at pixel level (supporting fullhd)
- [x] text display (with custom bitmap font)
- [x] keyboard input
- [x] proper build system (using make)
- [x] automated github action build

## TODO
- [ ] Proper UI lib
- [ ] IDLE task
- [ ] New scheduler algo (+ better task state handling)
- [ ] Mouse input
- [ ] Basic shell input and commands
- [ ] Filesystem
- [ ] BMP Image display

## How to try
Run workflow:
- Get an ISO file in the latest github action successfull build [here](https://github.com/knightmar/knightos/actions)
- if you want to run it in vm, get a compatible vm that is able to run i686 (vmware, virtualbox or qemu)
- if you want to try it on a physical computer, flash the iso on a usb drive and run it as you would do with any other iso.

## How to build
To build the projet yourself :

1. Install the requirements
### Debian / Ubuntu
```
sudo apt update && sudo apt upgrade -y
```
```
sudo apt install -y build-essential binutils grub-pc-bin xorriso mtools wget curl git qemu-system-x86
```
### Arch
```
sudo pacman -Syu
```
```
sudo pacman -S --needed base-devel grub libisoburn mtools wget curl git qemu-desktop
```

2. Install rust nightly, press 1 to go with the default install, then restart your terminal
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
```
rustup toolchain install nightly
```
3. Install the rust requirements
```
rustup component add llvm-tools-preview rust-src --toolchain nightly
```
```
cargo install cargo-binutils
```
4. Clone the repo
```
git clone https://github.com/knightmar/knightos
```
5. Build the project
```
make build
```
6. Build done, the iso is at the root of the project, congrats !






