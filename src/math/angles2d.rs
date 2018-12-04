use crate::math::Angle;

#[repr(C, packed)]
pub struct Angles2d {
    pub yaw: Angle,
    pub pitch: Angle
}

impl Angles2d {
    pub fn new(yaw: Angle, pitch: Angle) -> Angles2d {
        Angles2d { yaw, pitch }
    }
}