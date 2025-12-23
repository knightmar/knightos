#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

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

#[allow(clippy::empty_loop)]
#[cfg_attr(test, allow(dead_code))]
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    log!("Testing gdt");

    GdtDescriptor::load_gdt();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga::WRITER.lock().change_fg_color(Red);
    log!(Error, "Erreur critique : {}", info);
    println!("\n{}", info);
    loop {}
}
