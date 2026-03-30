use crate::backend::allocator::BITMAP_PAGE;
use crate::backend::descriptors::idt::load_idt;
use crate::backend::descriptors::pic::Pic;
use crate::backend::paging::init_paging;
use crate::backend::serial::Serial;
use crate::backend::{qemu_shutdown, wait};
use crate::{println, run_test};

pub fn protected_main() {
    init_paging();

    unsafe { Pic::remap() }
    unsafe { load_idt() }
    Serial::outb(0x21, 0xFC); // activate interrupts
    unsafe {
        core::arch::asm!("sti");
    }

    // unsafe {
    //     let ptr = 0x8000 as *mut u8;
    //     println!("{}", *ptr);
    //     for x in 0..1000 {
    //         *(ptr.wrapping_add(x)) = 255;
    //     }
    //     println!("{}", *ptr);
    // }

    unsafe {
        let p1 = BITMAP_PAGE.lock().alloc_frame().unwrap();
        println!("{}", p1);
        let p2 = BITMAP_PAGE.lock().alloc_frame().unwrap();
        println!("{}", p2);
        BITMAP_PAGE
            .lock()
            .free_frame(p1)
            .expect("TODO: panic message");
        let p3 = BITMAP_PAGE.lock().alloc_frame().unwrap();
        println!("{}", p3);
    }
    run_test();
}
