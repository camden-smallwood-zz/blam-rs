use crate::math::Vector3d;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Plane3d<T> {
    pub normal: Vector3d<T>,
    pub distance: T
}

impl<T> Plane3d<T> {
    pub fn new(normal: Vector3d<T>, distance: T) -> Plane3d<T> {
        Plane3d { normal, distance }
    }
}