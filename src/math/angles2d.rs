use crate::math::{Angle, Real};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Angles2d<T: Real> {
    pub yaw: Angle<T>,
    pub pitch: Angle<T>
}

impl<T: Real> Angles2d<T> {
    pub fn new(yaw: Angle<T>, pitch: Angle<T>) -> Angles2d<T> {
        Angles2d { yaw, pitch }
    }
}

impl<T: Default + Real> Default for Angles2d<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}