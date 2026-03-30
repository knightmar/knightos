use crate::backend::interrupts::TICK_COUNT;
use crate::backend::serial::LogLevel::Info;
use crate::log;
use core::arch::asm;
use core::sync::atomic::Ordering;
use core::sync::atomic::Ordering::Relaxed;

pub mod allocator;
pub mod descriptors;
pub mod interrupts;
pub mod paging;
pub mod serial;
pub mod vga;

pub fn wait(time: u32) {
    let start = TICK_COUNT.load(Ordering::Relaxed);
    while TICK_COUNT.load(Ordering::Relaxed) - start < time {
        core::hint::spin_loop();
    }
}

pub fn qemu_shutdown() -> ! {
    unsafe {
        // Method for modern QEMU/Bochs
        asm!("out dx, ax", in("dx") 0x604, in("ax") 0x2000 as u16);

        // Fallback for older QEMU (ISA debug exit)
        // Requires: -device isa-debug-exit,iobase=0xf4,iosize=0x04
        asm!("out dx, al", in("dx") 0xf4, in("al") 0x00 as u8);
    }

    // If it didn't work, just hang
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
