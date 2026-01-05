use core::arch::asm;
use crate::log;
use crate::serial::LogLevel::Info;

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        let name = core::any::type_name::<T>();

        crate::print!("Testing {} ... ", name);

        self();

        crate::println!("[ok]\n------------------------------");
    }
}

#[test_case]
fn breakpoint_interrupt() {
    unsafe {
        asm!("int3");
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

// #[test_case]
// fn paging_test() {
//     unsafe {
//         let ptr = 0xdeadbeef as *mut u8;
//         log!(Info, "{}", *ptr);
//         *ptr = 42;
//         log!(Info, "{}", *ptr);
//     }
// }
