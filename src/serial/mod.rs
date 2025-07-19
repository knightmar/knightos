use core::arch::asm;

struct Serial {
    port: u16,
}

impl Serial {
    pub fn new(port: u16) -> Self {
        Serial { port }
    }

    pub fn outb(port: u16, val: u8) {
        unsafe {
            asm!(
            "out dx, al",
            in("dx") port,
            in("al") val,
            options(nostack, nomem, preserves_flags)
            );
        }
    }

    pub fn inb(port: u16) -> u8 {
        let mut value: u8;
        unsafe {
            asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nostack, nomem, preserves_flags)
            );
        }

        value
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        Self::outb(self.port + 1, 0x00);
        Self::outb(self.port + 3, 0x80);
        Self::outb(self.port + 0, 0x03);
        Self::outb(self.port + 1, 0x00);
        Self::outb(self.port + 3, 0x03);
        Self::outb(self.port + 2, 0xC7);
        Self::outb(self.port + 4, 0x0B);
        Self::outb(self.port + 4, 0x1E);
        Self::outb(self.port + 0, 0xAE);

        // if Self::inb(self.port + 0) != 0xAE {
        //     Err("test")
        // }

        Ok(())
    }
}
