use core::arch::asm;
use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;

pub enum LogLevel {
    Info,
    Warn,
    Error
}


pub struct Serial {
    port: u16,
}

lazy_static! {
    pub static ref LOGGER: Mutex<Serial> = Mutex::new(Serial::new(0x3f8));
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::serial::_log(format_args!("[INFO] {}", $($arg)*));
    };
    ($($arg:tt)*, $($log_level:tt)*) => {
        let str = match $log_level {
            LogLevel::Info => "[INFO]",
            LogLevel::Warn => "[WARN]",
            LogLevel::Error => "[ERROR]",
        };

        $crate::serial::_log(format_args!("{} {}", str, $($arg)*));
    };
}

pub fn _log(args: fmt::Arguments) {
    use core::fmt::Write;
    let _ = LOGGER.lock().write_fmt(args);
}

impl Serial {
    pub fn new(port: u16) -> Self {
        Serial { port }
    }

    pub fn outb(port: u16, val: u8) {
        unsafe {
            asm!(
            "out dx, al",
            in("dx") port,
            in("al") val,
            options(nostack, nomem, preserves_flags)
            );
        }
    }

    pub fn inb(port: u16) -> u8 {
        let mut value: u8;
        unsafe {
            asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nostack, nomem, preserves_flags)
            );
        }

        value
    }
    pub fn init(&mut self) -> Result<(), &'static str> {
        Self::outb(self.port + 1, 0x00);
        Self::outb(self.port + 3, 0x80);
        Self::outb(self.port + 0, 0x03);
        Self::outb(self.port + 1, 0x00);
        Self::outb(self.port + 3, 0x03);
        Self::outb(self.port + 2, 0xC7);
        Self::outb(self.port + 4, 0x0B);
        Self::outb(self.port + 4, 0x1E);
        Self::outb(self.port + 0, 0xAE);

        if Self::inb(self.port + 0) != 0xAE {
            return Err("test");
        }

        Self::outb(self.port + 4, 0x0F);
        Ok(())
    }

    pub fn write_serial(&self, chr: u8) {
        Self::outb(self.port, chr);
    }
}

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for chr in s.as_bytes() {
            self.write_serial(*chr);
        }
        Ok(())
    }
}
