use crate::backend::serial::LogLevel::Test;
#[cfg(test)]
use crate::backend::wait;
use crate::log;
#[cfg(test)]
use claim::assert_ok;
#[cfg(test)]
use core::arch::asm;

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
#[allow(clippy::eq_op)]
fn trivial_assertion() {
    assert_ok!(Ok::<i32, i32>(0));
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
