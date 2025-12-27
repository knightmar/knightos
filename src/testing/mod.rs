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

        crate::print!("Testing {} ... ", name);

        self();

        crate::println!("[ok]");
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
