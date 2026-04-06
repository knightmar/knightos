use crate::backend::interrupts::TICK_COUNT;
use core::arch::asm;
use core::sync::atomic::Ordering::Relaxed;

pub mod descriptors;
pub mod interrupts;
pub mod memory;
pub mod paging;
pub mod serial;
pub mod vga;

pub fn wait(time: u32) {
    unsafe {
        let start = TICK_COUNT.load(Relaxed);
        while TICK_COUNT.load(Relaxed) - start < time {
            asm!("hlt");
        }
    }
}

pub fn qemu_shutdown() -> ! {
    unsafe {
        // Method for modern QEMU/Bochs
        asm!("out dx, ax", in("dx") 0x604, in("ax") 0x2000 as u16);

        // Fallback for older QEMU (ISA debug exit)
        asm!("out dx, al", in("dx") 0xf4, in("al") 0x00 as u8);
    }
    
    // Fallback
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
