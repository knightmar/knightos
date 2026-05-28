mod utils;

use crate::backend::multitasking::{SCHEDULER, Scheduler, TaskState};
use crate::backend::serial::LogLevel::{Error, Info};
use crate::backend::serial::Serial;
use crate::backend::vga::colors::VGAColors::Red;
use crate::backend::{serial, vga};
use crate::user_interface::INPUT_SYSTEM;
use crate::{log, println};
use core::arch::{asm, global_asm};
use core::sync::atomic::Ordering::Relaxed;
use core::sync::atomic::{AtomicU32, Ordering};
use lazy_static::lazy_static;
use spin::Mutex;

pub static TICK_COUNT: AtomicU32 = AtomicU32::new(0);

#[repr(C)]
#[derive(Debug)]
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

unsafe extern "C" {
    pub fn timer_interrupt_entry();
}

global_asm!(
    ".global timer_interrupt_entry",
    "timer_interrupt_entry:",
    "    pusha",
    "    mov eax, esp", // ← capture esp AFTER pusha (before push arg)
    "    push eax",     // push as arg
    "    call timer_handler_inner",
    "    add esp, 4",   // clean arg — esp is back to post-pusha position
    "    mov esp, eax", // eax = returned esp (new or same task's post-pusha esp)
    "    popa",
    "    iret",
);
#[unsafe(no_mangle)]
pub unsafe extern "C" fn timer_handler_inner(current_esp: u32) -> u32 {
    TICK_COUNT.fetch_add(1, Relaxed);

    Serial::outb(0x20, 0x20);

    let mut scheduler = SCHEDULER.lock();

    scheduler.switch_next(current_esp)
}

#[unsafe(no_mangle)]
pub extern "C" fn double_fault_handler_inner(esp: u32) {
    // Absolutely minimal — just write to serial directly, no allocations
    Serial::outb(0x3F8, b'D');
    Serial::outb(0x3F8, b'F');
    Serial::outb(0x3F8, b'\n');

    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

pub extern "x86-interrupt" fn gpf_handler(frame: InterruptStackFrame, error_code: u32) {
    vga::force_unlock();
    serial::force_unlock();

    log!(Error, "EXCEPTION: GENERAL PROTECTION FAULT");
    log!(
        Error,
        "Error Code: {:#x} (GDT Index: {})",
        error_code,
        error_code >> 3
    );
    log!(Error, "Faulting EIP: {:#x}", frame.instruction_pointer);
    log!(Error, "Faulting CS: {:#x}", frame.code_segment);

    unsafe {
        core::arch::asm!("cli");
        loop {
            core::arch::asm!("hlt");
        }
    }
}

global_asm!(
    ".global double_fault_entry",
    "double_fault_entry:",
    "    cli",
    "    pusha",
    "    push esp",
    "    call double_fault_handler_inner",
    "    hlt",
);

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

// fill out handler

pub extern "x86-interrupt" fn generic_handler(_frame: InterruptStackFrame) {
    log!(Info, "Unhandled interrupt : {:#?}", _frame);
    Serial::outb(0x20, 0x20);
    Serial::outb(0xA0, 0x20);
}
