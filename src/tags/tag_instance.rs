use crate::tags::TagGroup;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagInstance {
    pub checksum: u32,
    pub size: u32,
    pub dependency_count: i16,
    pub data_fixup_count: i16,
    pub resource_fixup_count: i16,
    unused: i16,
    pub definition_offset: u32,
    pub group: TagGroup
}