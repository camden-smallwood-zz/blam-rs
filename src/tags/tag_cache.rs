use std::{io::{self, Error, ErrorKind, Read, Seek, SeekFrom, Write}, mem, ops::{Index, IndexMut}, u32};
use crate::{cache::{CacheFile, CacheFileHeader}, io::{ReadBinary, WriteBinary}, tags::{TagGroup, TagInstance, TagInstanceHeader}};

pub struct TagCache {
    pub file: CacheFile,
    pub instances: Vec<TagInstance>
}

impl TagCache {
    pub fn open(path: String) -> io::Result<TagCache> {
        let mut file = CacheFile::open(path)?;
        let mut instances: Vec<TagInstance> = vec![];
        
        file.seek(SeekFrom::Start(u64::from(file.header.unwrap().index_offset)))?;
        let mut offsets: Vec<Option<u64>> = vec![None; file.header.unwrap().instance_count as usize];
        
        for index in 0..file.header.unwrap().instance_count {
            let offset: u32 = file.read_binary()?;
            offsets[index as usize] = match offset {
                0u32 | u32::MAX => None,
                _ => Some(u64::from(offset))
            };
        }

        for index in 0..file.header.unwrap().instance_count {
            let mut instance = TagInstance {
                index: Some(index as usize),
                offset: offsets[index as usize],
                ..Default::default()
            };
            instance.read_header(&mut file)?;
            instances.push(instance);
        }

        Ok(Self { file, instances })
    }

    pub fn allocate_tag(&mut self, group: TagGroup) -> &mut TagInstance {
        let index = self.instances.len();
        self.instances.push(TagInstance::new(group));
        let instance = &mut self.instances[index as usize];
        instance.index = Some(index);
        instance
    }

    pub fn extract_tag_data(&mut self, index: usize, writer: &mut dyn Write) -> io::Result<()> {
        if index >= self.instances.len() {
            Err(Error::new(ErrorKind::InvalidInput, format!("Invalid tag index: {}", index).to_string()))
        } else {
            let instance = &self.instances[index];
            if let Some(offset) = instance.offset {
                let mut buffer = vec![0u8; instance.header.unwrap().size as usize];
                self.file.seek(SeekFrom::Start(offset as u64))?;
                self.file.read_exact(buffer.as_mut_slice())?;
                writer.write_all(buffer.as_mut_slice())
            } else {
                Err(Error::new(ErrorKind::NotFound, "TagInstance has no offset"))
            }
        }
    }

    pub fn import_tag_data(&mut self, index: usize, reader: &mut (impl ReadBinary + Seek)) -> io::Result<()> {
        if index >= self.instances.len() {
            Err(Error::new(ErrorKind::InvalidInput, format!("Invalid tag index: {}", index).to_string()))
        } else {
            if self.instances[index].offset.is_none() {
                self.instances[index].offset = Some(self.get_new_tag_offset(index)?);
            }
            
            let new_header: TagInstanceHeader = reader.read_binary()?;
            reader.seek(SeekFrom::Current(0 - mem::size_of::<TagInstanceHeader>() as i64))?;
            
            let mut buffer = vec![0u8; new_header.size as usize];
            reader.read_exact(buffer.as_mut_slice())?;
            
            let instance = &mut self.instances[index];
            let old_end_offset = instance.offset.unwrap() + u64::from(instance.header.unwrap().size);
            let size_delta = new_header.size as isize - instance.header.unwrap().size as isize;

            self.file.resize_block(
                instance.offset.unwrap() as u64,
                u64::from(instance.header.unwrap().size),
                u64::from(new_header.size))?;
            
            self.file.seek(SeekFrom::Start(instance.offset.unwrap() as u64))?;
            self.file.write_all(buffer.as_mut_slice())?;

            instance.read_header(&mut self.file)?;
            self.update_tag_offsets(old_end_offset, size_delta)
        }
    }
    
    fn get_tag_data_end_offset(&self) -> io::Result<u64> {
        if self.file.header.is_some() {
            let mut end_offset = mem::size_of::<CacheFileHeader>() as u64;
            for instance in &self.instances {
                if let Some(ref header) = instance.header {
                    end_offset += u64::from(header.size);
                }
            }
            Ok(end_offset)
        } else {
            Err(Error::new(ErrorKind::NotConnected, "TagCache has not been opened"))
        }
    }

    fn get_new_tag_offset(&self, index: usize) -> io::Result<u64> {
        if self.file.header.is_some() {
            let tag_count = self.instances.len();
            if tag_count > 0 && index < (tag_count - 1) {
                for i in (0..(index - 1)).rev() {
                    if let Some(offset) = self.instances[i].offset {
                        if let Some(ref header) = self.instances[i].header {
                            return Ok(offset + u64::from(header.size));
                        }
                    }
                }
            }
            self.get_tag_data_end_offset()
        } else {
            Err(Error::new(ErrorKind::NotConnected, "TagCache has not been opened"))
        }
    }

    fn update_tag_offsets(&mut self, start_offset: u64, size_delta: isize) -> io::Result<()> {
        let index_offset = self.get_tag_data_end_offset()?;
        self.file.seek(SeekFrom::Start(index_offset as u64))?;
        for instance in &mut self.instances {
            if let Some(ref mut offset) = instance.offset {
                if *offset >= start_offset {
                    *offset = (*offset as isize + size_delta) as u64;
                }
                self.file.write_binary(&(*offset as u32))?;
            } else {
                self.file.write_binary(&0u32)?;
            }
        }
        let position = self.file.position()?;
        self.file.set_len(position)?;
        self.update_header(index_offset as u32)
    }

    fn update_header(&mut self, index_offset: u32) -> io::Result<()> {
        let instance_count = self.instances.len() as u32;
        if self.file.header.is_none() {
            self.file.header = Some(CacheFileHeader { index_offset, instance_count, ..Default::default() });
        }
        if let Some(ref mut header) = self.file.header {
            header.index_offset = index_offset;
            header.instance_count = instance_count;
        }
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_binary(&self.file.header.unwrap())
    }
}

impl Index<usize> for TagCache {
    type Output = TagInstance;
    fn index(&self, index: usize) -> &TagInstance {
        &self.instances[index]
    }
}

impl IndexMut<usize> for TagCache {
    fn index_mut(&mut self, index: usize) -> &mut TagInstance {
        &mut self.instances[index]
    }
}