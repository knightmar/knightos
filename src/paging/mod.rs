use bitfield_struct::bitfield;
use core::arch::asm;

#[repr(C, align(4096))]
#[bitfield(u32)]
struct PageTable {
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
    physical_address: usize,
}

pub fn init_paging() {
    let mut page_directory: [PageTable; 1024] = [PageTable::default()
        .with_present(false)
        .with_rw(true)
        .with_us(true); 1024];

    let mut first_page_table: [PageTable; 1024] = [PageTable::default(); 1024];
    for i in 0..1024 {
        let entry = &mut first_page_table[i];
        *entry = PageTable::default()
            .with_present(true)
            .with_rw(true)
            .with_us(true)
            .with_physical_address(i);
    }

    page_directory[0] = PageTable::default()
        .with_present(true)
        .with_physical_address(first_page_table.as_ptr() as usize);

    unsafe {
        asm!(
            "mov cr3, ${page_address}",
            page_address = in(reg) page_directory.as_ptr()
        );

        asm!(
            "mov eax, cr0",
            "or eax, 0x80000000",
            "mov cr0, eax",
        )
    }
}
