#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct StringId(pub u32);

impl Default for StringId {
    fn default() -> Self {
        StringId(0)
    }
}