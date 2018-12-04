use crate::tags::TagData;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagFunction {
    pub data: TagData
}