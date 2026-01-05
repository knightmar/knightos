#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(dead_code)]

use crate::backend::descriptors::gdt::GdtDescriptor;
use crate::backend::serial::LogLevel::Error;
use crate::backend::vga;
use crate::backend::vga::colors::VGAColors::Red;
use crate::testing::Testable;
use core::arch::asm;
use core::panic::PanicInfo;

mod backend;
mod kernel;
mod testing;
mod user_interface;

#[allow(clippy::empty_loop)]
#[cfg_attr(test, allow(dead_code))]
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    log!("Main");

    GdtDescriptor::load_gdt();
    loop {}
}

pub fn run_test() {
    #[cfg(test)]
    test_main();

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga::force_unlock();

    vga::WRITER.lock().change_fg_color(Red);
    log!(Error, "Erreur critique : {}", info);
    println!("\n{}", info);
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga::WRITER.lock().change_fg_color(Red);
    println!("\n[failed]\n{}", info);
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    println!("Done all tests, all succeded !");
}
