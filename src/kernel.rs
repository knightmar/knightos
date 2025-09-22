use crate::{log, println, vga};
use core::arch::asm;
use crate::vga::colors::VGAColors::{Red, Yellow};

pub fn protected_main() {
    log!("Protected mode entered");
    let cs: u16;
    let ds: u16;
    unsafe {
        asm!(
        "mov {0:x}, cs",
        "mov {1:x}, ds",
        out(reg) cs,
        out(reg) ds,
        );
    }

    log!(format_args!("CS : {} DS : {}", cs, ds));
    vga::WRITER.lock().change_color(Red, Yellow);
    println!("test");
    loop {}
}
