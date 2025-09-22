use crate::kernel::protected_main;
use crate::log;
use core::arch::asm;
use lazy_static::lazy_static;

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
    },
    GdtEntry {
        limit_low: 0xFFFF,
        base_low: 0,
        base_middle: 0,
        access: 0b10011010,
        granularity: 0b11001111,
        base_high: 0,
    },
    GdtEntry {
        limit_low: 0xFFFF,
        base_low: 0,
        base_middle: 0,
        access: 0b10010010,
        granularity: 0b11001111,
        base_high: 0,
    },
];

lazy_static! {
    static ref GDT_DESCRIPTOR: GdtDescriptorStruct = GdtDescriptorStruct {
        limit: size_of_val(&GDT_TABLE) as u16 - 1,
        base: GDT_TABLE.as_ptr() as u32,
    };
}

impl GdtDescriptor {
    pub fn load_gdt() {
        unsafe {
            asm!(
            "lgdt ({gdt_ptr})",
            gdt_ptr = in(reg) &*GDT_DESCRIPTOR,
            options(att_syntax, nostack, preserves_flags),
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
                "movl %cr0, %eax",
                "orl $1, %eax",
                "movl %eax, %cr0",
                options(att_syntax, nostack, preserves_flags),
            );

            asm!(
            "ljmp ${code_seg}, ${target}",
            code_seg = const 0x08u16,
            target = sym post_gdt,
            options(att_syntax, nostack, preserves_flags),
            );
        }
    }
}

#[allow(dead_code)]
pub extern "C" fn post_gdt() {
    log!("Gdt loaded successfully, entering protected mode");
    protected_main();
}
