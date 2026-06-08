#[cfg(test)]
use core::arch::asm;
use crate::backend::serial::LogLevel::Test;
#[cfg(test)]
use crate::backend::wait;
use crate::log;

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        let name = core::any::type_name::<T>();

        log!(Test, "Testing {} ... ", name);

        self();

        log!(Test, "[ok]\n------------------------------");
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

#[test_case]
fn wait_test() {
    wait(50);
}