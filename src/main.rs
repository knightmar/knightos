#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![feature(allocator_api)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(dead_code)]

use crate::backend::allocator::{BITMAP_PAGE, BitMapPages};
use crate::backend::descriptors::gdt::GdtDescriptor;
use crate::backend::serial::LogLevel::Error;
use crate::backend::vga;
use crate::backend::vga::colors::VGAColors::Red;
use crate::testing::Testable;
use crate::user_interface::text_user_interface::TUI;
use core::arch::asm;
use core::panic::PanicInfo;

mod backend;
mod kernel;
mod testing;
mod user_interface;

#[allow(clippy::empty_loop)]
#[cfg_attr(test, allow(dead_code))]
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main(magic: u32, mb_info_ptr: usize) -> ! {
    log!("Main");
    log!(Error, "{:x}", magic);
    unsafe {
        // Get a raw pointer to the static bitmap
        let bitmap_ptr = core::ptr::addr_of_mut!(BITMAP_PAGE);

        // Call init through the raw pointer
        (*bitmap_ptr).init(mb_info_ptr);
    }

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

    TUI.lock().vga_text.change_fg_color(Red);
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
    TUI.lock().vga_text.change_fg_color(Red);
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
