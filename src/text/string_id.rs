#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd)]
pub struct StringId(pub u32);

impl Default for StringId {
    fn default() -> Self {
        StringId(0)
    }
}