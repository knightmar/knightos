use crate::backend::descriptors::idt::load_idt;
use crate::backend::descriptors::pic::Pic;
use crate::backend::memory::init_heap;
use crate::backend::memory::vmm::MemMapper;
use crate::backend::paging::init_paging;
use crate::backend::serial::Serial;
use crate::img::IMAGE_DATA;
use crate::{BOOT_CONFIG, log, run_test};

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
                if x < 467 && y < 616 {
                    let pixel_offset = (y * mbi.fb_pitch) + (x * (mbi.fb_bpp as u32 / 8));
                    let triplet = IMAGE_DATA[y as usize][x as usize];
                    unsafe {
                        *fb_ptr.add(pixel_offset as usize) = triplet.2; // Blue
                        *fb_ptr.add(pixel_offset as usize + 1) = triplet.1; // Green
                        *fb_ptr.add(pixel_offset as usize + 2) = triplet.0; // Red
                    }
                }
            }
        }
    }

    drop(guard);
    run_test();
}
