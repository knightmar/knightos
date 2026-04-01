use crate::backend::descriptors::idt::load_idt;
use crate::backend::descriptors::pic::Pic;
use crate::backend::memory::init_heap;
use crate::backend::memory::pmm::BITMAP_PAGE;
use crate::backend::memory::vmm::MemMapper;
use crate::backend::paging::init_paging;
use crate::backend::serial::LogLevel::Info;
use crate::backend::serial::Serial;
use crate::backend::{qemu_shutdown, wait};
use crate::{BOOT_CONFIG, log, println, run_test};
use alloc::boxed::Box;
use alloc::vec::Vec;

pub fn protected_main() {
    log!("test");

    init_paging();

    unsafe { Pic::remap() }
    unsafe { load_idt() }
    Serial::outb(0x21, 0xFC); // activate interrupts
    unsafe { init_heap() }

    unsafe {
        core::arch::asm!("sti");
    }
    let guard = BOOT_CONFIG.lock();
    if let Some(mbi) = *guard {
        let fb_phys = mbi.fb_addr as u32;
        let fb_size = mbi.fb_pitch * mbi.fb_height;

        let fb_virt = 0x40000000;

        unsafe {
            MemMapper::map_range(fb_virt, fb_phys, fb_size, 3);
        }

        let fb_ptr = fb_virt as *mut u8;

        for y in 0..mbi.fb_height {
            for x in 0..mbi.fb_width {
                let pixel_offset = (y * mbi.fb_pitch) + (x * (mbi.fb_bpp as u32 / 8));
                unsafe {
                    *fb_ptr.add(pixel_offset as usize) = 255_i32.wrapping_add((x * y) as i32) as u8; // Blue
                    *fb_ptr.add(pixel_offset as usize + 1) = 100_i32.wrapping_add((x + y) as i32) as u8; // Green
                    *fb_ptr.add(pixel_offset as usize + 2) = 50_i32.wrapping_add((x / (y+1) ) as i32) as u8; // Red
                }
            }
        }
    }

    drop(guard);
    run_test();
}
