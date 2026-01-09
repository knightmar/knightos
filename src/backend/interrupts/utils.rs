use core::arch::asm;



pub fn hlt_loop() {
    loop {
        unsafe { asm!("hlt") }
    }
}
