use crate::backend::memory::vmm::MemMapper;
use crate::utils::NotInitError;
use crate::{BOOT_CONFIG, BootConfig};
use alloc::ffi::{CString, NulError};
use core::alloc::AllocError;
use core::error::Error;
use core::ops::Add;

pub struct GraphicsHelper {
    boot_config: BootConfig,
    fb_ptr: *mut u8,
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
            let helper = GraphicsHelper {
                boot_config: guard,
                fb_ptr: 0x40000000 as *mut u8,
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

    pub fn draw_pixel(&self, x: u32, y: u32, color: Color) {
        let pixel_offset = self.get_pixel_offset(x, y);
        unsafe {
            *self.fb_ptr.add(pixel_offset as usize) = color.b; // Blue
            *self.fb_ptr.add(pixel_offset as usize + 1) = color.g; // Green
            *self.fb_ptr.add(pixel_offset as usize + 2) = color.r; // Red
        }
    }
}
