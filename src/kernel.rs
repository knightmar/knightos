use crate::backend::descriptors::idt::load_idt;
use crate::backend::descriptors::pic::Pic;
use crate::backend::memory::init_heap;
use crate::backend::memory::pmm::BITMAP_PAGE;
use crate::backend::memory::vmm::MemMapper;
use crate::backend::paging::init_paging;
use crate::backend::serial::LogLevel::Info;
use crate::backend::serial::Serial;
use crate::backend::{qemu_shutdown, wait};
use crate::{log, println, run_test};
use alloc::boxed::Box;
use alloc::vec::Vec;

pub fn protected_main() {
    log!("test");

    init_paging();

    unsafe { Pic::remap() }
    unsafe { load_idt() }
    Serial::outb(0x21, 0xFC); // activate interrupts
    unsafe { init_heap() }

    unsafe {
        core::arch::asm!("sti");
    }
    run_test();
}
