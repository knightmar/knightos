mod utils;

use crate::backend::interrupts::TICK_COUNT;
use crate::backend::serial::Serial;
use crate::backend::vga::colors::VGAColors;
use crate::backend::vga::colors::VGAColors::{Black, Red};
use crate::backend::vga::{Pointer, VGAText};
use crate::user_interface::text_user_interface::utils::translate_keys;
use crate::{log, print, println};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref TUI: Mutex<TextUserInterface> = Mutex::new(TextUserInterface::new());
}

#[derive(Debug)]
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

pub struct TextUserInterface {
    pub vga_text: VGAText,
    pub(crate) keyboard_nav_event: KeyboardNavEvent,
    pub ptr_color: VGAColors,
}

impl TextUserInterface {
    pub fn new() -> Self {
        let mut text = VGAText::new();
        text.pointer = Pointer::new();
        Self::disable_cursor();
        text.pointer.set_max_y(25);
        Self {
            vga_text: text,
            keyboard_nav_event: KeyboardNavEvent::default(),
            ptr_color: Red,
        }
    }

    pub fn disable_cursor() {
        Serial::outb(0x3D4, 0x0A);
        Serial::outb(0x3D5, 0x20);
    }

    pub fn on_keyboard_event(&mut self, scancode: u8) -> bool {
        let key = translate_keys(scancode);
        if key != '\0' {
            print!(self.vga_text, "{}", key);
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
                0x52 => {
                    let count = *TICK_COUNT.lock();
                    println!(self.vga_text, "{}", count);
                }
                0x4f => {
                    self.vga_text.clear_screen();
                }
                0xe => {
                    self.vga_text.delete_char();
                }
                _ => {}
            }

            // log!(format_args!("KEYBOARD SCANCODE: {:#x}", scancode));
            if scancode == 0x1 {
                panic!("Exiting");
            }
        }
        true
    }

    pub fn on_keyboard_nav(&mut self) {
        if self.keyboard_nav_event.up
            || self.keyboard_nav_event.down
            || self.keyboard_nav_event.left
            || self.keyboard_nav_event.right
        {
            self.vga_text.change_color_current_ptr(Black);

            match &self.keyboard_nav_event {
                a if a.left => self.vga_text.pointer.move_left(),
                a if a.right => self.vga_text.pointer.move_right(),
                a if a.up => self.vga_text.pointer.move_up(),
                a if a.down => self.vga_text.pointer.move_down(),
                _ => {}
            }
            self.vga_text.change_color_current_ptr(Red);
        }
    }
}
