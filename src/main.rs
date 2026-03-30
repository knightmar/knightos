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
use crate::backend::serial::LogLevel::{Error, Info};
use crate::backend::{qemu_shutdown, vga, wait};
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
    
    unsafe {
        // Get a raw pointer to the static bitmap
        let bitmap_ptr = core::ptr::addr_of_mut!(BITMAP_PAGE);

        // Call init through the raw pointer
        (*bitmap_ptr).init(mb_info_ptr);

        log!(Info, "Page 0 Used: {}", (*bitmap_ptr).is_used(0));
        log!(
            Info,
            "Kernel Used: {}",
            (*bitmap_ptr).is_used(0x100000 / 4096)
        );
        log!(
            Info,
            "Free RAM: {}",
            (*bitmap_ptr).is_used(0x2000000 / 4096)
        );
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

    qemu_shutdown();
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
    qemu_shutdown();
}
