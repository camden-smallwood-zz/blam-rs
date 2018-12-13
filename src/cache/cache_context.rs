use crate::{resources::{ResourceCache, ResourceLocation}, tags::TagCache};
use std::{collections::HashMap, io, path::Path};

pub struct CacheContext<P: AsRef<Path>> {
    pub path: P,
    pub tag_cache: TagCache,
    pub resource_caches: HashMap<ResourceLocation, ResourceCache>
}

impl<P: AsRef<Path>> CacheContext<P> {
    pub fn open(path: P) -> io::Result<CacheContext<P>> {
        let tag_cache = TagCache::open(path.as_ref().join("tags.dat"))?;
        
        let mut resource_caches: HashMap<ResourceLocation, ResourceCache> = HashMap::new();
        resource_caches.insert(ResourceLocation::Audio, ResourceCache::open(path.as_ref().join("audio.dat"))?);
        resource_caches.insert(ResourceLocation::Resources, ResourceCache::open(path.as_ref().join("resources.dat"))?);
        resource_caches.insert(ResourceLocation::ResourcesB, ResourceCache::open(path.as_ref().join("resources_b.dat"))?);
        resource_caches.insert(ResourceLocation::Textures, ResourceCache::open(path.as_ref().join("textures.dat"))?);
        resource_caches.insert(ResourceLocation::TexturesB, ResourceCache::open(path.as_ref().join("textures_b.dat"))?);

        Ok(CacheContext { path, tag_cache, resource_caches })
    }
}