use bitfield_struct::bitfield;
use core::arch::asm;

#[bitfield(u32)]
struct PageEntry {
    present: bool,
    rw: bool,  // read / write
    us: bool,  // user / suppervisor
    pwt: bool, // write through
    pcd: bool,
    accessed: bool,  // set by cpu
    dirty: bool,     // set by cpu
    page_size: bool, //  0 = 4KB, 1 = 4MB
    #[bits(4)]
    empty: usize,
    #[bits(20)]
    frame_index: usize, // Store Address >> 12
}

#[repr(C, align(4096))]
struct PageTable {
    entries: [PageEntry; 1024],
}

static mut PAGE_DIRECTORY: PageTable = PageTable {
    entries: [PageEntry::from_bits(0); 1024],
};
static mut FIRST_PAGE_TABLE: PageTable = PageTable {
    entries: [PageEntry::from_bits(0); 1024],
};

pub fn init_paging() {
    unsafe {
        for i in 0..1024 {
            FIRST_PAGE_TABLE.entries[i] = PageEntry::default()
                .with_present(true)
                .with_rw(true)
                .with_us(false)
                .with_frame_index(i);
        }

        for i in 0..1024 {
            PAGE_DIRECTORY.entries[i] = PageEntry::default().with_rw(true)
        }

        let pt_address = &raw const FIRST_PAGE_TABLE as *const _ as usize;
        PAGE_DIRECTORY.entries[0] = PageEntry::default()
            .with_present(true)
            .with_rw(true)
            .with_us(false)
            .with_frame_index(pt_address >> 12);

        let pd_address = &raw const PAGE_DIRECTORY as *const _ as usize;
        asm!(
            "mov cr3, ${page_address}",
            page_address = in(reg) pd_address
        );

        asm!("mov eax, cr0", "or eax, 0x80000000", "mov cr0, eax",)
    }
}
