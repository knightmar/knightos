use crate::backend::memory::memset_u32;
use crate::backend::memory::vmm::MemMapper;
use crate::utils::NotInitError;
use crate::{BOOT_CONFIG, BootConfig};
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Add;

pub struct GraphicsHelper {
    boot_config: BootConfig,
    fb_ptr: *mut u8,
    back_buffer: Vec<u8>,
}
pub struct Point {
    x: u32,
    y: u32,
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl GraphicsHelper {
    pub fn new() -> Result<Self, NotInitError> {
        if let Some(guard) = *BOOT_CONFIG.lock() {
            let fb_size = (guard.fb_pitch * guard.fb_height) as usize;

            let helper = GraphicsHelper {
                boot_config: guard,
                fb_ptr: 0x40000000 as *mut u8,
                back_buffer: vec![0u8; fb_size],
            };

            let fb_phys = helper.boot_config.fb_addr as u32;
            let fb_size = helper.boot_config.fb_pitch * helper.boot_config.fb_height;

            unsafe {
                MemMapper::map_range(helper.fb_ptr as u32, fb_phys, fb_size, 3);
            }
            return Ok(helper);
        }

        Err(NotInitError)
    }

    fn get_pixel_offset(&self, x: u32, y: u32) -> u32 {
        (y * self.boot_config.fb_pitch) + (x * (self.boot_config.fb_bpp as u32 / 8))
    }

    pub fn draw_pixel(&mut self, point: Point, color: Color) {
        let offset = self.get_pixel_offset(point.x, point.y) as usize;
        if offset + 2 < self.back_buffer.len() {
            self.back_buffer[offset] = color.b;
            self.back_buffer[offset + 1] = color.g;
            self.back_buffer[offset + 2] = color.r;
        }
    }

    pub fn draw_line(&self, a: Point, b: Point) {}

    pub fn clear_screen(&mut self) {
        self.back_buffer.fill(0);
    }

    pub fn flush(&self) {
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.back_buffer.as_ptr(),
                self.fb_ptr,
                self.back_buffer.len(),
            );
            core::arch::asm!("mfence");
        }
    }
}
