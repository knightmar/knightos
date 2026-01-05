#[macro_export]
macro_rules! get_colors {
    ($foreground:expr, $background:expr) => {
        ($background as u8) << 4 | (($foreground as u8) & 0b01111111)
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::backend::vga::_print(format_args!($($arg)*));
        $crate::log!($crate::backend::serial::LogLevel::Info, $($arg)*);
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*));
    };
}