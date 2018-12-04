use crate::{objects::*, tags::*};

tag_definition! {
    #[flags, repr(u16)]
    pub enum CrateDefinitionFlags {
        DoesNotBlockAreaOfEffect = 1,
        AttachTextureCameraHack = 2,
        Targetable = 4,
        CrateWallsBlockAreaOfEffect = 8,
        CrateBlocksDamageFlashDamageResponse = 16,
        CrateBlocksRumbleDamageResponse = 32,
        CrateTakesTopLevelAreaOfEffectDamage = 64,
        CrateBlocksForcedProjectileOverpenetration = 128,
        Unimportant = 256,
        AlwaysCheckChildrenCollision = 512,
        AllowFriendlyTeamToPassThroughInside = 1024,
        AllowAllyTeamToPassThroughInside = 2048,
        AllowFriendlyTeamToPassThroughOutside = 4096,
        AllowAllyTeamToPassThroughOutside = 8192,
        RejectAllContactPointsInside = 16384,
        RejectAllContactPointsOutside = 32768
    }
}

tag_definition! {
    #[group_name = "crate", group_tag = "bloc"]
    pub struct CrateDefinition {
        pub object_definition: ObjectDefinition,
        pub crate_flags: TagEnum<u16, CrateDefinitionFlags>,
        unused1: TagPadding<u16>,
        unused2: TagPadding<[u32; 3]>,
        unused3: TagPadding<[u8; 4]>
    }
}