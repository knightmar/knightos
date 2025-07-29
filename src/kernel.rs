use crate::log;
use core::arch::asm;

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

    log!(format_args!("CS : {} DS :{}", cs, ds));
    loop {}
}
