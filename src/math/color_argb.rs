#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct ColorArgb<T> {
    pub alpha: T,
    pub red: T,
    pub green: T,
    pub blue: T
}

impl<T> ColorArgb<T> {
    pub fn new(alpha: T, red: T, green: T, blue: T) -> ColorArgb<T> {
        ColorArgb { alpha, red, green, blue }
    }
}