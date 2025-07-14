.PHONY: build run

build:
	./build.sh

run: build
	qemu-system-i386 -cdrom knightos.iso

clear:
	rm -Rrf build
	rm -Rrf knightos.iso
	rm target -Rrf

clean:
	rm -Rrf build
	rm -Rrf knightos.iso
	rm target/x86-unknown-bare_metal/release/deps -Rrf
