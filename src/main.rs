#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod display;
use crate::display::display::{Color, Display};
use core::fmt::Write;
use ::core::panic::PanicInfo;


static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut display = Display::new();
    let color = Display::get_color(Color::Black, Color::Red);

    display.write_str("Ceci est un test\n");
    display.write_str("Banane");

    loop {}
}
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
