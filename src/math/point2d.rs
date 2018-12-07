#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Point2d<T> {
    pub x: T,
    pub y: T
}

impl<T> Point2d<T> {
    pub fn new(x: T, y: T) -> Point2d<T> {
        Point2d { x, y }
    }
}