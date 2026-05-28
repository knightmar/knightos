use crate::backend::descriptors::idt::load_idt;
use crate::backend::descriptors::pic::Pic;
use crate::backend::memory::init_heap;
use crate::backend::multitasking::{SCHEDULER, Scheduler, Task, TaskState, create_task};
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
        log!("Bonjour de la tâche A !");
        Scheduler::yield_now();
    }
}

fn task_b() {
    loop {
        log!("... Et coucou de la tâche B !");
        Scheduler::yield_now();
    }
}

pub fn protected_main() {
    log!("test");

    init_paging();

    unsafe {
        Pic::remap();
        Pic::init_timer();
    }
    unsafe { load_idt() }
    Serial::outb(0x21, 0xFC); // activate interrupts

    unsafe { init_heap() }

    unsafe {
        core::arch::asm!("sti");
    }

    let mut result = GraphicsHelper::new().unwrap();
    result.clear_screen();

    {
        let mut scheduler = SCHEDULER.lock();

        let main_task = Task {
            id: 0,
            esp: 0,
            cr3: 0,
            state: TaskState::UNINITIALIZED,
            stack: vec![],
        };
        scheduler.add_task(main_task).unwrap();

        let mut t1 = create_task(task_a, 0);
        t1.id = 1;
        scheduler.add_task(t1).unwrap();

        let mut t2 = create_task(task_b, 0);
        t2.id = 2;
        scheduler.add_task(t2).unwrap();
    }

    log!("Lancement du multitâche...");

    Scheduler::yield_now();

    loop {
        Scheduler::yield_now();
    }

    run_test();
}
