use crate::{objects::*, tags::*};

tag_definition! {
    #[repr(i16)]
    pub enum SceneryPathfindingPolicy {
        CutOut,
        Static,
        Dynamic,
        None
    }
}

tag_definition! {
    #[flags, repr(u16)]
    pub enum SceneryFlags {
        PhysicallySimulates = 1,
        UseComplexActication = 2
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum SceneryLightmappingPolicy {
        PerVertex,
        PerPixel,
        Dynamic
    }
}

tag_definition! {
    #[group_name = "scenery", group_tag = "scen"]
    pub struct SceneryDefinition {
        pub object_definition: ObjectDefinition,
        pub pathfinding_policy: TagEnum<i16, SceneryPathfindingPolicy>,
        pub scenery_flags: TagEnum<u16, SceneryFlags>,
        pub lightmapping_policy: TagEnum<i16, SceneryLightmappingPolicy>,
        unused1: TagPadding<u16>,
        unused2: TagPadding<[u32; 2]>
    }
}
