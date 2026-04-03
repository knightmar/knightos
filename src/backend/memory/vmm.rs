use crate::backend::memory::memset_u32;
use crate::backend::memory::pmm::BITMAP_PAGE;
use crate::backend::paging::{PAGE_DIRECTORY, PageEntry};
use core::arch::asm;

pub struct MemMapper {}

impl MemMapper {
    pub unsafe fn mem_map(vaddr: u32, paddr: u32, flags: u32) {
        let dir_idx = (vaddr >> 22) as usize;
        let table_idx = (vaddr >> 12 & 0x3FF) as usize;

        let recursive_pd = 0xFFFFF000 as *mut PageEntry;
        let table_vaddr = (0xFFC00000 + (dir_idx * 4096)) as *mut PageEntry;

        // 1. If the Page Table for this directory index doesn't exist, create it
        if !(*recursive_pd.add(dir_idx)).present() {
            if let Some(phys_frame) = BITMAP_PAGE.lock().alloc_frame() {
                // Link new table to Directory
                *recursive_pd.add(dir_idx) = PageEntry::new()
                    .with_present(true)
                    .with_rw(true)
                    .with_frame_index((phys_frame >> 12) as usize);

                // CRITICAL: Flush the TLB for the TABLE VIRTUAL ADDRESS
                // This allows the CPU to see the new table at 0xFFCxxxxx
                asm!("invlpg [{}]", in(reg) table_vaddr);

                // Zero out the entire 4KB page (1024 PageEntries)
                core::ptr::write_bytes(table_vaddr as *mut u8, 0, 4096);
            } else {
                panic!("CRITICAL: Out of physical memory while creating Page Table");
            }
        }

        // 2. Map the actual page in the Page Table
        *table_vaddr.add(table_idx) = PageEntry::new()
            .with_frame_index((paddr >> 12) as usize)
            .with_rw(flags >> 1 & 0b1 == 1)
            .with_present(flags & 0b1 == 1);

        // 3. Flush the TLB for the actual target virtual address
        asm!("invlpg [{}]", in(reg) vaddr);
    }

    pub fn map_range(vaddr_start: u32, paddr_start: u32, size: u32, flags: u32) {
        unsafe {
            let start = vaddr_start & !0xFFF;
            let p_start = paddr_start & !0xFFF;

            for offset in (0..size).step_by(4096) {
                Self::mem_map(start + offset, p_start + offset, flags);
            }
        }
    }
}
