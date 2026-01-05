use core::arch::asm;
use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::vga::WRITER;

pub enum LogLevel {
    Info,
    Warn,
    Error,
}

pub struct Serial {
    port: u16,
}

lazy_static! {
    pub static ref LOGGER: Mutex<Serial> = Mutex::new(Serial::new(0x3f8));
}

#[macro_export]
macro_rules! log {
    // --------------------------------------------------------
    // Case 1: Explicit Level + String Literal + Arbitrary Args
    // This matches what println! sends: ("{}\n", format_args!(...))
    // --------------------------------------------------------
    ($log_level:expr, $fmt:literal, $($args:tt)*) => {
        $crate::serial::_log(format_args!(
            "[{}] {}\n",
            match $log_level {
                $crate::serial::LogLevel::Info => "INFO",
                $crate::serial::LogLevel::Warn => "WARN",
                $crate::serial::LogLevel::Error => "ERROR",
            },
            format_args!($fmt, $($args)*)
        ));
    };

    // --------------------------------------------------------
    // Case 2: Explicit Level + String Literal (No args)
    // --------------------------------------------------------
    ($log_level:expr, $fmt:literal) => {
        $crate::serial::_log(format_args!(
            "[{}] {}\n",
            match $log_level {
                $crate::serial::LogLevel::Info => "INFO",
                $crate::serial::LogLevel::Warn => "WARN",
                $crate::serial::LogLevel::Error => "ERROR",
            },
            $fmt
        ));
    };

    // --------------------------------------------------------
    // Case 3: Explicit Level + Expression (e.g. manual format_args!)
    // --------------------------------------------------------
    ($log_level:expr, $e:expr) => {
        $crate::serial::_log(format_args!(
            "[{}] {}\n",
            match $log_level {
                $crate::serial::LogLevel::Info => "INFO",
                $crate::serial::LogLevel::Warn => "WARN",
                $crate::serial::LogLevel::Error => "ERROR",
            },
            $e
        ));
    };

    // --------------------------------------------------------
    // Defaults (Info level)
    // --------------------------------------------------------
    ($fmt:literal, $($args:tt)*) => {
        $crate::log!($crate::serial::LogLevel::Info, $fmt, $($args)*);
    };
    ($fmt:literal) => {
        $crate::log!($crate::serial::LogLevel::Info, $fmt);
    };
    ($e:expr) => {
        $crate::log!($crate::serial::LogLevel::Info, $e);
    };
}
pub fn _log(args: fmt::Arguments) {
    use core::fmt::Write;
    let _ = LOGGER.lock().write_fmt(args);
}

pub fn force_unlock() {
    unsafe {
        LOGGER.force_unlock();
    }
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
