use crate::backend::vga::colors::VGAColors;
use crate::backend::vga::colors::VGAColors::*;
use crate::get_colors;
use crate::user_interface::text_user_interface::TUI;
use core::fmt;
use core::fmt::Write;

pub(crate) mod colors;
mod macros;

pub fn force_unlock() {
    unsafe {
        TUI.force_unlock();
    }
}

unsafe impl Send for VGAText {}
unsafe impl Sync for VGAText {}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let _ = TUI.lock().vga_text.write_fmt(args);
}

pub struct Pointer {
    x: u32,
    y: u32,
    max_x: u32,
    max_y: u32,
    color: VGAColors,
}

pub struct VGAText {
    pub pointer: Pointer,
    color: u8,
    buffer: *mut u8,
}

impl VGAText {
    pub fn new() -> Self {
        let mut text = VGAText {
            pointer: Pointer::new(),
            color: get_colors!(White, Black),
            buffer: 0xb8000 as *mut u8,
        };

        text.clear_screen();
        text
    }

    pub fn change_color(&mut self, foreground_color: VGAColors, background_color: VGAColors) {
        self.color = get_colors!(foreground_color, background_color);
    }

    pub fn change_fg_color(&mut self, color: VGAColors) {
        self.color = (self.color & 0b11110000) | (color as u8);
    }

    pub fn change_bg_color(&mut self, color: VGAColors) {
        self.color = (self.color & 0b00001111) | ((color as u8) << 4);
    }

    pub fn set_color(&mut self, color: u8) {
        self.color = color;
    }

    pub fn clear_screen(&mut self) {
        self.pointer.reset_pos();
        let old_color = self.color;
        self.change_color(Black, Black);
        for i in 0..(self.pointer.max_x * self.pointer.max_y) {
            unsafe {
                *self.buffer.offset((i * 2) as isize) = b' ';
                *self.buffer.offset((i * 2 + 1) as isize) = self.color;
            }
        }
        self.set_color(old_color);
        self.pointer.reset_pos();
    }

    pub fn write_str(&mut self, str: &str) {
        for chr in str.as_bytes() {
            if chr.is_ascii_alphanumeric() || !chr.is_ascii_control() || b' '.eq(chr) {
                self.write_chr(char::from(*chr));
            } else if b'\n'.eq(chr) {
                self.pointer.new_line();
            } else {
                self.write_chr('�');
            }
        }
    }

    pub fn write_chr(&mut self, chr: char) {
        unsafe {
            *self
                .buffer
                .offset(((self.pointer.max_x * self.pointer.y + self.pointer.x) * 2) as isize) =
                chr as u8;
            *self.buffer.offset(
                ((self.pointer.max_x * self.pointer.y + self.pointer.x) * 2 + 1) as isize,
            ) = self.color;
        }
        self.pointer.move_right();
        self.change_color_current_ptr(Red);
    }

    pub fn change_color_current_ptr(&mut self, color: VGAColors) {
        unsafe {
            *self.buffer.offset(
                ((self.pointer.max_x * self.pointer.y + self.pointer.x) * 2 + 1) as isize,
            ) = get_colors!(White, color);
        }
    }
}

impl Write for VGAText {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

impl Pointer {
    pub fn new() -> Self {
        Pointer {
            x: 0,
            y: 0,
            max_x: 80,
            max_y: 25,
            color: Red,
        }
    }

    pub fn reset_pos(&mut self) {
        self.x = 0;
        self.y = 0;
    }
    pub fn move_right(&mut self) {
        if self.x + 1 >= self.max_x {
            self.x = 0;
            if self.y + 1 >= self.max_y {
                self.y = 0;
            } else {
                self.y += 1;
            }
        } else {
            self.x += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        } else {
            if self.y > 0 {
                self.y -= 1;
                self.x = self.max_x;
            } else {
                self.y = 0;
                self.x = 0;
            }
        }
    }

    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.y + 1 <= self.max_y {
            self.y += 1;
        }
    }

    pub fn new_line(&mut self) {
        self.x = 0;
        if self.y + 1 >= self.max_y {
            self.y = 0;
        } else {
            self.y += 1;
        }
    }
}
