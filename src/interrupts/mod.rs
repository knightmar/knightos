mod utils;

use crate::serial::Serial;
use crate::vga::colors::VGAColors::Red;
use crate::{log, print, println, serial, vga};
use core::arch::asm;

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
    } else if scancode == 0x4b {
        vga::WRITER.lock().pointer.move_prev_pos();
    } else {
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

    vga::WRITER.lock().change_fg_color(Red);

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:#x}", accessed_address);
    println!("Error Code: {:#b}", error_code);
    panic!("Page error occurred, check logs for more infos");
}
