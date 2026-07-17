use crate::backend::memory::vmm::MemMapper;
use crate::backend::serial::LogLevel::Info;
use crate::user_interface::graphics::text::TextManager;
use crate::utils::NotInitError;
use crate::{BOOT_CONFIG, BootConfig, log};
use alloc::vec;
use alloc::vec::Vec;
use core::ops::{Add, AddAssign};
use lazy_static::lazy_static;
use spin::mutex::Mutex;

lazy_static! {
    pub static ref GRAPHICS_HELPER: Mutex<GraphicsHelper> =
        Mutex::new(GraphicsHelper::new().unwrap());
}
pub struct GraphicsHelper {
    pub(crate) boot_config: BootConfig,
    front_buffer_ptr: *mut u8,
    back_buffer: Vec<u8>,
}

//region Structs
#[derive(Clone, Copy)]
pub struct Point {
    pub(crate) x: u32,
    pub(crate) y: u32,
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

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}

#[derive(Copy, Clone)]
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

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Point> for &mut Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
//endregion

unsafe impl Send for GraphicsHelper {}
unsafe impl Sync for GraphicsHelper {}

impl GraphicsHelper {
    pub fn new() -> Result<Self, NotInitError> {
        if let Some(guard) = *BOOT_CONFIG.lock() {
            let fb_size = (guard.fb_pitch * guard.fb_height) as usize;

            let helper = GraphicsHelper {
                boot_config: guard,
                front_buffer_ptr: 0x40000000 as *mut u8,
                back_buffer: vec![0u8; fb_size],
            };

            let fb_phys = helper.boot_config.fb_addr as u32;
            let fb_size = helper.boot_config.fb_pitch * helper.boot_config.fb_height;

            log!(
                Info,
                "{}x{}",
                helper.boot_config.fb_width,
                helper.boot_config.fb_height
            );

            MemMapper::map_range(helper.front_buffer_ptr as u32, fb_phys, fb_size, 3);

            return Ok(helper);
        }

        Err(NotInitError)
    }

    fn get_pixel_offset(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.boot_config.fb_width || y >= self.boot_config.fb_height {
            return None;
        }

        let pitch = self.boot_config.fb_pitch as usize;
        let bpp = (self.boot_config.fb_bpp as usize) / 8;

        let y_offset = (y as usize).checked_mul(pitch)?;
        let x_offset = (x as usize).checked_mul(bpp)?;

        y_offset.checked_add(x_offset)
    }

    pub fn draw_pixel(&mut self, point: Point, color: Color) {
        if let Some(offset) = self.get_pixel_offset(point.x, point.y)
            && offset + 2 < self.back_buffer.len()
        {
            self.back_buffer[offset] = color.b;
            self.back_buffer[offset + 1] = color.g;
            self.back_buffer[offset + 2] = color.r;
        }
    }

    pub fn print_char(&mut self, c: char, point: &Point) {
        let glyph = TextManager::lookup_char(c);
        for y in 0..15 {
            for x in 0..7 {
                let bit = (glyph[y] << x) & 0b10000000;
                if bit != 0 {
                    self.draw_pixel(
                        point
                            + Point {
                                x: x as u32,
                                y: y as u32,
                            },
                        Color {
                            r: 255,
                            g: 255,
                            b: 255,
                        },
                    )
                }
            }
        }
    }

    pub fn draw_line(&mut self, a: Point, b: Point, color: Color) {
        let (mut x1, mut y1) = (a.x as i32, a.y as i32);
        let (x2, y2) = (b.x as i32, b.y as i32);

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();

        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx - dy;

        loop {
            self.draw_pixel((x1 as u32, y1 as u32).into(), color);

            if x1 == x2 && y1 == y2 {
                break;
            }

            let e2 = 2 * err;

            if e2 > -dy {
                err -= dy;
                x1 += sx;
            }

            if e2 < dx {
                err += dx;
                y1 += sy;
            }
        }
    }

    pub fn clear_screen(&mut self) {
        self.back_buffer.fill(0);
    }

    pub fn flush(&self) {
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.back_buffer.as_ptr(),
                self.front_buffer_ptr,
                self.back_buffer.len(),
            );
            core::arch::asm!("mfence");
        }
    }
}
