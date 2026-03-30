use crate::backend::memory::memset_u32;
use crate::backend::memory::pmm::BITMAP_PAGE;
use crate::backend::paging::{PAGE_DIRECTORY, PageEntry};
use core::arch::asm;

pub struct MemMapper {}

impl MemMapper {
    pub unsafe fn mem_map(vaddr: u32, paddr: u32, flags: u32) {
        let offset = vaddr & 0xFFF;
        let table_index = (vaddr >> 12 & 0x3FF);
        let directory_index = (vaddr >> 22 & 0x3FF) as usize;

        if !PAGE_DIRECTORY.entries[directory_index].present() {
            if let Some(mut option) = BITMAP_PAGE.lock().alloc_frame() {
                let pt_phys = option as *mut u32;
                memset_u32(pt_phys, 0, 1024);

                PAGE_DIRECTORY.entries[directory_index] = PageEntry::new()
                    .with_present(true)
                    .with_rw(true)
                    .with_frame_index((option >> 12) as usize);
            };
        };

        let pt_phys = PAGE_DIRECTORY.entries[directory_index].frame_index() << 12;
        let table = pt_phys as *mut PageEntry;

        unsafe {
            *(table.add(table_index as usize)) = PageEntry::new() // add -> *(table + table_index) = ...
                .with_frame_index((paddr >> 12) as usize)
                .with_rw(true)
                .with_present(true);
        }

        asm!("invlpg [{}]", in(reg) vaddr);
    }
}
