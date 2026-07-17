pub mod utils;

use crate::backend::multitasking::SCHEDULER;
use crate::backend::serial;
use crate::backend::serial::LogLevel::{Error, Info};
use crate::backend::serial::Serial;
use crate::log;
use crate::user_interface::input::INPUT_SYSTEM;
use core::arch::{asm, global_asm};
use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering::Relaxed;

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

// black magic to avoid rust messing up the interrupt stack
global_asm!(
    ".global timer_interrupt_entry",
    "timer_interrupt_entry:",
    "    pusha",
    "    mov eax, esp",
    "    push eax",
    "    call timer_handler_inner",
    "    add esp, 4",
    "    mov esp, eax",
    "    popa",
    "    iretd",
);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn timer_handler_inner(current_esp: u32) -> u32 {
    TICK_COUNT.fetch_add(1, Relaxed);
    Serial::outb(0x20, 0x20);

    let mut scheduler = SCHEDULER.lock();
    scheduler.switch_next(current_esp)
}

#[unsafe(no_mangle)]
pub extern "C" fn double_fault_handler_inner(_esp: u32) {
    log!(Error, "Double fault happened T-T");

    unsafe {
        asm!("int3");
    }

    loop {
        unsafe { asm!("hlt") };
    }
}

pub unsafe extern "x86-interrupt" fn gpf_handler(frame: InterruptStackFrame, error_code: u32) {
    serial::force_unlock();

    // Read segment registers / esp / cr3
    let ds: u32;
    let es: u32;
    let fs: u32;
    let gs: u32;
    let ss: u32;
    let esp_reg: u32;
    let cr3: u32;
    unsafe {
        asm!("mov {}, ds", out(reg) ds);
        asm!("mov {}, es", out(reg) es);
        asm!("mov {}, fs", out(reg) fs);
        asm!("mov {}, gs", out(reg) gs);
        asm!("mov {}, ss", out(reg) ss);
        asm!("mov {}, esp", out(reg) esp_reg);
        asm!("mov {}, cr3", out(reg) cr3);
    }

    log!(Error, "EXCEPTION: GENERAL PROTECTION FAULT");
    log!(
        Error,
        "Error Code: {:#x} (GDT Index: {})",
        error_code,
        error_code >> 3
    );
    log!(Error, "Faulting EIP: {:#x}", frame.instruction_pointer);
    log!(Error, "Faulting CS: {:#x}", frame.code_segment);

    log!(
        Error,
        "SEGMENTS: DS={:#x} ES={:#x} FS={:#x} GS={:#x} SS={:#x}",
        ds,
        es,
        fs,
        gs,
        ss
    );
    log!(Error, "REGS: ESP={:#x} CR3={:#x}", esp_reg, cr3);

    unsafe {
        asm!("cli");
        asm!("int3");
        loop {
            asm!("hlt");
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
    let scancode: u8 = Serial::inb(0x60);

    if let Some(mut ip) = INPUT_SYSTEM.try_lock() {
        ip.on_keyboard_event(scancode);
    }

    Serial::outb(0x20, 0x20);
}

pub extern "x86-interrupt" fn page_fault_handler(_frame: InterruptStackFrame, error_code: u32) {
    let accessed_address: usize;
    unsafe { asm!("mov {}, cr2", out(reg) accessed_address) };

    serial::force_unlock();

    // INPUT_SYSTEM.lock().vga_text.change_fg_color(Red);
    //
    log!(Error, "EXCEPTION: PAGE FAULT");
    log!(Error, "Accessed Address: {:#x}", accessed_address);
    log!(Error, "Error Code: {:#b}", error_code);
    unsafe {
        asm!("int3");
    }

    panic!("Page error occurred, check logs for more infos");
}

// fill out handler

pub extern "x86-interrupt" fn generic_handler(_frame: InterruptStackFrame) {
    log!(Info, "Unhandled interrupt : {:#?}", _frame);
    Serial::outb(0x20, 0x20);
    Serial::outb(0xA0, 0x20);
}
