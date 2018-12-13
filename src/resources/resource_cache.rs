use std::{io, path::Path};
use crate::cache::CacheFile;

pub struct ResourceCache {
    pub file: CacheFile
}

impl ResourceCache {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<ResourceCache> {
        Ok(Self { file: CacheFile::open(path)? })
    }
}