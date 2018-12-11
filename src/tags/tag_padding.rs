#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagPadding<T>(pub T);

impl<T: Default> Default for TagPadding<T> {
    fn default() -> Self {
        TagPadding(Default::default())
    }
}