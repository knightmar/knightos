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

fn graphics() {
    let mut graph = GraphicsHelper::new().unwrap();

    loop {
        graph.clear_screen();

        for y in 0..200 {
            for x in 0..200 {
                graph.draw_pixel((x, y).into(), (255, 0, 0).into());
            }
            graph.flush();
        }
    }
}

fn task_b() {
    loop {
        log!("Task B");
    }
}

pub fn protected_main() {
    log!("Initialisation Système...");
    unsafe { core::arch::asm!("cli") };

    init_paging();

    unsafe {
        Pic::remap();
        Pic::init_timer();
    }

    unsafe { load_idt() }
    Serial::outb(0x21, 0xFE);

    unsafe { init_heap() }

    {
        let mut scheduler = SCHEDULER.lock();

        let t1 = create_task(graphics, 0);
        scheduler.add_task(t1).unwrap();

        let t2 = create_task(task_b, 1);
        scheduler.add_task(t2).unwrap();
    }

    log!("Starting scheduler...");
    start_scheduler();
}
