#![allow(clippy::cast_lossless, clippy::transmute_ptr_to_ptr)]

use std::{fs::File, io::{Seek, SeekFrom, self}};
use crate::{cache::CacheFile, io::ReadBinary, tags::{TagGroup, TagGroupDefinition}};

#[repr(C, packed)]
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

impl Default for TagInstanceHeader {
    fn default() -> Self {
        Self {
            checksum: 0,
            size: 0,
            dependency_count: 0,
            data_fixup_count: 0,
            resource_fixup_count: 0,
            unused: 0,
            definition_offset: 0,
            group: Default::default()
        }
    }
}

pub struct TagInstance {
    pub header: Option<TagInstanceHeader>,
    pub index: Option<usize>,
    pub offset: Option<u64>,
    pub dependencies: Vec<usize>,
    pub data_fixups: Vec<u64>,
    pub resource_fixups: Vec<u64>
}

impl TagInstance {
    pub fn new(group: TagGroup) -> TagInstance {
        Self {
            header: Some(TagInstanceHeader {
                group,
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    pub fn read_header(&mut self, file: &mut CacheFile) -> io::Result<()> {
        if let Some(offset) = self.offset {
            file.seek(SeekFrom::Start(offset as u64))?;
            self.header = Some(file.read_binary::<TagInstanceHeader>()?);

            for _ in 0..self.header.unwrap().dependency_count {
                self.dependencies.push(file.read_binary::<i32>()? as usize);
            }

            for _ in 0..self.header.unwrap().data_fixup_count {
                self.data_fixups.push(u64::from(file.read_binary::<u32>()?));
            }

            for _ in 0..self.header.unwrap().resource_fixup_count {
                self.resource_fixups.push(u64::from(file.read_binary::<u32>()?));
            }

            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "TagInstance has no offset"))
        }
    }

    pub fn read_definition<T: Copy + Sized + TagGroupDefinition>(&self, file: &mut File) -> io::Result<T> {
        if let Some(offset) = self.offset {
            if let Some(header) = self.header {
                file.seek(io::SeekFrom::Start((offset as u32 + header.definition_offset) as u64))?;
                Ok(file.read_binary()?)
            } else {
                Err(io::Error::new(io::ErrorKind::NotFound, "TagInstance has no header"))
            }
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "TagInstance has no offset"))
        }
    }
}

impl Default for TagInstance {
    fn default() -> Self {
        Self {
            header: None,
            index: None,
            offset: None,
            dependencies: vec![],
            data_fixups: vec![],
            resource_fixups: vec![]
        }
    }
}