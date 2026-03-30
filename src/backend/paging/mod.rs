use bitfield_struct::bitfield;
use core::arch::asm;

#[bitfield(u32)]
pub struct PageEntry {
    pub present: bool,
    pub rw: bool,  // read / write
    pub us: bool,  // user / suppervisor
    pub pwt: bool, // write through
    pub pcd: bool,
    pub accessed: bool,  // set by cpu
    pub dirty: bool,     // set by cpu
    pub page_size: bool, //  0 = 4KB, 1 = 4MB
    #[bits(4)]
    pub empty: usize,
    #[bits(20)]
    pub frame_index: usize, // Store Address >> 12
}

#[repr(C, align(4096))]
#[derive(Copy)]
#[derive(Clone)]
pub struct PageTable {
    pub entries: [PageEntry; 1024],
}

pub static mut PAGE_DIRECTORY: PageTable = PageTable {
    entries: [PageEntry::from_bits(0); 1024],
};
static mut FIRSTS_PAGES_TABLES: [PageTable; 32] = [
    PageTable { entries: [PageEntry::from_bits(0); 1024] }; 32
];

pub fn init_paging() {
    unsafe {
        for x in 0..32 {
            for i in 0..1024 {
                FIRSTS_PAGES_TABLES[x].entries[i] = PageEntry::default()
                    .with_present(true)
                    .with_rw(true)
                    .with_us(false)
                    .with_frame_index(x* 1024 + i);
            }
        }

        for i in 0..1024 {
            PAGE_DIRECTORY.entries[i] = PageEntry::default().with_rw(true)
        }

        let mut pt_address = &raw const FIRSTS_PAGES_TABLES as *const _ as usize;

        for page in 0..32 {
            PAGE_DIRECTORY.entries[page] = PageEntry::default()
                .with_present(true)
                .with_rw(true)
                .with_us(false)
                .with_frame_index(pt_address >> 12);



            pt_address += size_of::<PageTable>();
        }

        let pd_address = &raw const PAGE_DIRECTORY as *const _ as usize;
        asm!(
        "mov cr3, {page_address}",
        page_address = in(reg) pd_address
        );

        asm!("mov eax, cr0", "or eax, 0x80000000", "mov cr0, eax",)
    }
}
