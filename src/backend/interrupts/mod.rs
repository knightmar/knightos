mod utils;

use crate::backend::serial::Serial;
use crate::backend::vga::colors::VGAColors::Red;
use crate::backend::{serial, vga};
use crate::{log, print, println};
use core::arch::asm;
use crate::user_interface::text_user_interface::TUI;

#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u32,
    pub code_segment: u32,
    pub cpu_flags: u32,
}

pub extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    log!(format_args!(
        "Breakpoint at {:#x}",
        frame.instruction_pointer
    ));
}

pub extern "x86-interrupt" fn timer_handler(_frame: InterruptStackFrame) {
    if let Some(mut tui) = crate::user_interface::text_user_interface::TUI.try_lock() {
        tui.on_keyboard_nav();
    }
    Serial::outb(0x20, 0x20);
}

pub extern "x86-interrupt" fn double_fault_handler(
    frame: InterruptStackFrame,
    error_code: u32,
) -> ! {
    panic!(
        "DOUBLE FAULT: code {}, at {:#x}",
        error_code, frame.instruction_pointer
    );
}

pub extern "x86-interrupt" fn keyboard_handler(_frame: InterruptStackFrame) {
    let scancode = Serial::inb(0x60);

    let key = utils::translate_keys(scancode);
    if key != '\0' {
        print!("{}", key);
    } else {
        let mut tui = crate::user_interface::text_user_interface::TUI.lock();

        match scancode {
            // Press events
            0x48 => tui.keyboard_nav_event.up = true,
            0x50 => tui.keyboard_nav_event.down = true,
            0x4B => tui.keyboard_nav_event.left = true,
            0x4D => tui.keyboard_nav_event.right = true,
            // Release events (scancode + 0x80)
            0xC8 => tui.keyboard_nav_event.up = false,
            0xD0 => tui.keyboard_nav_event.down = false,
            0xCB => tui.keyboard_nav_event.left = false,
            0xCD => tui.keyboard_nav_event.right = false,
            _ => {}
        }

        log!(format_args!("KEYBOARD SCANCODE: {:#x}", scancode));
        if scancode == 0x1 {
            panic!("Exiting");
        }
    }

    // end of interrupt
    Serial::outb(0x20, 0x20);
}

pub extern "x86-interrupt" fn page_fault_handler(_frame: InterruptStackFrame, error_code: u32) {
    let accessed_address: usize;
    unsafe { asm!("mov {}, cr2", out(reg) accessed_address) };

    vga::force_unlock();
    serial::force_unlock();

    TUI.lock().vga_text.change_fg_color(Red);

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:#x}", accessed_address);
    println!("Error Code: {:#b}", error_code);
    panic!("Page error occurred, check logs for more infos");
}
