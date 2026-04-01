use core::error::Error;
use core::fmt::{Debug, Display, Formatter};

pub struct NotInitError;

impl Debug for NotInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "NotInitError")
    }
}

impl Display for NotInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "NotInitError")
    }
}

impl Error for NotInitError {}
