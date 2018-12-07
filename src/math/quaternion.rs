#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Quaternion<T> {
    pub i: T,
    pub j: T,
    pub k: T,
    pub w: T
}

impl<T> Quaternion<T> {
    pub fn new(i: T, j: T, k: T, w: T) -> Quaternion<T> {
        Quaternion { i, j, k, w }
    }
}