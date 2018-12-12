use crate::math::Angle;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Angles3d {
    pub yaw: Angle,
    pub pitch: Angle,
    pub roll: Angle
}

impl Angles3d {
    pub fn new(yaw: Angle, pitch: Angle, roll: Angle) -> Angles3d {
        Angles3d { yaw, pitch, roll }
    }
}

impl Default for Angles3d {
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), Default::default())
    }
}