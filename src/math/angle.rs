#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Angle(pub f32);

impl Default for Angle {
    fn default() -> Self {
        Angle(0.0)
    }
}