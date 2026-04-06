#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![feature(allocator_api)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(dead_code)]
extern crate alloc;

use crate::backend::descriptors::gdt::GdtDescriptor;
use crate::backend::memory::pmm::{MultibootInfo, BITMAP_PAGE};
use crate::backend::serial::LogLevel::Error;
use crate::backend::{qemu_shutdown, vga, wait};
use crate::testing::Testable;
use core::arch::asm;
use core::panic::PanicInfo;
use spin::mutex::Mutex;

mod backend;
mod kernel;
mod testing;
mod user_interface;
mod utils;

#[derive(Copy, Clone)]
pub struct BootConfig {
    pub fb_addr: u64,
    pub fb_width: u32,
    pub fb_height: u32,
    pub fb_pitch: u32,
    pub fb_bpp: u8,
    pub mem_upper: u32,
}

// This is perfectly safe and "Send" because it contains no pointers
pub static BOOT_CONFIG: Mutex<Option<BootConfig>> = Mutex::new(None);

#[allow(clippy::empty_loop)]
#[cfg_attr(test, allow(dead_code))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_main(magic: u32, mb_info_ptr: *const MultibootInfo) -> ! {
    log!("Main");

    unsafe {
        // Get a raw pointer to the static bitmap
        let mut bitmap_ptr = core::ptr::addr_of!(BITMAP_PAGE).as_ref().unwrap().lock();

        // Call init through the raw pointer
        (*bitmap_ptr).init(mb_info_ptr);

        let mbi = &*mb_info_ptr;
        let config = BootConfig {
            fb_addr: mbi.framebuffer_addr,
            fb_width: mbi.framebuffer_width,
            fb_height: mbi.framebuffer_height,
            fb_pitch: mbi.framebuffer_pitch,
            fb_bpp: mbi.framebuffer_bpp,
            mem_upper: mbi.mem_upper,
        };
        let mut guard = BOOT_CONFIG.lock();
        *guard = Some(config);
        drop(guard);
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

    // TUI.lock().vga_text.change_fg_color(Red);
    log!(Error, "Erreur critique : {}", info);
    println!("\n[ERROR] Shutting down\n{}", info);

    unsafe { asm!("sti") };

    wait(1000);

    qemu_shutdown();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TUI.lock().vga_text.change_fg_color(Red);
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

#[cfg(not(test))]
#[warn(unused)]
pub fn test_main() {}
