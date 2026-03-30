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
use alloc::vec::Vec;

pub fn protected_main() {
    init_paging();
    unsafe { init_heap() }


    unsafe { Pic::remap() }
    unsafe { load_idt() }
    Serial::outb(0x21, 0xFC); // activate interrupts

    log!("test");


    log!("test");

    // unsafe {
    //     let ptr = 0x8000 as *mut u8;
    //     println!("{}", *ptr);
    //     for x in 0..1000 {
    //         *(ptr.wrapping_add(x)) = 255;
    //     }
    //     println!("{}", *ptr);
    // }

    unsafe {
        use alloc::boxed::Box;
        let test_value = Box::new(42);
        log!(Info, "Heap value: {}", *test_value);
    }

    log!("test");


    unsafe {
        core::arch::asm!("sti");
    }
    run_test();
}
