.PHONY: build run

build:
	./build.sh

test:
	cargo +nightly test --target x86-unknown-bare_metal.json --bin knightos
run: build
	qemu-system-i386 -cdrom knightos.iso \
	    -chardev stdio,mux=on,id=char0,logfile=serial.log,signal=off \
        -mon chardev=char0 \
        -serial chardev:char0 \

clear:
	rm -Rrf build || true
	rm -Rrf knightos.iso || true

clean:
	rm serial.log || true
	rm -Rrf build || true
	rm -Rrf knightos.iso || true
	rm target/x86-unknown-bare_metal/release/deps -Rrf || true
