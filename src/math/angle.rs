use crate::math::Real;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Angle<T: Real>(pub T);

impl<T: Default + Real> Default for Angle<T> {
    fn default() -> Self {
        Angle(Default::default())
    }
}