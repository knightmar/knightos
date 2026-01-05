#[macro_export]
macro_rules! log {
    // --------------------------------------------------------
    // Case 1: Explicit Level + String Literal + Arbitrary Args
    // This matches what println! sends: ("{}\n", format_args!(...))
    // --------------------------------------------------------
    ($log_level:expr, $fmt:literal, $($args:tt)*) => {
        $crate::backend::serial::_log(format_args!(
            "[{}] {}\n",
            match $log_level {
                $crate::backend::serial::LogLevel::Info => "INFO",
                $crate::backend::serial::LogLevel::Warn => "WARN",
                $crate::backend::serial::LogLevel::Error => "ERROR",
            },
            format_args!($fmt, $($args)*)
        ));
    };

    // --------------------------------------------------------
    // Case 2: Explicit Level + String Literal (No args)
    // --------------------------------------------------------
    ($log_level:expr, $fmt:literal) => {
        $crate::backend::serial::_log(format_args!(
            "[{}] {}\n",
            match $log_level {
                $crate::backend::serial::LogLevel::Info => "INFO",
                $crate::backend::serial::LogLevel::Warn => "WARN",
                $crate::backend::serial::LogLevel::Error => "ERROR",
            },
            $fmt
        ));
    };

    // --------------------------------------------------------
    // Case 3: Explicit Level + Expression (e.g. manual format_args!)
    // --------------------------------------------------------
    ($log_level:expr, $e:expr) => {
        $crate::backend::serial::_log(format_args!(
            "[{}] {}\n",
            match $log_level {
                $crate::backend::serial::LogLevel::Info => "INFO",
                $crate::backend::serial::LogLevel::Warn => "WARN",
                $crate::backend::serial::LogLevel::Error => "ERROR",
            },
            $e
        ));
    };

    // --------------------------------------------------------
    // Defaults (Info level)
    // --------------------------------------------------------
    ($fmt:literal, $($args:tt)*) => {
        $crate::log!($crate::backend::serial::LogLevel::Info, $fmt, $($args)*);
    };
    ($fmt:literal) => {
        $crate::log!($crate::backend::serial::LogLevel::Info, $fmt);
    };
    ($e:expr) => {
        $crate::log!($crate::backend::serial::LogLevel::Info, $e);
    };
}
