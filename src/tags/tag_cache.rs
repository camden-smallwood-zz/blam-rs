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

    pub fn get_instance_offset(&self, tag_index: isize) -> isize {
        let header = self.get_header().unwrap();

        let tag_offsets: *const i32 = unsafe {
            self.mmap.as_ptr().offset(header.index_offset as isize) as *const i32
        };

        if tag_offsets.is_null() {
            -1
        } else {
            unsafe { *tag_offsets.offset(tag_index) as isize }
        }
    }

    pub fn get_instance_ptr<T>(&self, tag_index: isize, offset: isize) -> *const T {
        let tag_offset = self.get_instance_offset(tag_index);
        if tag_offset <= 0 {
            std::ptr::null()
        } else {
            unsafe { self.mmap.as_ptr().offset(tag_offset + offset) as *const T }
        }
    }

    pub fn get_instance_mut_ptr<T>(&mut self, tag_index: isize, offset: isize) -> *mut T {
        let tag_offset = self.get_instance_offset(tag_index);
        if tag_offset <= 0 {
            std::ptr::null_mut()
        } else {
            unsafe { self.mmap.as_mut_ptr().offset(tag_offset + offset) as *mut T }
        }
    }

    pub fn get_instance(&'a self, tag_index: isize) -> Option<&'a TagInstance> {
        unsafe { self.get_instance_ptr::<TagInstance>(tag_index, 0).as_ref() }
    }

    pub fn get_instance_mut(&'a mut self, tag_index: isize) -> Option<&'a mut TagInstance> {
        unsafe { self.get_instance_mut_ptr::<TagInstance>(tag_index, 0).as_mut() }
    }

    pub fn get_definition<T: TagGroupDefinition>(&'a self, tag_index: isize) -> Option<&'a T> {
        unsafe {
            let tag_instance = self.get_instance(tag_index).unwrap();
            self.get_instance_ptr::<T>(tag_index, tag_instance.definition_offset as isize).as_ref()
        }
    }

    pub fn get_definition_mut<T: TagGroupDefinition>(&'a mut self, tag_index: isize) -> Option<&'a mut T> {
        unsafe {
            let tag_instance = self.get_instance(tag_index).unwrap();
            self.get_instance_mut_ptr::<T>(tag_index, tag_instance.definition_offset as isize).as_mut()
        }
    }
}