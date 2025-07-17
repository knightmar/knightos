use crate::vga::colors::VGAColors;
use crate::vga::colors::VGAColors::*;
use lazy_static::lazy_static;
use spin::Mutex;

pub(crate) mod colors;

#[macro_export]
macro_rules! get_colors {
    ($foreground:expr, $background:expr) => {
        ($foreground as u8) << 4 | (($background as u8) & 0b01111111)
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => {
        _print!("\n")
    };
    ($($arg:tt)*) => {{ _print!("{}\n", $arg) }};
}

#[doc(hidden)]
pub fn _print(str: &str) {
    Writer.lock().write_str(str);
}

unsafe impl Send for VGAText {}
unsafe impl Sync for VGAText {}

lazy_static! {
    pub static ref Writer: Mutex<VGAText> = Mutex::new(VGAText {
        pointer: Pointer {
            x: 0,
            y: 0,
            max_x: 80,
            max_y: 25,
        },
        color: get_colors!(Black, White),
        buffer: 0xb8000 as *mut u8,
    });
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
        VGAText {
            pointer: Pointer::new(),
            color: get_colors!(Black, White),
            buffer: 0xb8000 as *mut u8,
        }
    }

    pub fn change_color(&mut self, foreground_color: VGAColors, background_color: VGAColors) {
        self.color = get_colors!(foreground_color, background_color);
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
            self.write_chr(char::from(*chr));
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
}
