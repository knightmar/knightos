#!/bin/bash
export PATH=$PATH:/home/knightmar/opt/cross/bin/

make clean
mkdir -p build/isodir/boot/grub

# If boot.s is still required for your multiboot header, keep assembling it;
# otherwise you can remove these two lines.
i686-elf-as src/boot/boot.s -o ./build/boot.o

cargo +nightly build --release --target x86-unknown-bare_metal.json

KERNEL_ELF=target/x86-unknown-bare_metal/release/knightos

# Preferred: GRUB loads ELF kernels directly
cp "$KERNEL_ELF" build/isodir/boot/knightos.elf

# If you insist on a flat binary:
rust-objcopy -O binary "$KERNEL_ELF" build/isodir/boot/knightos.bin

cp src/boot/grub.cfg build/isodir/boot/grub/grub.cfg
grub-mkrescue -o knightos.iso build/isodir