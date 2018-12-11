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