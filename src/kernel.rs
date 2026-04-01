use crate::backend::descriptors::idt::load_idt;
use crate::backend::descriptors::pic::Pic;
use crate::backend::memory::init_heap;
use crate::backend::memory::vmm::MemMapper;
use crate::backend::paging::init_paging;
use crate::backend::serial::Serial;
use crate::user_interface::graphic_user_interface::{Color, GraphicsHelper};
use crate::{BOOT_CONFIG, log, run_test};

include!("../ressources/image_data.rs");

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

    let result = GraphicsHelper::new().unwrap();
    for x in 0..100 {
        for y in 0..100 {
            result.draw_pixel(10 + x, 10 + y, Color::new(255, 0, 0));
        }
    }

    run_test();
}
