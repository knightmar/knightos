use crate::backend::descriptors::idt::load_idt;
use crate::backend::descriptors::pic::Pic;
use crate::backend::memory::init_heap;
use crate::backend::multitasking::{SCHEDULER, create_task, start_scheduler};
use crate::backend::paging::init_paging;
use crate::backend::serial::LogLevel::Info;
use crate::backend::serial::Serial;
use crate::log;
use crate::user_interface::graphics::render_task;

fn graphics_input_task() {
    log!(Info, "INPUT TASK STARTING");
    loop {
        for _ in 0..1000000 {
            unsafe {
                core::arch::asm!("hlt");
            }
        }
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
    Serial::outb(0x21, 0xFC);

    unsafe { init_heap() }

    {
        let mut scheduler = SCHEDULER.lock();

        let t1 = create_task(graphics_input_task, 0);
        scheduler.add_task(t1).unwrap();

        let t2 = create_task(render_task, 1);
        scheduler.add_task(t2).unwrap();
    }

    let mask: u8;
    unsafe {
        core::arch::asm!("in al, dx", in("dx") 0x21u16, out("al") mask);
    }
    log!(Info, "PIC MASTER MASK = {:#x}", mask);

    log!("Starting scheduler...");
    start_scheduler();
}
