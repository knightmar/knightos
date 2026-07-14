#!/bin/bash
export PATH=$PATH:$HOME/opt/cross/bin/
export PATH=$PATH:$HOME/.cargo/bin



make clear
mkdir -p build/isodir/boot/grub

if command -v i686-elf-as &> /dev/null; then
    echo "i686-elf-as found, using it"
    i686-elf-as src/boot/boot.s -o ./build/boot.o
else
    echo "i686-elf-as not found, switching to native assembler in 32-bit mode"
    as --32 src/boot/boot.s -o ./build/boot.o
fi

cargo +nightly build --release --target x86-unknown-bare_metal.json || exit 1

KERNEL_ELF=target/x86-unknown-bare_metal/release/knightos

# Preferred: GRUB loads ELF kernels directly
cp "$KERNEL_ELF" build/isodir/boot/knightos.elf

cargo objcopy -- -O binary "$KERNEL_ELF" build/isodir/boot/knightos.bin

cp src/boot/grub.cfg build/isodir/boot/grub/grub.cfg
grub-mkrescue -o knightos.iso build/isodir
