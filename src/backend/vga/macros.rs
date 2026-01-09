#[macro_export]
macro_rules! get_colors {
    ($foreground:expr, $background:expr) => {
        ($background as u8) << 4 | (($foreground as u8) & 0b01111111)
    };
}

#[macro_export]
macro_rules! print {
    (@global $fmt:expr, $($arg:tt)*) => {
        $crate::backend::vga::_print(format_args!($fmt, $($arg)*));
        $crate::log!($crate::backend::serial::LogLevel::Info, format_args!($fmt, $($arg)*));
    };
    (@vga $vga:expr, $fmt:expr, $($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($vga, $fmt, $($arg)*);
    }};

    ($fmt:literal, $($arg:tt)+) => {
        $crate::print!(@global $fmt, $($arg)+)
    };
    ($fmt:literal) => {
        $crate::print!(@global $fmt,)
    };
    ($vga:expr, $fmt:literal, $($arg:tt)*) => {
        $crate::print!(@vga $vga, $fmt, $($arg)*)
    };
    ($fmt:expr) => {
        $crate::print!(@global "{}", $fmt)
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!(@global "\n",)
    };
    ($fmt:literal, $($arg:tt)+) => {
        $crate::print!(@global concat!($fmt, "\n"), $($arg)+)
    };
    ($fmt:literal) => {
        $crate::print!(@global concat!($fmt, "\n"),)
    };
    ($vga:expr, $fmt:literal, $($arg:tt)+) => {
        $crate::print!(@vga $vga, concat!($fmt, "\n"), $($arg)+)
    };
    ($vga:expr, $fmt:literal) => {
        $crate::print!(@vga $vga, concat!($fmt, "\n"),)
    };
    ($vga:expr) => {
        $crate::print!(@vga $vga, "\n",)
    };
    ($fmt:expr) => {
        $crate::print!(@global "{}\n", $fmt)
    };
}
