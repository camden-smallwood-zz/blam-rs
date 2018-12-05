use crate::{objects::*, tags::*};

tag_definition! {
    #[flags, repr(u16)]
    pub enum CrateDefinitionFlags {
        DoesNotBlockAreaOfEffect = 1 << 0,
        AttachTextureCameraHack = 1 << 1,
        Targetable = 1 << 2,
        CrateWallsBlockAreaOfEffect = 1 << 3,
        CrateBlocksDamageFlashDamageResponse = 1 << 4,
        CrateBlocksRumbleDamageResponse = 1 << 5,
        CrateTakesTopLevelAreaOfEffectDamage = 1 << 6,
        CrateBlocksForcedProjectileOverpenetration = 1 << 7,
        Unimportant = 1 << 8,
        AlwaysCheckChildrenCollision = 1 << 9,
        AllowFriendlyTeamToPassThroughInside = 1 << 10,
        AllowAllyTeamToPassThroughInside = 1 << 11,
        AllowFriendlyTeamToPassThroughOutside = 1 << 12,
        AllowAllyTeamToPassThroughOutside = 1 << 13,
        RejectAllContactPointsInside = 1 << 14,
        RejectAllContactPointsOutside = 1 << 15
    }
}

tag_definition! {
    #[group_name = "crate", group_tag = "bloc"]
    pub struct CrateDefinition : ObjectDefinition {
        pub crate_flags: TagEnum<u16, CrateDefinitionFlags>,
        unused1: TagPadding<u16>,
        unused2: TagPadding<[u32; 3]>,
        unused3: TagPadding<[u8; 4]>
    }
}