use std::{fs::{File, OpenOptions}, io::{self, Read, Seek, SeekFrom, Write}};
use crate::io::{ReadBinary, WriteBinary};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CacheFileHeader {
    unused1: u32,
    pub index_offset: i32,
    pub instance_count: i32,
    unused2: u32,
    pub guid: i64,
    unused3: u64
}

pub struct CacheFile {
    pub file: Option<File>,
    pub header: Option<CacheFileHeader>,
    pub offsets: Vec<Option<usize>>
}

impl CacheFile {
    pub fn open(path: String) -> io::Result<CacheFile> {
        let mut file = OpenOptions::new().read(true).write(true).open(path)?;
        
        file.seek(SeekFrom::Start(0))?;
        let header: CacheFileHeader = file.read_binary()?;

        file.seek(SeekFrom::Start(header.index_offset as u64))?;
        let mut offsets: Vec<Option<usize>> = vec![None; header.instance_count as usize];
        
        for index in 0..header.instance_count {
            let offset: u32 = file.read_binary()?;
            offsets[index as usize] = if offset == std::u32::MAX { None } else { Some(offset as usize) };
        }

        Ok(Self { file: Some(file), header: Some(header), offsets })
    }
}

impl Seek for CacheFile {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        if let Some(ref mut file) = self.file {
            file.seek(pos)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "CacheFile has not been opened"))
        }
    }
}

impl Read for CacheFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(ref mut file) = self.file {
            file.read(buf)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "CacheFile has not been opened"))
        }
    }
}

impl ReadBinary for CacheFile {
    fn read_binary<T: Copy + Sized>(&mut self) -> io::Result<T> {
        if let Some(ref mut file) = self.file {
            file.read_binary()
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "CacheFile has not been opened"))
        }
    }
}

impl Write for CacheFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(ref mut file) = self.file {
            file.write(buf)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "CacheFile has not been opened"))
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        if let Some(ref mut file) = self.file {
            file.flush()
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "CacheFile has not been opened"))
        }
    }
}

impl WriteBinary for CacheFile {
    fn write_binary<T: Copy + Sized>(&mut self, value: &T) -> io::Result<()> {
        if let Some(ref mut file) = self.file {
            file.write_binary(value)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "CacheFile has not been opened"))
        }
    }
}