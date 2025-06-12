CC=i686-elf-gcc
AS=i686-elf-as
CFLAGS=-std=gnu99 -ffreestanding -O2 -Wall -Wextra
LDFLAGS=-T linker.ld -ffreestanding -O2 -nostdlib
BUILD=build
ISO=$(BUILD)/myos.iso
BIN=$(BUILD)/myos.bin

all: $(ISO)

$(BUILD):
	mkdir -p $(BUILD)

$(BUILD)/boot.o: boot.s | $(BUILD)
	$(AS) boot.s -o $(BUILD)/boot.o

$(BUILD)/kernel.o: src/kernel.c | $(BUILD)
	$(CC) -c src/kernel.c -o $(BUILD)/kernel.o $(CFLAGS)

$(BIN): linker.ld $(BUILD)/boot.o $(BUILD)/kernel.o | $(BUILD)
	$(CC) $(LDFLAGS) $(BUILD)/boot.o $(BUILD)/kernel.o -o $(BIN) -lgcc

$(BUILD)/isodir/boot/myos.bin: $(BIN)
	mkdir -p $(BUILD)/isodir/boot
	cp $(BIN) $(BUILD)/isodir/boot/myos.bin

$(BUILD)/isodir/boot/grub/grub.cfg: grub.cfg
	mkdir -p $(BUILD)/isodir/boot/grub
	cp grub.cfg $(BUILD)/isodir/boot/grub/grub.cfg

$(ISO): $(BUILD)/isodir/boot/myos.bin $(BUILD)/isodir/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO) $(BUILD)/isodir

run: $(ISO)
	qemu-system-i386 -cdrom $(ISO)

clean:
	rm -rf $(BUILD)

.PHONY: all clean run