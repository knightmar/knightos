#![no_std]
#![no_main]

use crate::vga::colors::VGAColors::*;
use core::fmt::{Debug, Pointer};
use core::panic::PanicInfo;
use crate::serial::LogLevel::Error;

mod serial;
mod vga;

#[allow(clippy::empty_loop)]
#[cfg_attr(test, allow(dead_code))]
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    println!("Init");

    log!("test");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga::WRITER.lock().change_fg_color(Red);
    println!("{}", info);
    loop {}
}
