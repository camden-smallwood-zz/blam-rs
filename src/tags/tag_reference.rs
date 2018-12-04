use crate::tags::Tag;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagReference {
    pub group_tag: Tag,
    unused1: u32,
    unused2: u32,
    pub tag_index: i32
}