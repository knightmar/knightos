use crate::backend::descriptors::idt::load_idt;
use crate::backend::descriptors::pic::Pic;
use crate::backend::memory::init_heap;
use crate::backend::multitasking::{
    SCHEDULER, Scheduler, Task, TaskState, create_task, start_scheduler,
};
use crate::backend::paging::init_paging;
use crate::backend::serial::LogLevel::Info;
use crate::backend::serial::Serial;
use crate::backend::wait;
use crate::user_interface::INPUT_SYSTEM;
use crate::user_interface::graphic_user_interface::{Color, GraphicsHelper};
use crate::{log, run_test};
use alloc::vec;
use alloc::vec::Vec;
// include!("../ressources/image_data.rs");

fn task_a() {
    loop {
        unsafe { core::arch::asm!("nop") } // Aucun log pour le moment
    }
}

fn task_b() {
    loop {
        unsafe { core::arch::asm!("nop") } // Aucun log pour le moment
    }
}

pub fn protected_main() {
    log!("Initialisation Système...");
    unsafe { core::arch::asm!("cli") };

    init_paging();

    unsafe {
        Pic::remap();
        Pic::init_timer(); // Make sure your PIT divider is configured here!
    }

    unsafe { load_idt() }
    Serial::outb(0x21, 0xFE); // Unmask IRQ0 (Timer Only). Disables keyboard to avoid conflicts for now.

    unsafe { init_heap() }

    let mut result = GraphicsHelper::new().unwrap();
    result.clear_screen();

    // Register our real execution units inside the scheduler array
    {
        let mut scheduler = SCHEDULER.lock();

        let mut t1 = create_task(task_a, 0);
        t1.id = 0;
        scheduler.add_task(t1).unwrap();

        let mut t2 = create_task(task_b, 0);
        t2.id = 1;
        scheduler.add_task(t2).unwrap();
    }

    log!("Lancement du préemptif...");
    start_scheduler(); // Jumps right into Task A, turning on interrupts during the iret phase!
}