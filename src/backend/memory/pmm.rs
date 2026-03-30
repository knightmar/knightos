
use crate::backend::paging::{PAGE_DIRECTORY, PageEntry};
use crate::backend::serial::LogLevel::Info;
use crate::{log, println};
use bitfield_struct::bitfield;
use core::alloc::{AllocError, Allocator, Layout};
use core::arch::asm;
use core::error::Error;
use core::ptr::NonNull;
use spin::Mutex;

#[repr(C, packed)]
struct MemoryMapEntry {
    pub size: u32,
    pub base_addr: u64,
    pub length: u64,
    pub available: u32,
}

#[repr(C)]
struct MultibootInfo {
    flags: u32,
    mem_lower: u32,
    mem_upper: u32,
    boot_device: u32,
    cmdline: u32,
    mods_count: u32,
    mods_addr: u32,
    syms: [u32; 4],
    mmap_length: u32,
    mmap_addr: u32,
}

const MAX_FRAME: usize = 32768;
pub struct BitMapPages {
    pub frame_map: [u32; MAX_FRAME], // 1 bit by mem page
}

pub static BITMAP_PAGE: Mutex<BitMapPages> = Mutex::new(BitMapPages {
    frame_map: [0xffffffff; MAX_FRAME],
});

impl BitMapPages {
    pub fn init(&mut self, multibootinfo_ptr: usize) {
        unsafe {
            // get mem size from bootloader
            // init all frame map depending on which are used

            let info = multibootinfo_ptr as *const MultibootInfo;
            let mut ptr = (*info).mmap_addr;
            while (ptr) < ((*info).mmap_addr + (*info).mmap_length) {
                let map_entry = ptr as *const MemoryMapEntry;
                let size = core::ptr::read_unaligned(core::ptr::addr_of!((*map_entry).size));
                let available =
                    core::ptr::read_unaligned(core::ptr::addr_of!((*map_entry).available));
                let addr = core::ptr::read_unaligned(core::ptr::addr_of!((*map_entry).base_addr));

                if available == 1 {
                    let start_page = addr / 4096;
                    let page_count = (*map_entry).length / 4096;
                    for page in 0..page_count {
                        self.set_free((start_page + page as u64) as usize)
                    }
                }

                ptr += size + 4;
            }

            // manual values
            self.set_used(0);
            // kernel mem
            unsafe extern "C" {
                static _kernel_end: u8;
            }

            let end_idx = (&_kernel_end as *const u8 as usize) / 4096;
            for page in (0x100000 / 4096)..end_idx {
                self.set_used(page)
            }
            // mem map
            let mmap_addr = (*info).mmap_addr;
            let mmap_count = ((*info).mmap_length + 4095) / 4096;
            for p in 0..mmap_count {
                self.set_used(((mmap_addr / 4096) + p) as usize);
            }

            // multiboot info
            self.set_used(multibootinfo_ptr / 4096);
        }
    }

    pub fn set_used(&mut self, frame_index: usize) {
        if frame_index < MAX_FRAME * 32 {
            let index = frame_index / 32;
            let bit_pos = frame_index % 32;
            self.frame_map[index] |= (1 << bit_pos);
        }
    }

    pub fn set_free(&mut self, frame_index: usize) {
        if frame_index < MAX_FRAME * 32 {
            let index = frame_index / 32;
            let bit_pos = frame_index % 32;
            self.frame_map[index] &= !(1 << bit_pos);
        }
    }

    pub fn is_used(&self, frame_index: usize) -> bool {
        let array_idx = frame_index / 32;
        let bit_pos = frame_index % 32;
        (self.frame_map[array_idx] & (1 << bit_pos)) != 0
    }

    pub fn alloc_frame(&mut self) -> Option<u32> {
        let mut index: usize = 0;
        while self.frame_map[index] == 0xFFFFFFFF && index < MAX_FRAME {
            index += 1;
        }

        if index >= MAX_FRAME || self.frame_map[index] == 0xFFFFFFFF {
            return None;
        }

        let bit_index = self.frame_map[index].trailing_ones();
        let page_index = index as u32 * 32 + bit_index;
        self.set_used(page_index as usize);

        Some(page_index * 4096)
    }

    pub fn free_frame(&mut self, address: u32) -> Result<(), AllocError> {
        let index = address / 4096;
        if index > MAX_FRAME as u32 * 32 {
            return Err(AllocError);
        }

        self.set_free(index as usize);

        Ok(())
    }
}
