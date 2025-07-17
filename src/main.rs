#![no_std]
#![no_main]

use crate::vga::colors::VGAColors::*;
use crate::vga::VGAText;
use core::panic::PanicInfo;

mod serial;
mod vga;

#[allow(clippy::empty_loop)]
#[cfg_attr(test, allow(dead_code))]
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let mut vga = VGAText::new();

    vga.clear_screen();

    println!("test");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
