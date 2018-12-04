use crate::math::Vector2d;

#[repr(C, packed)]
pub struct Plane2d<T> {
    pub normal: Vector2d<T>,
    pub distance: T
}

impl<T> Plane2d<T> {
    pub fn new(normal: Vector2d<T>, distance: T) -> Plane2d<T> {
        Plane2d { normal, distance }
    }
}