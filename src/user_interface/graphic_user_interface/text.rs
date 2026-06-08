const FONT: &[u8; 4096] = include_bytes!("../../../resources/font.bin");
const CHAR_WIDTH: usize = 8;
const CHAR_HEIGHT: usize = 16;

pub struct TextManager {}

impl TextManager {
    pub fn lookup_char(char: char) -> &'static [u8; 16] {
        let ascii_val = (char as usize).min(255); // Fallback boundary cap
        let start_offset = ascii_val * 16;

        FONT[start_offset..start_offset + 16].try_into().unwrap_or_else(|_| &[0; 16])
    }
}
