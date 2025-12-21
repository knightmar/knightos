.PHONY: build run

build:
	./build.sh

run: build
	qemu-system-i386 -cdrom knightos.iso \
	    -chardev stdio,mux=on,id=char0,logfile=qemu_file.txt,signal=off \
        -mon chardev=char0 \
        -serial chardev:char0 \

clear:
	rm -Rrf build
	rm -Rrf knightos.iso
	rm target -Rrf

clean:
	rm serial.log
	rm -Rrf build
	rm -Rrf knightos.iso
	rm target/x86-unknown-bare_metal/release/deps -Rrf
