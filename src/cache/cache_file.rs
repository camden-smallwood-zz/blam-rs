use std::{cmp, fs::{File, OpenOptions}, io::{self, Error, ErrorKind, Read, Seek, SeekFrom, Write}, mem};
use crate::io::{ReadBinary, WriteBinary};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CacheFileHeader {
    pub unused1: u32,
    pub index_offset: u32,
    pub instance_count: u32,
    pub unused2: u32,
    pub guid: i64,
    pub unused3: u64
}

impl Default for CacheFileHeader {
    fn default() -> Self {
        Self{
            unused1: 0,
            index_offset: mem::size_of::<CacheFileHeader>() as u32,
            instance_count: 0,
            unused2: 0,
            guid: 0,
            unused3: 0
        }
    }
}

pub struct CacheFile {
    pub file: Option<File>,
    pub header: Option<CacheFileHeader>
}

impl CacheFile {
    const PAGE_SIZE: usize = 0x1000;

    pub fn open(path: String) -> io::Result<CacheFile> {
        let mut file = OpenOptions::new().read(true).write(true).open(path)?;
        
        file.seek(SeekFrom::Start(0))?;
        let header: CacheFileHeader = file.read_binary()?;

        Ok(Self { file: Some(file), header: Some(header) })
    }

    pub fn position(&mut self) -> io::Result<u64> {
        if let Some(ref mut file) = self.file {
            file.seek(SeekFrom::Current(0))
        } else {
            Err(Error::new(ErrorKind::NotConnected, "CacheFile has not been opened"))
        }
    }

    pub fn set_len(&mut self, length: u64) -> io::Result<()> {
        if let Some(ref mut file) = self.file {
            file.set_len(length as u64)
        } else {
            Err(Error::new(ErrorKind::NotConnected, "CacheFile has not been opened"))
        }
    }

    pub fn copy_block(&mut self, old_pos: u64, new_pos: u64, length: u64) -> io::Result<()> {
        let mut remaining = length;

        while remaining > 0 {
            let mut buffer = vec![0u8; cmp::min(Self::PAGE_SIZE, remaining as usize)];

            let offset = if new_pos > old_pos {
                remaining - buffer.len() as u64
            } else {
                length - remaining as u64
            };

            self.seek(SeekFrom::Start((old_pos + offset) as u64))?;
            self.read_exact(buffer.as_mut_slice())?;

            self.seek(SeekFrom::Start((new_pos + offset) as u64))?;
            self.write_all(buffer.as_mut_slice())?;

            remaining -= buffer.len() as u64;
        }

        Ok(())
    }

    pub fn resize_block(&mut self, offset: u64, old_length: u64, new_length: u64) -> io::Result<()> {
        let old_pos: u64;
        let new_pos: u64;
        let length: u64;

        if let Some(ref mut file) = self.file {
            let old_end_offset = offset + old_length;
            let size_delta = new_length - old_length;
            
            old_pos = old_end_offset;
            new_pos = old_end_offset + size_delta;
            length = (file.metadata()?.len() as u64) - old_end_offset;
        } else {
            return Err(Error::new(ErrorKind::Other, "CacheFile has not been opened"));
        }

        self.copy_block(old_pos, new_pos, length)
    }
}

impl Seek for CacheFile {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        if let Some(ref mut file) = self.file {
            file.seek(pos)
        } else {
            Err(Error::new(ErrorKind::Other, "CacheFile has not been opened"))
        }
    }
}

impl Read for CacheFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(ref mut file) = self.file {
            file.read(buf)
        } else {
            Err(Error::new(ErrorKind::Other, "CacheFile has not been opened"))
        }
    }
}

impl ReadBinary for CacheFile {
    fn read_binary<T: Copy + Sized>(&mut self) -> io::Result<T> {
        if let Some(ref mut file) = self.file {
            file.read_binary()
        } else {
            Err(Error::new(ErrorKind::Other, "CacheFile has not been opened"))
        }
    }
}

impl Write for CacheFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(ref mut file) = self.file {
            file.write(buf)
        } else {
            Err(Error::new(ErrorKind::Other, "CacheFile has not been opened"))
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        if let Some(ref mut file) = self.file {
            file.flush()
        } else {
            Err(Error::new(ErrorKind::Other, "CacheFile has not been opened"))
        }
    }
}

impl WriteBinary for CacheFile {
    fn write_binary<T: Copy + Sized>(&mut self, value: &T) -> io::Result<()> {
        if let Some(ref mut file) = self.file {
            file.write_binary(value)
        } else {
            Err(Error::new(ErrorKind::Other, "CacheFile has not been opened"))
        }
    }
}