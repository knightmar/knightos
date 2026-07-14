#!/bin/bash
export PATH=$PATH:/home/knightmar/opt/cross/bin/
export PATH=$PATH:$HOME/.cargo/bin



make clear
mkdir -p build/isodir/boot/grub

i686-elf-as src/boot/boot.s -o ./build/boot.o

cargo +nightly build --target x86-unknown-bare_metal.json || exit 1

KERNEL_ELF=target/x86-unknown-bare_metal/debug/knightos

# Preferred: GRUB loads ELF kernels directly
cp "$KERNEL_ELF" build/isodir/boot/knightos.elf

cargo objcopy -- -O binary "$KERNEL_ELF" build/isodir/boot/knightos.bin

cp src/boot/grub.cfg build/isodir/boot/grub/grub.cfg
grub-mkrescue -o knightos.iso build/isodir
