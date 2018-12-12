use std::io;
use crate::cache::CacheFile;

pub struct ResourceCache {
    pub file: CacheFile
}

impl ResourceCache {
    pub fn open(path: String) -> io::Result<ResourceCache> {
        Ok(Self { file: CacheFile::open(path)? })
    }
}