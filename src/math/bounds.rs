#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Bounds<T> {
    pub lower: T,
    pub upper: T
}

impl<T> Bounds<T> {
    pub fn new(lower: T, upper: T) -> Bounds<T> {
        Bounds { lower, upper }
    }
}

impl<T: Default> Default for Bounds<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}