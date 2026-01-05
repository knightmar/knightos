use crate::backend::vga::colors::VGAColors;
use crate::backend::vga::colors::VGAColors::{Black, Red};
use crate::backend::vga::VGAText;
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
        Self {
            vga_text: VGAText::new(),
            keyboard_nav_event: KeyboardNavEvent::default(),
            ptr_color: Red,
        }
    }

    pub fn update_ptr_color() {}

    pub fn on_keyboard_nav(&mut self) {
        if self.keyboard_nav_event.up
            || self.keyboard_nav_event.down
            || self.keyboard_nav_event.left
            || self.keyboard_nav_event.right
        {
            self.vga_text.change_color_current_ptr(Black);

            match &self.keyboard_nav_event {
                a if a.left => self.vga_text.pointer.move_prev_pos(),
                a if a.right => self.vga_text.pointer.move_next_pos(),
                _ => {}
            }
            self.vga_text.change_color_current_ptr(Red);
        }
    }
}
