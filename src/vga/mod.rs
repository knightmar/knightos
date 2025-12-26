use crate::serial::LogLevel;
use crate::vga::colors::VGAColors;
use crate::vga::colors::VGAColors::*;
use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;

pub(crate) mod colors;
#[macro_export]
macro_rules! get_colors {
    ($foreground:expr, $background:expr) => {
        ($background as u8) << 4 | (($foreground as u8) & 0b01111111)
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::vga::_print(format_args!($($arg)*));
        $crate::log!($crate::serial::LogLevel::Info, $($arg)*);
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*));
    };
}

unsafe impl Send for VGAText {}
unsafe impl Sync for VGAText {}

lazy_static! {
    pub static ref WRITER: Mutex<VGAText> = Mutex::new(VGAText::new());
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let _ = WRITER.lock().write_fmt(args);
}

struct Pointer {
    x: u32,
    y: u32,
    max_x: u32,
    max_y: u32,
}

pub struct VGAText {
    pointer: Pointer,
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
        self.pointer.move_next_pos();
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
        }
    }

    pub fn reset_pos(&mut self) {
        self.x = 0;
        self.y = 0;
    }
    pub fn move_next_pos(&mut self) {
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

    pub fn new_line(&mut self) {
        self.x = 0;
        if self.y + 1 >= self.max_y {
            self.y = 0;
        } else {
            self.y += 1;
        }
    }
}
