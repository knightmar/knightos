use alloc::vec;
use alloc::vec::Vec;
use core::ops::Not;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::backend::serial::LogLevel::Info;
use crate::log;

const FONT: &[u8; 4096] = include_bytes!("../../../resources/font.bin");
const CHAR_WIDTH: usize = 8;
const CHAR_HEIGHT: usize = 16;
const NB_CHAR_BY_LINE: usize = 230;

lazy_static! {
    pub static ref TEXT_MANAGER: Mutex<TextManager> = Mutex::new(TextManager::new());
}

pub struct TextManager {
    text_lines: Vec<[char; NB_CHAR_BY_LINE]>,
    current_line_char_count: usize,
}

impl TextManager {
    pub fn lookup_char(char: char) -> &'static [u8; 16] {
        let ascii_val = (char as usize).min(255); // Fallback boundary cap
        let start_offset = ascii_val * 16;

        FONT[start_offset..start_offset + 16]
            .try_into()
            .unwrap_or_else(|_| &[0; 16])
    }

    pub fn add_char(&mut self, char: char) {
        if char.is_ascii_alphanumeric().not() {
            return;
        }

        if self.current_line_char_count < NB_CHAR_BY_LINE
            && let Some(line) = self.text_lines.last_mut()
        {
            line[self.current_line_char_count] = char;

            self.current_line_char_count += 1;
        } else {
            let mut line = [0 as char; NB_CHAR_BY_LINE];
            line[0] = char;
            self.text_lines.push(line);
            self.current_line_char_count = 1;
        }
    }

    pub fn new() -> Self {
        Self {
            text_lines: Vec::new(),
            current_line_char_count: 0,
        }
    }

    pub fn text_lines(&self) -> &Vec<[char; 230]> {
        &self.text_lines
    }

    pub fn current_line_char_count(&self) -> usize {
        self.current_line_char_count
    }
}
