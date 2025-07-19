#![no_std]
#![no_main]

use core::fmt::{Debug, Pointer};
use core::panic::PanicInfo;
use crate::vga::colors::VGAColors::*;

mod serial;
mod vga;

#[allow(clippy::empty_loop)]
#[cfg_attr(test, allow(dead_code))]
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    println!("test1");
    vga::WRITER.lock().change_fg_color(Yellow);
    println!("test2");
    vga::WRITER.lock().change_bg_color(LightBlue);
    println!("test3");

    panic!("ERROR TEST");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga::WRITER.lock().change_fg_color(Red);
    println!("{}", info);
    loop {}
}
