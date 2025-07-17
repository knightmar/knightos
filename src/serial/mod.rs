use core::arch::asm;

struct Serial {
    port: u8,
}

impl Serial {
    pub fn new(port: u8) -> Self {
        Serial { port }
    }

    pub fn init(&mut self) {
        unsafe {
            asm!(
            "outb "
            );
        }
    }
}
