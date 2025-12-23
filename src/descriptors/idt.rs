use crate::interrupts::*;
use core::arch::asm;
use core::ptr::addr_of;

#[repr(C, packed)]
struct IDTDescriptor {
    size: u16,
    offset: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    reserved: u8,
    attributes: u8,
    offset_high: u16,
}

static mut IDT: [IdtEntry; 256] = [IdtEntry::new(); 256];

impl IdtEntry {
    pub const fn new() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            reserved: 0,
            attributes: 0,
            offset_high: 0,
        }
    }

    pub fn set_handler(&mut self, handler: u32) {
        self.offset_low = (handler & 0xFFFF) as u16;
        self.offset_high = ((handler >> 16) & 0xFFFF) as u16;
        self.selector = 0x08;
        self.reserved = 0;
        self.attributes = 0x8E;
    }
}

pub unsafe fn load_idt() {
    let idt_ptr = IDTDescriptor {
        size: (size_of::<[IdtEntry; 256]>() - 1) as u16,
        offset: unsafe { addr_of!(IDT) as u32 },
    };

    IDT[3].set_handler(breakpoint_handler as u32);
    IDT[8].set_handler(double_fault_handler as u32);
    IDT[32].set_handler(timer_handler as u32);
    IDT[33].set_handler(keyboard_handler as u32);

    unsafe {
        // load idt
        asm!(
        "lidt ({idt_ptr})",
        idt_ptr = in(reg) &idt_ptr,
        options(att_syntax),
        );

        // enable interrupts
        asm!("sti");
    }
}
