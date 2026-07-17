#[macro_export]
macro_rules! log {
    (@expand $log_level:expr, $args:expr) => {
        $crate::backend::serial::_log(format_args!(
            "[{}] {}\n",
            $log_level,
            $args
        ));
    };

    ($log_level:expr, $fmt:literal, $($args:tt)*) => {
        $crate::log!(@expand $log_level, format_args!($fmt, $($args)*));
    };

    ($log_level:expr, $e:expr) => {
        $crate::log!(@expand $log_level, $e);
    };

    ($fmt:literal, $($args:tt)*) => {
        $crate::log!($crate::backend::serial::LogLevel::Info, $fmt, $($args)*);
    };
    ($e:expr) => {
        $crate::log!($crate::backend::serial::LogLevel::Info, $e);
    };
}