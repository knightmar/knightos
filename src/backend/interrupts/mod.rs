mod utils;

use crate::backend::serial::LogLevel::{Error, Info};
use crate::backend::serial::Serial;
use crate::backend::vga::colors::VGAColors::Red;
use crate::backend::{serial, vga};
use crate::user_interface::INPUT_SYSTEM;
use crate::{log, println};
use core::arch::asm;
use core::sync::atomic::Ordering::Relaxed;
use core::sync::atomic::{AtomicU32, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;

pub static TICK_COUNT: AtomicU32 = AtomicU32::new(0);

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
    TICK_COUNT.fetch_add(1, Relaxed);

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

    let mut input = INPUT_SYSTEM.lock();

    input.on_keyboard_event(scancode);
    // end of interrupt
    Serial::outb(0x20, 0x20);
}

pub extern "x86-interrupt" fn page_fault_handler(_frame: InterruptStackFrame, error_code: u32) {
    let accessed_address: usize;
    unsafe { asm!("mov {}, cr2", out(reg) accessed_address) };

    vga::force_unlock();
    serial::force_unlock();

    // INPUT_SYSTEM.lock().vga_text.change_fg_color(Red);
    //
    log!(Error, "EXCEPTION: PAGE FAULT");
    log!(Error, "Accessed Address: {:#x}", accessed_address);
    log!(Error, "Error Code: {:#b}", error_code);
    panic!("Page error occurred, check logs for more infos");
}
