use crate::{objects::*, tags::*};

tag_definition! {
    pub enum SceneryPathfindingPolicy {
        CutOut,
        Static,
        Dynamic,
        None
    }
}

tag_definition! {
    #[repr(flags)]
    pub enum SceneryFlags {
        PhysicallySimulates = 1 << 0,
        UseComplexActication = 1 << 1
    }
}

tag_definition! {
    pub enum SceneryLightmappingPolicy {
        PerVertex,
        PerPixel,
        Dynamic
    }
}

tag_definition! {
    #[group_name = "scenery", group_tag = "scen"]
    pub struct SceneryDefinition : ObjectDefinition {
        pub pathfinding_policy: TagEnum<i16, SceneryPathfindingPolicy>,
        pub scenery_flags: TagFlags<u16, SceneryFlags>,
        pub lightmapping_policy: TagEnum<i16, SceneryLightmappingPolicy>,
        unused1: TagPadding<u16>,
        unused2: TagPadding<[u32; 2]>
    }
}
