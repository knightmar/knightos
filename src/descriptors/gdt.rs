use crate::kernel::protected_main;
use crate::log;
use core::arch::asm;

#[repr(C, packed)]
struct GdtDescriptorStruct {
    limit: u16,
    base: u32,
}

pub struct GdtDescriptor {}

#[repr(C, packed)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

static GDT_TABLE: [GdtEntry; 3] = [
    GdtEntry {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
    }, // Null
    GdtEntry {
        limit_low: 0xFFFF,
        base_low: 0,
        base_middle: 0,
        access: 0x9A,
        granularity: 0xCF,
        base_high: 0,
    }, // Code
    GdtEntry {
        limit_low: 0xFFFF,
        base_low: 0,
        base_middle: 0,
        access: 0x92,
        granularity: 0xCF,
        base_high: 0,
    }, // Data
];

impl GdtDescriptor {
    pub fn load_gdt() {
        let descriptor = GdtDescriptorStruct {
            limit: (size_of::<[GdtEntry; 3]>() - 1) as u16,
            base: GDT_TABLE.as_ptr() as u32,
        };

        unsafe {
            asm!(
            "lgdt ({gdt_ptr})",
            gdt_ptr = in(reg) &descriptor,
            options(att_syntax),
            );

            asm!(
                "movw $0x10, %ax",
                "movw %ax, %ds",
                "movw %ax, %es",
                "movw %ax, %fs",
                "movw %ax, %gs",
                "movw %ax, %ss",
                options(att_syntax, nostack, preserves_flags),
            );

            asm!(
                "pushl $0x08",
                "pushl $1f",
                "lretl",
                "1:",
                options(att_syntax, nostack)
            );
        }
    }
}

#[allow(dead_code)]
pub extern "C" fn post_gdt() {
    log!("GDT loaded, segments refreshed.");
    protected_main();
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
