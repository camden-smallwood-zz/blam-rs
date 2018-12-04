use crate::tags::TagGroup;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagInstanceHeader {
    pub checksum: u32,
    pub size: u32,
    pub dependency_count: i16,
    pub data_fixup_count: i16,
    pub resource_fixup_count: i16,
    unused: i16,
    pub definition_offset: u32,
    pub group: TagGroup
}

pub struct TagInstance {
    pub header: Option<TagInstanceHeader>,
    pub index: Option<isize>,
    pub offset: Option<isize>,
    pub dependencies: Vec<isize>,
    pub data_fixups: Vec<usize>,
    pub resource_fixups: Vec<usize>
}

impl TagInstance {
    pub fn new() -> TagInstance {
        TagInstance {
            header: None,
            index: None,
            offset: None,
            dependencies: vec![],
            data_fixups: vec![],
            resource_fixups: vec![]
        }
    }
    
    pub fn get_checksum(&self) -> Option<usize> {
        if let Some(ref header) = self.header {
            Some(header.checksum as usize)
        } else {
            None
        }
    }

    pub fn get_size(&self) -> Option<usize> {
        if let Some(ref header) = self.header {
            Some(header.size as usize)
        } else {
            None
        }
    }

    pub fn get_dependency_count(&self) -> Option<isize> {
        if let Some(ref header) = self.header {
            Some(header.dependency_count as isize)
        } else {
            None
        }
    }

    pub fn get_data_fixup_count(&self) -> Option<isize> {
        if let Some(ref header) = self.header {
            Some(header.data_fixup_count as isize)
        } else {
            None
        }
    }

    pub fn get_resource_fixup_count(&self) -> Option<isize> {
        if let Some(ref header) = self.header {
            Some(header.resource_fixup_count as isize)
        } else {
            None
        }
    }

    pub fn get_definition_offset(&self) -> Option<usize> {
        if let Some(ref header) = self.header {
            Some(header.definition_offset as usize)
        } else {
            None
        }
    }

    pub fn get_group(&self) -> Option<TagGroup> {
        if let Some(ref header) = self.header {
            Some(header.group)
        } else {
            None
        }
    }
}