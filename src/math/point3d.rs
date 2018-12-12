#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Point3d<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Point3d<T> {
    pub fn new(x: T, y: T, z: T) -> Point3d<T> {
        Point3d { x, y, z }
    }
}

impl<T: Default> Default for Point3d<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), Default::default())
    }
}