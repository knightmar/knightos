use crate::serial::Serial;

pub struct Pic;

impl Pic {
    const MASTER_CMD: u16 = 0x20;
    const MASTER_DATA: u16 = 0x21;
    const SLAVE_CMD: u16 = 0xA0;
    const SLAVE_DATA: u16 = 0xA1;

    pub unsafe fn remap() {
        Serial::outb(Self::MASTER_CMD, 0x11);
        Serial::outb(Self::SLAVE_CMD, 0x11);

        Serial::outb(Self::MASTER_DATA, 0x20);
        Serial::outb(Self::SLAVE_DATA, 0x28);

        Serial::outb(Self::MASTER_DATA, 0x04);
        Serial::outb(Self::SLAVE_DATA, 0x02);

        Serial::outb(Self::MASTER_DATA, 0x01);
        Serial::outb(Self::SLAVE_DATA, 0x01);

        Serial::outb(Self::MASTER_DATA, 0xFF);
        Serial::outb(Self::SLAVE_DATA, 0xFF);
    }
}
