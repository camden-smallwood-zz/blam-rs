use crate::tags::TagData;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagFunction {
    pub data: TagData
}

impl Default for TagFunction {
    fn default() -> Self {
        Self {
            data: Default::default()
        }
    }
}