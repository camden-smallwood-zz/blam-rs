#[repr(C, packed)]
pub struct Vector3d<T> {
    pub i: T,
    pub j: T,
    pub k: T
}

impl<T> Vector3d<T> {
    pub fn new(i: T, j: T, k: T) -> Vector3d<T> {
        Vector3d { i, j, k }
    }
}