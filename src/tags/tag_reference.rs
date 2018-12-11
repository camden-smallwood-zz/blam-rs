use crate::tags::Tag;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagReference {
    pub group_tag: Tag,
    unused1: u32,
    unused2: u32,
    pub tag_index: i32
}

impl Default for TagReference {
    fn default() -> Self {
        Self {
            group_tag: Default::default(),
            unused1: 0,
            unused2: 0,
            tag_index: -1
        }
    }
}