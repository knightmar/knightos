const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub struct Display {
    screen_content: [[char; BUFFER_WIDTH]; BUFFER_HEIGHT],
    current_position: (usize, usize),
    buffer: *mut u8,
}

impl Display {
    pub fn print_chr(&mut self, chr: char, color: u8) {
        if self.current_position.0 >= BUFFER_WIDTH {
            self.current_position.0 = 0;
            self.current_position.1 += 1; // Move to the next row
        }
        if self.current_position.1 >= BUFFER_HEIGHT {
            self.current_position.1 = 0; // Wrap to the top
        }

        // Update screen content
        self.screen_content[self.current_position.1][self.current_position.0] = chr;

        // Calculate the buffer offset
        let offset = (self.current_position.1 * BUFFER_WIDTH + self.current_position.0) * 2;

        unsafe {
            *(self.buffer.offset(offset as isize)) = chr as u8;
            *(self.buffer.offset(offset as isize + 1)) = color;
        }

        self.current_position.0 += 1;
    }

    pub fn print_str(&mut self, content: &str, color: u8) {
        content.chars().for_each(|chr| self.print_chr(chr, color))
    }

    pub fn new() -> Self {
        Self {
            screen_content: [[0 as char; BUFFER_WIDTH]; BUFFER_HEIGHT],
            current_position: (0, 0),
            buffer: VGA_BUFFER,
        }
    }

    pub fn get_color(bg: Color, fg: Color) -> u8 {
        (bg as u8) << 4 | fg as u8
    }

    pub fn set_current_position(&mut self, current_position: (usize, usize)) {
        self.current_position = current_position;
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}
