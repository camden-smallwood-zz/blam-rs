use crate::math::{Angle, Real};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Angles3d<T: Real> {
    pub yaw: Angle<T>,
    pub pitch: Angle<T>,
    pub roll: Angle<T>
}

impl<T: Real> Angles3d<T> {
    pub fn new(yaw: Angle<T>, pitch: Angle<T>, roll: Angle<T>) -> Angles3d<T> {
        Angles3d { yaw, pitch, roll }
    }
}

impl<T: Default + Real> Default for Angles3d<T> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), Default::default())
    }
}