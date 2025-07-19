#!/bin/bash

export PATH=$PATH:/home/knightmar/opt/cross/bin/

make clean
mkdir -p build/isodir/boot/grub

i686-elf-as src/boot/boot.s -o ./build/boot.o

cargo build --release --target x86-unknown-bare_metal.json

cp target/x86-unknown-bare_metal/release/deps/knightos-*.o build/knightos.o

find target/x86-unknown-bare_metal/release/deps -name "libcore-*.rlib" | head -1 | xargs ar x
find target/x86-unknown-bare_metal/release/deps -name "libcompiler_builtins-*.rlib" | head -1 | xargs ar x

CORE_OBJS=$(find . -maxdepth 1 -name "*.o" ! -name "boot.o" ! -name "knightos.o")

i686-elf-ld -T src/boot/linker.ld -o build/isodir/boot/knightos.bin build/boot.o build/knightos.o $CORE_OBJS

rm -f *.o
rm target/x86-unknown-bare_metal/release/deps/knightos-*.o

cp src/boot/grub.cfg build/isodir/boot/grub/grub.cfg
grub-mkrescue -o knightos.iso build/isodir