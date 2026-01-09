mod utils;

use crate::backend::serial::Serial;
use crate::backend::vga::colors::VGAColors::Red;
use crate::backend::{serial, vga};
use crate::user_interface::text_user_interface::TUI;
use crate::{log, println};
use core::arch::asm;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref TICK_COUNT: Mutex<u64> = Mutex::new(0);
}

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
    if let Some(mut count) = TICK_COUNT.try_lock() {
        *count += 1;
    }

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

    let mut tui = crate::user_interface::text_user_interface::TUI.lock();

    tui.on_keyboard_event(scancode);
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
