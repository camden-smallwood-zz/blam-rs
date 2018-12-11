use crate::tags::*;

tag_definition! {
    pub enum MaterialPropertyType {
        LightmapResolution,
        LightmapPower,
        LightmapHalfLife,
        LightmapDiffuseScale
    }
}

tag_definition! {
    pub struct MaterialProperty {
        pub property_type: TagEnum<i32, MaterialPropertyType>,
        pub int_value: i32,
        pub real_value: f32
    }
}

tag_definition! {
    pub struct Material {
        pub render_method: TagReference,
        pub properties: TagBlock<MaterialProperty>,
        unused1: TagPadding<u32>,
        pub breakable_surface_index: i8,
        unused2: TagPadding<[u8; 3]>
    }
}