use crate::user_interface::utils::translate_keys;
use spin::mutex::Mutex;

pub mod graphic_user_interface;
mod utils;

pub static INPUT_SYSTEM: Mutex<InputSystem> = Mutex::new(InputSystem {
    keyboard_nav_event: KeyboardNavEvent {
        up: false,
        right: false,
        left: false,
        down: false,
    },
});

#[derive(Copy, Clone)]
pub struct InputSystem {
    pub keyboard_nav_event: KeyboardNavEvent,
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
            //
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

            // log!(format_args!("KEYBOARD SCANCODE: {:#x}", scancode));
            if scancode == 0x1 {
                panic!("Exiting");
            }
        }
        true
    }
}
