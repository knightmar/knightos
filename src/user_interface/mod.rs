use crate::backend::serial::LogLevel::Info;
use crate::log;
use crate::user_interface::utils::translate_keys;
use spin::mutex::Mutex;

pub mod graphics;
pub mod input;
pub mod utils;

