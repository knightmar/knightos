
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MultibootInfo {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: [u32; 4],
    pub mmap_length: u32,
    pub mmap_addr: u32,
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name: u32,
    pub apm_table: u32,
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
    pub framebuffer_addr: u64, // Physical address
    pub framebuffer_pitch: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u8,
    pub framebuffer_type: u8, // 0 = Indexed, 1 = RGB, 2 = EGA Text
    pub color_info: [u8; 6],
}

const MAX_FRAME: usize = 32768;
pub struct BitMapPages {
    pub frame_map: [u32; MAX_FRAME], // 1 bit by mem page
}

pub static BITMAP_PAGE: Mutex<BitMapPages> = Mutex::new(BitMapPages {
    frame_map: [0xffffffff; MAX_FRAME],
});

impl BitMapPages {
    pub fn init(&mut self, multibootinfo_ptr: *const MultibootInfo) {
        unsafe {
            let info = &*multibootinfo_ptr;

            if (info.flags & (1 << 6)) == 0 {
                panic!("Bootloader did not provide a memory map!");
            }

            let mut ptr = info.mmap_addr;
            while ptr < (info.mmap_addr + info.mmap_length) {
                let map_entry = ptr as *const MemoryMapEntry;
                let size = core::ptr::read_unaligned(core::ptr::addr_of!((*map_entry).size));
                let m_type = core::ptr::read_unaligned(core::ptr::addr_of!((*map_entry).available));
                let addr = core::ptr::read_unaligned(core::ptr::addr_of!((*map_entry).base_addr));
                let len = core::ptr::read_unaligned(core::ptr::addr_of!((*map_entry).length));

                if m_type == 1 { // Available RAM
                    let start_page = (addr / 4096) as usize;
                    let page_count = (len / 4096) as usize;
                    for page in 0..page_count {
                        self.set_free(start_page + page);
                    }
                }
                ptr += size + 4;
            }

            unsafe extern "C" {
                static _kernel_start: u8;
                static _kernel_end: u8;
            }

            let start = &_kernel_start as *const u8 as usize;
            let end = &_kernel_end as *const u8 as usize;

            for page in (start / 4096)..((end + 4095) / 4096) {
                self.set_used(page);
            }

            self.set_used((&raw const PAGE_DIRECTORY as usize) / 4096);

            for page in 0..256 {
                self.set_used(page);
            }
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
        for index in 0..MAX_FRAME {
            let entry = self.frame_map[index];

            if entry != 0xFFFFFFFF {
                for bit in 0..32 {
                    if (entry & (1 << bit)) == 0 {
                        let page_index = index * 32 + bit;

                        self.set_used(page_index);

                        return Some((page_index as u32) * 4096);
                    }
                }
            }
        }

        None
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
