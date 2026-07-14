.PHONY: build run test clear clean

build:
	./build.sh

test:
	cargo +nightly test --target x86-unknown-bare_metal.json --bin knightos
run: build
	qemu-system-i386 -cdrom knightos.iso \
	    -chardev stdio,mux=on,id=char0,logfile=serial.log,signal=off \
        -mon chardev=char0 \
        -serial chardev:char0 \
        -m 512M

vnc :
	$(MAKE) run &
	@sleep 2
	vncviewer localhost:5900

gdb: build
	qemu-system-i386 -cdrom knightos.iso \
		-chardev stdio,mux=on,id=char0,logfile=serial.log,signal=off \
		-mon chardev=char0 -serial chardev:char0 \
		-gdb tcp::1234 -S -m 512M

clear:
	rm -Rrf build || true
	rm -Rrf knightos.iso || true

clean:
	rm serial.log || true
	rm -Rrf build || true
	rm -Rrf knightos.iso || true
	rm target -Rrf || true
