use memmap::MmapMut;
use std::{fs::OpenOptions, mem};
use crate::tags::{TagGroupDefinition, TagInstance, TagInstanceHeader};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagCacheHeader {
    unused1: u32,
    pub index_offset: i32,
    pub index_count: i32,
    unused2: u32,
    pub guid: i64,
    unused3: u64
}

pub struct TagCache {
    pub mmap: MmapMut
}

impl TagCache {
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

    pub fn get_header(&mut self) -> TagCacheHeader {
        let result = self.mmap.as_mut_ptr() as *mut TagCacheHeader;
        unsafe { *result }
    }

    pub fn get_tag_instance(&mut self, tag_index: isize) -> Option<TagInstance> {
        let header = self.get_header();
        
        let tag_offsets: *mut i32 = unsafe {
            self.mmap.as_mut_ptr().offset(header.index_offset as isize) as *mut i32
        };

        if tag_offsets.is_null() {
            None
        } else {
            let tag_offset = unsafe { *tag_offsets.offset(tag_index) as isize };
            if tag_offset <= 0 {
                None
            } else {
                let mut result = TagInstance::new();

                result.header = Some(unsafe { *(self.mmap.as_mut_ptr().offset(tag_offset) as *mut TagInstanceHeader) });
                result.index = Some(tag_index as isize);
                result.offset = Some(tag_offset);

                let tag_header = result.header.unwrap();

                let tag_dependencies: *mut i32 = unsafe {
                    self.mmap.as_mut_ptr()
                        .offset(tag_offset)
                        .offset(mem::size_of::<TagInstanceHeader>() as isize) as *mut i32
                };

                for i in 0..tag_header.dependency_count {
                    result.dependencies.push(unsafe { *tag_dependencies.offset(i as isize) as isize });
                }

                let tag_data_fixups: *mut u32 = unsafe {
                    self.mmap.as_mut_ptr()
                        .offset(tag_offset)
                        .offset(mem::size_of::<TagInstanceHeader>() as isize)
                        .offset((mem::size_of::<i32>() * tag_header.dependency_count as usize) as isize) as *mut u32
                };

                for i in 0..tag_header.data_fixup_count {
                    result.data_fixups.push(unsafe { *tag_data_fixups.offset(i as isize) as usize });
                }
                
                let tag_resource_fixups: *mut u32 = unsafe {
                    self.mmap.as_mut_ptr()
                        .offset(tag_offset)
                        .offset(mem::size_of::<TagInstanceHeader>() as isize)
                        .offset((mem::size_of::<i32>() * tag_header.dependency_count as usize) as isize)
                        .offset((mem::size_of::<u32>() * tag_header.data_fixup_count as usize) as isize) as *mut u32
                };

                for i in 0..tag_header.resource_fixup_count {
                    result.resource_fixups.push(unsafe { *tag_resource_fixups.offset(i as isize) as usize });
                }
                
                Some(result)
            }
        }
    }

    pub fn get_tag_definition<T: Copy + TagGroupDefinition>(&mut self, tag_index: isize) -> Option<T> {
        let header = self.get_header();
        
        let tag_offsets: *mut i32 = unsafe {
            self.mmap.as_mut_ptr().offset(header.index_offset as isize) as *mut i32
        };

        if tag_offsets.is_null() {
            None
        } else {
            let tag_offset = unsafe { *tag_offsets.offset(tag_index) as isize };
            if tag_offset <= 0 {
                None
            } else {
                let tag_header = unsafe { *(self.mmap.as_mut_ptr().offset(tag_offset) as *mut TagInstanceHeader) };
                Some(unsafe { *(self.mmap.as_mut_ptr().offset(tag_offset + tag_header.definition_offset as isize) as *mut T) })
            }
        }
    }
}