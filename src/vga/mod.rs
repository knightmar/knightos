pub(crate) mod colors;

#[macro_export]
macro_rules! get_colors {
    ($foreground:expr, $background:expr) => {
        ($foreground as u8) << 4 | ($background as u8)
    };
}


struct VGAText {
    
}


