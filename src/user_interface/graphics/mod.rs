use crate::backend::interrupts::utils::no_int_runner;
use crate::backend::serial::LogLevel::Info;
use crate::log;
use crate::user_interface::graphics::helper::{GRAPHICS_HELPER, Point};
use crate::user_interface::graphics::text::TEXT_MANAGER;
use core::arch::asm;
use core::ops::Not;

pub mod helper;
pub mod text;
pub fn render_task() {
    log!(Info, "RENDER TASK STARTING");
    loop {
        let lines = no_int_runner(|| {
            let text = TEXT_MANAGER.lock();
            text.text_lines().clone()
        });

        if lines.is_empty() {
            unsafe { asm!("hlt") };
            continue;
        }

        let mut char_point: Point = (0, 0).into();

        no_int_runner(|| {
            let mut graphics = GRAPHICS_HELPER.lock();

            let max_height = graphics.boot_config.fb_height;

            for line in &lines {
                if char_point.y + 16 >= max_height {
                    break;
                }

                char_point.x = 0;

                for &char in line {
                    if char == '\0' || char == 0 as char {
                        continue;
                    }
                    graphics.print_char(char, &char_point);
                    char_point.x += 8;
                }
                char_point.y += 16;
            }

            graphics.flush();
        });

        unsafe { asm!("hlt") }
    }
}
