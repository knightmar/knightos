make clean
mkdir -p build/isodir/boot/grub

i686-elf-as src/boot/boot.s -o ./build/boot.o
cargo build --release --target x86-unknown-bare_metal.json
cp target/x86-unknown-bare_metal/release/deps/knightos-*.o build/knightos.o
i686-elf-gcc -T src/boot/linker.ld -o build/isodir/boot/knightos.bin -ffreestanding -O2 -nostdlib build/boot.o build/knightos.o -lgcc
cp src/boot/grub.cfg build/isodir/boot/grub/grub.cfg
grub-mkrescue -o knightos.iso build/isodir
