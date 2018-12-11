use std::{fs::{File, OpenOptions}, io::{Seek, SeekFrom, self}, mem, ops::{Index, IndexMut}};
use crate::{io::ReadBinary, tags::{TagGroup, TagInstance}};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagCacheHeader {
    unused1: u32,
    pub index_offset: i32,
    pub instance_count: i32,
    unused2: u32,
    pub guid: i64,
    unused3: u64
}

pub struct TagCache {
    pub file: Option<File>,
    pub header: Option<TagCacheHeader>,
    pub instances: Vec<TagInstance>
}

impl TagCache {
    pub fn open(path: String) -> io::Result<TagCache> {
        let mut file = OpenOptions::new().read(true).write(true).open(path)?;
        let mut instances: Vec<TagInstance> = vec![];
        
        file.seek(SeekFrom::Start(0))?;
        let header: TagCacheHeader = file.read_binary()?;

        for index in 0..header.instance_count {
            file.seek(SeekFrom::Start(header.index_offset as u64 + (index as u64 * mem::size_of::<u32>() as u64)))?;
            let offset: u32 = file.read_binary()?;

            let mut instance = TagInstance {
                index: Some(index as usize),
                offset: Some(offset as usize),
                ..Default::default()
            };

            if offset > 0 {
                instance.read_header(&mut file)?;
            }

            instances.push(instance);
        }

        Ok(Self { file: Some(file), header: Some(header), instances })
    }

    pub fn allocate_tag(&mut self, group: TagGroup) -> &mut TagInstance {
        let index = self.instances.len();
        self.instances.push(TagInstance::new(group));
        let instance = &mut self.instances[index as usize];
        instance.index = Some(index);
        instance
    }
}

impl Index<usize> for TagCache {
    type Output = TagInstance;
    fn index(&self, index: usize) -> &TagInstance {
        &self.instances[index]
    }
}

impl IndexMut<usize> for TagCache {
    fn index_mut(&mut self, index: usize) -> &mut TagInstance {
        &mut self.instances[index]
    }
}