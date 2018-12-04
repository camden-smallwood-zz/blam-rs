use crate::tags::*;

tag_definition! {
    #[repr(i8)]
    pub enum MeshPartType {
        NotDrawn,
        OpaqueShadowOnly,
        OpaqueShadowCasting,
        OpaqueNonshadowing,
        Transparent,
        LightmapOnly
    }
}

tag_definition! {
    #[flags, repr(u8)]
    pub enum MeshPartFlags {
        IsWaterSurface = 1,
        PerVertexLightmapPart = 2,
        RenderInZPrepass = 4,
        CanBeRenderedInDrawBundles = 8,
        DrawCullDistanceMedium = 16,
        DrawCullDistanceClose = 32,
        DrawCullRenderingShields = 64,
        DrawCullRenderingActiveCamo = 128
    }
}

tag_definition! {
    pub struct MeshPart {
        pub material_index: i16,
        pub transparent_sorting_index: i16,
        pub first_index: u16,
        pub index_count: u16,
        pub first_subpart_index: i16,
        pub subpart_count: i16,
        pub part_type: TagEnum<i8, MeshPartType>,
        pub part_flags: TagEnum<u8, MeshPartFlags>,
        pub vertex_count: u16
    }
}

tag_definition! {
    pub struct MeshSubpart {
        pub first_index: u16,
        pub index_count: u16,
        pub part_index: i16,
        pub vertex_count: u16
    }
}

tag_definition! {
    #[flags, repr(u8)]
    pub enum MeshFlags {
        HasVertexColor = 1,
        UseRegionIndexForSorting = 2,
        CanBeRenderedInDrawBundles = 4,
        IsCustomShadowCaster = 8,
        IsUnindexed = 16,
        RenderInZPrepass = 32,
        HasWater = 64,
        HasDecals = 128
    }
}

tag_definition! {
    #[repr(i8)]
    pub enum MeshVertexType {
        World,
        Rigid,
        Skinned,
        ParticleModel,
        FlatWorld,
        FlatRigid,
        FlatSkinned,
        Screen,
        Debug,
        Transparent,
        Particle,
        Contrail,
        LightVolume,
        SimpleChud,
        FancyChud,
        Decorator,
        TinyPosition,
        PatchyFog,
        Water,
        Ripple,
        Implicit,
        Beam,
        DualQuaternion
    }
}

tag_definition! {
    #[repr(i8)]
    pub enum MeshPrtType {
        None,
        Ambient,
        Linear,
        Quadratic
    }
}

tag_definition! {
    #[repr(i8)]
    pub enum MeshPrimitiveType {
        PointList,
        LineList,
        LineStrip,
        TriangleList,
        TriangleFan,
        TriangleStrip
    }
}

tag_definition! {
    pub struct MeshInstancedGeometryData {
        pub value: i16
    }
}

tag_definition! {
    pub struct MeshInstancedGeometry {
        pub section1: i16,
        pub section2: i16,
        pub data: TagBlock<MeshInstancedGeometryData>
    }
}

tag_definition! {
    pub struct MeshWaterData {
        pub value: i16
    }
}

tag_definition! {
    pub struct Mesh {
        pub parts: TagBlock<MeshPart>,
        pub subparts: TagBlock<MeshSubpart>,
        pub vertex_buffer_indices: [u16; 8],
        pub index_buffer_indices: [u16; 2],
        pub flags: TagEnum<u8, MeshFlags>,
        pub rigid_node_index: i8,
        pub vertex_type: TagEnum<i8, MeshVertexType>,
        pub prt_type: TagEnum<i8, MeshPrtType>,
        pub primitive_type: TagEnum<i8, MeshPrimitiveType>,
        unused: TagPadding<[u8; 3]>,
        pub instanced_geometry: TagBlock<MeshInstancedGeometry>,
        pub water: TagBlock<MeshWaterData>
    }
}