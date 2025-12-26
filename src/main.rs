#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]



use crate::descriptors::gdt::{GdtDescriptor, post_gdt};
use crate::descriptors::idt::load_idt;
use crate::descriptors::pic::Pic;
use crate::serial::LogLevel::Error;
use crate::serial::{LogLevel, Serial};
use crate::vga::colors::VGAColors::*;
use core::panic::PanicInfo;

mod descriptors;
mod interrupts;
mod kernel;
mod serial;
mod vga;
mod testing;

#[allow(clippy::empty_loop)]
#[cfg_attr(test, allow(dead_code))]
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    GdtDescriptor::load_gdt();
    #[cfg(test)]
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga::WRITER.lock().change_fg_color(Red);
    log!(Error, "Erreur critique : {}", info);
    println!("\n{}", info);
    loop {}
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}