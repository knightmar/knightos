use crate::backend::serial::LogLevel::Info;
use crate::log;
use crate::user_interface::graphic_user_interface::{GRAPHICS_HELPER, Point};
use crate::user_interface::utils::translate_keys;
use spin::mutex::Mutex;

pub mod graphic_user_interface;
pub mod utils;

pub static INPUT_SYSTEM: Mutex<InputSystem> = Mutex::new(InputSystem {
    keyboard_nav_event: KeyboardNavEvent {
        up: false,
        right: false,
        left: false,
        down: false,
    },
    char_cursor_pos: Point { x: 0, y: 0 },
});

#[derive(Copy, Clone)]
pub struct InputSystem {
    pub keyboard_nav_event: KeyboardNavEvent,
    pub char_cursor_pos: Point,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyboardNavEvent {
    pub(crate) up: bool,
    pub(crate) right: bool,
    pub(crate) left: bool,
    pub(crate) down: bool,
}

impl Default for KeyboardNavEvent {
    fn default() -> Self {
        Self {
            up: false,
            right: false,
            left: false,
            down: false,
        }
    }
}

impl InputSystem {
    pub fn on_keyboard_event(&mut self, scancode: u8) -> bool {
        let key = translate_keys(scancode);
        if key != '\0' {
            log!(Info, "{}", key);
            let mut guard = GRAPHICS_HELPER.lock();
            guard.print_char(key, &self.char_cursor_pos);
            guard.draw_line((100, 200).into(), (500, 400).into(), (255, 0, 0).into());
            guard.flush();
            self.char_cursor_pos += Point::new(8, 0);
        } else {
            match scancode {
                // Press events
                0x48 => self.keyboard_nav_event.up = true,
                0x50 => self.keyboard_nav_event.down = true,
                0x4B => self.keyboard_nav_event.left = true,
                0x4D => self.keyboard_nav_event.right = true,
                // Release events (scancode + 0x80)
                0xC8 => self.keyboard_nav_event.up = false,
                0xD0 => self.keyboard_nav_event.down = false,
                0xCB => self.keyboard_nav_event.left = false,
                0xCD => self.keyboard_nav_event.right = false,
                _ => {}
            }

            if scancode == 0x1 {
                panic!("Exiting");
            }
        }
        true
    }
}
