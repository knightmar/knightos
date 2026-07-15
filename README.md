# KnightOS
A x86 rust kernel written by hand.
The main idea of the project is to learn, so I'm trying to write the most code possible myself.
It is surely not efficient, a lot of parts might seem silly or strangely implemented, that's because I try to do the design myself and not look to much on what have been done elsewhere. 

## Design
I try to split all the features in dedicated modules, and organize them in logical places. For exemple, an interrupt module, a memory module, etc.. all in the backend global module.

I'll maybe do a proper mapping of the archi i'm building at some point.

## Features
For now, here is a non exhaustive list of the features implemented:
- ASM bootloader
- basic booting
- memory module (pmm, vmm, allocator, paging)
- interrupts
- scheduler (RR for now, but will change)
- serial logging
- testsuite using cargo test
- UI at pixel level (supporting fullhd)
- text display (with custom bitmap font)
- keyboard input
- proper build system (using make)
- automated github action build

## TODO
- [] Proper UI lib
- [] IDLE task
- [] New scheduler algo (+ better task state handling)
- [] Mouse input
- [] Basic shell input and commands
- [] Filesystem
- [] BMP Image display


