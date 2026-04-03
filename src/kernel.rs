use crate::backend::descriptors::idt::load_idt;
use crate::backend::descriptors::pic::Pic;
use crate::backend::memory::init_heap;
use crate::backend::memory::vmm::MemMapper;
use crate::backend::paging::init_paging;
use crate::backend::serial::Serial;
use crate::user_interface::INPUT_SYSTEM;
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

    let mut x_offset = 0;
    let mut y_offset = 0;
    let mut result = GraphicsHelper::new().unwrap();
    loop {

        let input = *INPUT_SYSTEM.lock();
        let speed = 5;

        if input.keyboard_nav_event.right {
            x_offset += speed;
        }
        if input.keyboard_nav_event.left && x_offset > 0 {
            x_offset -= speed;
        }
        if input.keyboard_nav_event.up && y_offset > 0 {
            y_offset -= speed;
        }
        if input.keyboard_nav_event.down {
            y_offset += speed;
        }

        result.clear_screen();

        for x in 0..20 {
            for y in 0..20 {
                result.draw_pixel(
                    (10 + x + x_offset, 10 + y + y_offset).into(),
                    Color::new(255, 0, 0),
                );
            }
        }

        result.flush();
    }
    run_test();
}
