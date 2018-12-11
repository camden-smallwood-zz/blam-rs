use std::{fs::File, io::{Read, self}, mem};

pub trait ReadBinary: Read {
    fn read_binary<T: Copy + Sized>(&mut self) -> io::Result<T>;
}

impl ReadBinary for File {
    fn read_binary<T: Copy + Sized>(&mut self) -> io::Result<T> {
        let mut data = vec![0u8; mem::size_of::<T>()];
        self.read_exact(data.as_mut_slice())?;
        Ok(unsafe { *(data.as_ptr() as *const T) })
    }
}