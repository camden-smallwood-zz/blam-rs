use memmap::MmapMut;
use std::fs::OpenOptions;
use crate::tags::{TagGroupDefinition, TagInstance};

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
    pub mmap: MmapMut
}

impl<'a> TagCache {
    pub fn open(path: &str) -> TagCache {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .expect(format!("Failed to open \"{}\"", path).as_str());
        TagCache {
            mmap: unsafe { MmapMut::map_mut(&file).expect(format!("Failed to map \"{}\"", path).as_str()) }
        }
    }

    pub fn get_header(&'a self) -> Option<&'a TagCacheHeader> {
        unsafe { (self.mmap.as_ptr() as *const TagCacheHeader).as_ref() }
    }

    pub fn get_instance_count(&'a self) -> i32 {
        self.get_header().unwrap().instance_count
    }

    pub fn get_instance(&'a self, tag_index: isize) -> Option<&'a TagInstance> {
        let header = self.get_header().unwrap();

        let tag_offsets: *const i32 = unsafe {
            self.mmap.as_ptr().offset(header.index_offset as isize) as *const i32
        };

        if tag_offsets.is_null() {
            None
        } else {
            let tag_offset = unsafe { *tag_offsets.offset(tag_index) as isize };
            if tag_offset <= 0 {
                None
            } else {
                unsafe { (self.mmap.as_ptr().offset(tag_offset) as *const TagInstance).as_ref() }
            }
        }
    }

    pub fn get_definition<T: Copy + TagGroupDefinition>(&'a self, tag_index: isize) -> Option<&'a T> {
        let header = self.get_header().unwrap();
        
        let tag_offsets: *const i32 = unsafe {
            self.mmap.as_ptr().offset(header.index_offset as isize) as *const i32
        };

        if tag_offsets.is_null() {
            None
        } else {
            let tag_offset = unsafe { *tag_offsets.offset(tag_index) as isize };
            if tag_offset <= 0 {
                None
            } else {
                unsafe {
                    let tag_instance = (self.mmap.as_ptr().offset(tag_offset) as *const TagInstance).as_ref().unwrap();
                    (self.mmap.as_ptr().offset(tag_offset + tag_instance.definition_offset as isize) as *const T).as_ref()
                }
            }
        }
    }
}