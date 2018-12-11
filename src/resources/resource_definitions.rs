use crate::tags::*;

tag_definition! {
    #[repr(flags)]
    pub enum RawPageFlags {
        UseChecksum = 1 << 0,
        InResources = 1 << 1,
        InTextures = 1 << 2,
        InTexturesB = 1 << 3,
        InAudio = 1 << 4,
        InResourcesB = 1 << 5,
        Unknown6 = 1 << 6,
        UseChecksum2 = 1 << 7
    }
}

tag_definition! {
    pub struct RawPage {
        pub salt: u16,
        pub flags: TagFlags<u8, RawPageFlags>,
        pub compression_codec_index: i8,
        pub index: i32,
        pub compressed_block_size: u32,
        pub uncompressed_block_size: u32,
        pub crc_checksum: u32,
        pub block_asset_count: u16,
        unused: TagPadding<[u8; 14]>
    }
}

tag_definition! {
    #[repr(flags)]
    pub enum TagResourceFlags {
        Bit0 = 1 << 0,
        ResourceValid = 1 << 1,
        Bit2 = 1 << 2,
        Bit3 = 1 << 3,
        Bit4 = 1 << 4,
        Bit5 = 1 << 5,
        Bit6 = 1 << 6,
        Bit7 = 1 << 7
    }
}

tag_definition! {
    pub struct TagResourceFixup {
        pub block_offset: u32,
        pub address: u32
    }
}

tag_definition! {
    pub struct TagResourceDefinitionFixup {
        pub address: u32,
        pub resource_structure_type_index: i32
    }
}

tag_definition! {
    pub struct TagResource {
        pub owner: TagReference,
        pub salt: u16,
        pub resource_type_index: i8,
        pub flags: TagFlags<u8, TagResourceFlags>,
        pub definition_data: TagData,
        pub definition_address: u32,
        pub resource_fixups: TagBlock<TagResourceFixup>,
        pub resource_definition_fixups: TagBlock<TagResourceDefinitionFixup>,
        unused: TagPadding<u32>
    }
}

tag_definition! {
    pub struct PageableResource {
        pub page: RawPage,
        pub resource: TagResource
    }
}

tag_definition! {
    pub enum ResourceLocation {
        None = -1,
        Resources,
        Textures,
        TexturesB,
        Audio,
        ResourcesB,
        RenderModels,
        Lightmaps
    }
}