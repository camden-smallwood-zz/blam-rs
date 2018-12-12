use crate::math::Angle;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Angles2d {
    pub yaw: Angle,
    pub pitch: Angle
}

impl Angles2d {
    pub fn new(yaw: Angle, pitch: Angle) -> Angles2d {
        Angles2d { yaw, pitch }
    }
}

impl Default for Angles2d {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}