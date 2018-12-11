use memmap::MmapMut;
use std::{fs::OpenOptions, io};
use crate::tags::{TagGroup, TagGroupDefinition};

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

static mut MMAP: Option<MmapMut> = None;

pub fn open(path: &str) -> io::Result<()> {
    let file = OpenOptions::new().read(true).write(true).open(path)?;
    if let Ok(result) = unsafe { MmapMut::map_mut(&file) } {
        unsafe { MMAP = Some(result); }
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, format!("Failed to map \"{}\"", path).as_str()))
    }
}

fn get_ptr<T>(offset: isize) -> io::Result<*const T> {
    if let Some(mmap) = unsafe { &MMAP } {
        Ok(unsafe { mmap.as_ptr().offset(offset) as *const T })
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Tag cache not loaded"))
    }
}

fn get_mut_ptr<T>(offset: isize) -> io::Result<*mut T> {
    if let Some(mmap) = unsafe { &mut MMAP } {
        Ok(unsafe { mmap.as_mut_ptr().offset(offset) as *mut T })
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Tag cache not loaded"))
    }
}

fn get_header<'a>() -> io::Result<&'a TagCacheHeader> {
    Ok(unsafe { self::get_ptr::<TagCacheHeader>(0)?.as_ref().unwrap() })
}

fn get_offset(tag_index: isize) -> io::Result<Option<isize>> {
    let header = self::get_header()?;
    if tag_index < 0 || tag_index >= header.instance_count as isize {
        Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid tag index: 0x{:X}", tag_index).to_string()))
    } else {
        let tag_offsets = self::get_ptr::<i32>(header.index_offset as isize)?;
        if tag_offsets.is_null() {
            Err(io::Error::new(io::ErrorKind::AddrNotAvailable, "Tag index address is null"))
        } else {
            let tag_offset = unsafe { *(tag_offsets.offset(tag_index)) as isize };
            if tag_offset <= 0 {
                Ok(None)
            } else {
                Ok(Some(tag_offset))
            }
        }
    }
}

pub fn get_instance<'a>(tag_index: isize) -> io::Result<Option<&'a TagInstance>> {
    if let Some(offset) = self::get_offset(tag_index)? {
        Ok(unsafe { self::get_ptr::<TagInstance>(offset)?.as_ref() })
    } else {
        Ok(None)
    }
}

pub fn get_instance_mut<'a>(tag_index: isize) -> io::Result<Option<&'a mut TagInstance>> {
    if let Some(offset) = self::get_offset(tag_index)? {
        Ok(unsafe { self::get_mut_ptr::<TagInstance>(offset)?.as_mut() })
    } else {
        Ok(None)
    }
}

pub fn get_definition<'a, T: TagGroupDefinition>(tag_index: isize) -> io::Result<&'a T> {
    if let Some(ref instance) = self::get_instance(tag_index)? {
        Ok(unsafe { self::get_ptr::<T>(self::get_offset(tag_index)?.unwrap() + instance.definition_offset as isize)?.as_ref().unwrap() })
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, format!("Tag instance 0x{:X} is null", tag_index).to_string()))
    }
}

pub fn get_definition_mut<'a, T: TagGroupDefinition>(tag_index: isize) -> io::Result<&'a mut T> {
    if let Some(ref instance) = self::get_instance(tag_index)? {
        Ok(unsafe { self::get_mut_ptr::<T>(self::get_offset(tag_index)?.unwrap() + instance.definition_offset as isize)?.as_mut().unwrap() })
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, format!("Tag instance 0x{:X} is null", tag_index).to_string()))
    }
}