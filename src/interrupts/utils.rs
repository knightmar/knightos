use core::arch::asm;

pub fn translate_keys(scan_code: u8) -> char {
    match scan_code {
        0x02 => '1',
        0x03 => '2',
        0x04 => '3',
        0x05 => '4',
        0x06 => '5',
        0x07 => '6',
        0x08 => '7',
        0x09 => '8',
        0x0a => '9',
        0x0b => '0',
        0x10 => 'a',
        0x11 => 'z',
        0x12 => 'e',
        0x13 => 'r',
        0x14 => 't',
        0x15 => 'y',
        0x16 => 'u',
        0x17 => 'i',
        0x18 => 'o',
        0x19 => 'p',
        0x1e => 'q',
        0x1f => 's',
        0x20 => 'd',
        0x21 => 'f',
        0x22 => 'g',
        0x23 => 'h',
        0x24 => 'j',
        0x25 => 'k',
        0x26 => 'l',
        0x27 => 'm',
        0x2c => 'w',
        0x2d => 'x',
        0x2e => 'c',
        0x2f => 'v',
        0x30 => 'b',
        0x31 => 'n',
        0x39 => ' ',
        _ => '\0',
    }
}

pub fn hlt_loop() {
    loop {
        unsafe { asm!("hlt") }
    }
}
