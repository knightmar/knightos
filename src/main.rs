#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod display;
use crate::display::display::{Color, Display};
use ::core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut display = Display::new();
    let color = Display::get_color(Color::Black, Color::LightGray);

    HELLO.iter().for_each(|x| {
        display.print_chr(char::from(x.clone()), color);
    });

    for _ in (0..1000) {
        display.print_chr('A', color);
    }

    loop {}
}
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
