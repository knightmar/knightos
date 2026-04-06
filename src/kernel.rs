use crate::backend::descriptors::idt::load_idt;
use crate::backend::descriptors::pic::Pic;
use crate::backend::memory::init_heap;
use crate::backend::paging::init_paging;
use crate::backend::serial::Serial;
use crate::backend::wait;
use crate::user_interface::graphic_user_interface::{Color, GraphicsHelper};
use crate::user_interface::INPUT_SYSTEM;
use crate::{log, run_test};

include!("../ressources/image_data.rs");

pub fn protected_main() {
    log!("test");

    init_paging();

    unsafe {
        Pic::remap();
        Pic::init_timer();
    }
    unsafe { load_idt() }
    Serial::outb(0x21, 0xFC); // activate interrupts

    unsafe { init_heap() }

    unsafe {
        core::arch::asm!("sti");
    }

    let mut x_offset = 0;
    let mut y_offset = 0;
    let mut old_x = 1;
    let mut old_y = 1;

    let mut result = GraphicsHelper::new().unwrap();
    result.clear_screen();

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

        if old_x != x_offset || old_y != y_offset {
            for x in 0..20 {
                for y in 0..20 {
                    result.draw_pixel((10 + x + old_x, 10 + y + old_y).into(), Color::new(0, 0, 0));
                }
            }

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

        wait(1);

        old_x = x_offset;
        old_y = y_offset;
    }
    run_test();
}
