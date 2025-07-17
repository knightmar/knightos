#![no_std]
#![no_main]

use crate::vga::colors::VGAColors::*;
use core::panic::PanicInfo;

mod vga;

#[allow(clippy::empty_loop)]
static HELLO: &[u8] = b"Hello World!";

#[cfg_attr(test, allow(dead_code))]
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for i in 0..80 * 25 {
        unsafe {
            *vga_buffer.offset((i * 2) as isize) = b' ';
            *vga_buffer.offset((i * 2 + 1) as isize) = get_colors!(Blue, Black);
        }
    }

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = get_colors!(Blue, White);
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
