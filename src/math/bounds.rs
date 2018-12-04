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