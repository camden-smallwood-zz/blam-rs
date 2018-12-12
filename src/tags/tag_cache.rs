use std::{io, ops::{Index, IndexMut}};
use crate::{cache::CacheFile, tags::{TagGroup, TagInstance}};

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
    pub file: CacheFile,
    pub instances: Vec<TagInstance>
}

impl TagCache {
    pub fn open(path: String) -> io::Result<TagCache> {
        let mut file = CacheFile::open(path)?;
        let mut instances: Vec<TagInstance> = vec![];
        
        for index in 0..file.header.unwrap().instance_count {
            let mut instance = TagInstance {
                index: Some(index as usize),
                offset: file.offsets[index as usize],
                ..Default::default()
            };
            instance.read_header(&mut file)?;
            instances.push(instance);
        }

        Ok(Self { file, instances })
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