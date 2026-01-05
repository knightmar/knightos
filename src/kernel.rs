use crate::backend::descriptors::idt::load_idt;
use crate::backend::descriptors::pic::Pic;
use crate::backend::paging::init_paging;
use crate::backend::serial::Serial;
use crate::{println, run_test};

pub fn protected_main() {
    unsafe { Pic::remap() }
    unsafe { load_idt() }
    Serial::outb(0x21, 0xFC); // activate interrupts
    init_paging();

    println!("test");

    // unsafe {
    //     let ptr = 0xdeadbeef as *mut u8;
    //     println!("{}", *ptr);
    //     *ptr = 42;
    //     println!("{}", *ptr);
    // }

    run_test();
}
