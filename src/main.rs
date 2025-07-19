#![no_std]
#![no_main]

use crate::vga::colors::VGAColors::*;
use core::fmt::{Debug, Pointer};
use core::panic::PanicInfo;

mod serial;
mod vga;

#[allow(clippy::empty_loop)]
#[cfg_attr(test, allow(dead_code))]
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let mut i: u64 = 0;
    loop {
        println!("You are at {} loops ", i);
        i += 1;
        if i == u64::MAX -1 {
            break;
        }
    }

    println!("Done");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga::WRITER.lock().change_fg_color(Red);
    println!("{}", info);
    loop {}
}
