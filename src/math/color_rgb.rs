#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct ColorRgb<T> {
    pub red: T,
    pub green: T,
    pub blue: T
}

impl<T> ColorRgb<T> {
    pub fn new(red: T, green: T, blue: T) -> ColorRgb<T> {
        ColorRgb { red, green, blue }
    }
}

impl<T: Default> Default for ColorRgb<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), Default::default())
    }
}