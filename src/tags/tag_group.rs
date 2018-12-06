use crate::{tags::{Tag, TagStructDefinition}, text::StringId};

pub trait TagGroupDefinition: TagStructDefinition {
    const GROUP_NAME: &'static str;
    const GROUP_TAG: Tag;
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagGroup {
    pub group_tags: [Tag; 3],
    pub group_name: StringId
}

impl TagGroup {
    pub fn get_group_tag(self) -> Tag {
        self.group_tags[0]
    }

    pub fn get_parent_group_tag(self) -> Tag {
        self.group_tags[1]
    }

    pub fn get_grandparent_group_tag(self) -> Tag {
        self.group_tags[2]
    }
}
