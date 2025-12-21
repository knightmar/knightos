use crate::vga::colors::VGAColors::{Red, Yellow};
use crate::{log, print, println, vga};
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

    log!(format_args!("CS : {} DS : {} SS : {}", cs, ds, ss));
    println!("{}", format_args!("CS : {} DS : {} SS : {}", cs, ds, ss));
    println!("test");
    loop {}
}
