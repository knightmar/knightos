use crate::backend::descriptors::idt::load_idt;
use crate::backend::descriptors::pic::Pic;
use crate::backend::paging::init_paging;
use crate::backend::serial::Serial;
use crate::{println, run_test};
use crate::backend::{qemu_shutdown, wait};

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

    run_test();
}
