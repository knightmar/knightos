use crate::descriptors::idt::load_idt;
use crate::descriptors::pic::Pic;
use crate::serial::Serial;
use crate::{log, println};
use core::arch::asm;

pub fn protected_main() {
    log!("Protected mode entered");
    let cs: u16;
    let ds: u16;
    let ss: u16;
    unsafe {
        asm!(
        "mov {0}, cs",
        "mov {1}, ds",
        "mov {2}, ds",
        out(reg) cs,
        out(reg) ds,
        out(reg) ss,
        );
    }

    // testing
    log!(format_args!("CS : {} DS : {} SS : {}", cs, ds, ss));
    println!("{}", format_args!("CS : {} DS : {} SS : {}", cs, ds, ss));
    println!("test");

    unsafe { Pic::remap() }
    unsafe { load_idt() }
    unsafe {
        Serial::outb(0x21, 0xFC);
    }
    log!("Testing Breakpoint...");
    unsafe {
        asm!("int3");
    }

    log!("Waiting for timer ticks...");
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
