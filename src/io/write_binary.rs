use std::{fs::File, io::{self, Write}, mem};

pub trait WriteBinary: Write {
    fn write_binary<T: Copy + Sized>(&mut self, value: &T) -> io::Result<()>;
}

impl WriteBinary for File {
    fn write_binary<T: Copy + Sized>(&mut self, value: &T) -> io::Result<()> {
        let mut data = vec![0u8; mem::size_of::<T>()];
        unsafe { *(data.as_mut_ptr() as *mut T) = *value }
        self.write_all(data.as_slice())
    }
}