use crate::{math::*, tags::*, text::*};

tag_definition! {
    #[repr(i16)]
    pub enum ObjectType {
        Biped,
        Vehicle,
        Weapon,
        Equipment,
        AlternateRealityDevice,
        Terminal,
        Projectile,
        Scenery,
        Machine,
        Control,
        SoundScenery,
        Crate,
        Creature,
        Giant,
        EffectScenery
    }
}

tag_definition! {
    #[flags, repr(u16)]
    pub enum ObjectDefinitionFlags {
        DoesNotCastShadow = 1 << 0,
        SearchCardinalDirectionMaps = 1 << 1,
        Bit2 = 1 << 2,
        NotAPathfindingObstacle = 1 << 3,
        ExtensionOfParent = 1 << 4,
        DoesNotCauseCollisionDamage = 1 << 5,
        EarlyMover = 1 << 6,
        EarlyMoverLocalizedPhysics = 1 << 7,
        UseStaticMassiveLightmapSample = 1 << 8,
        ObjectScalesAttachments = 1 << 9,
        InheritsPlayersAppearance = 1 << 10,
        DeadBipedsCantLocalize = 1 << 11,
        AttachToClustersByDynamicSphere = 1 << 12,
        EffectsDoNotSpawnObjectsInMultiplayer = 1 << 13
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum LightmapShadowMode {
        Default,
        Never,
        Always,
        Blur
    }
}

tag_definition! {
    #[repr(i8)]
    pub enum SweetenerSize {
        Small,
        Medium,
        Large
    }
}

tag_definition! {
    #[repr(i8)]
    pub enum WaterDensity {
        Default,
        Least,
        Some,
        Equal,
        More,
        MoreStill,
        LotsMore
    }
}

tag_definition! {
    pub struct EarlyMoverObb {
        pub name: StringId,
        pub x_bounds: Bounds<f32>,
        pub y_bounds: Bounds<f32>,
        pub z_bounds: Bounds<f32>,
        pub angles: Angles3d
    }
}

tag_definition! {
    #[flags, repr(i32)]
    pub enum ObjectAiFlags {
        DestroyableCover = 1 << 0,
        PathfindingIgnoreWhenDead = 1 << 1,
        DynamicCover = 1 << 2
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum ObjectAiSize {
        Default,
        Tiny,
        Small,
        Medium,
        Large,
        Huge,
        Immobile
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum ObjectAiDistance {
        None,
        Down,
        Step,
        Crouch,
        Stand,
        Storey,
        Tower,
        Infinite
    }
}

tag_definition! {
    pub struct ObjectAiProperties {
        pub flags: TagEnum<i32, ObjectAiFlags>,
        pub ai_type_name: StringId,
        pub size: TagEnum<i16, ObjectAiSize>,
        pub leap_jump_speed: TagEnum<i16, ObjectAiDistance>
    }
}

tag_definition! {
    #[flags, repr(i32)]
    pub enum ObjectFunctionFlags {
        Invert = 1 << 0,
        MappingDoesNotControlsActive = 1 << 1,
        AlwaysActive = 1 << 2,
        RandomTimeOffset = 1 << 3
    }
}

tag_definition! {
    pub struct ObjectFunction {
        pub flags: TagEnum<i32, ObjectFunctionFlags>,
        pub import_name: StringId,
        pub export_name: StringId,
        pub turn_off_with: StringId,
        pub minimum_value: f32,
        pub default_function: TagFunction,
        pub scale_by: StringId
    }
}

tag_definition! {
    #[flags, repr(i32)]
    pub enum ObjectAttachmentVisionFlags {
        GameplayVisionMode = 1 << 0,
        TheaterVisionMode = 1 << 1
    }
}

tag_definition! {
    #[repr(i8)]
    pub enum ObjectAttachmentChangeColor {
        None,
        Primary,
        Secondary,
        Tertiary,
        Quaternary
    }
}

tag_definition! {
    #[flags, repr(u8)]
    pub enum ObjectAttachmentFlags {
        ForceAlwaysOn = 1 << 0,
        EffectSizeScaleFromObjectScale = 1 << 1
    }
}

tag_definition! {
    pub struct ObjectAttachment {
        pub vision_flags: TagEnum<i32, ObjectAttachmentVisionFlags>,
        pub attachment: TagReference,
        pub marker: StringId,
        pub change_color: TagEnum<i8, ObjectAttachmentChangeColor>,
        pub flags: TagEnum<u8, ObjectAttachmentFlags>,
        unused: i16,
        pub primary_scale: StringId,
        pub secondary_scale: StringId
    }
}

tag_definition! {
    pub struct ObjectWidget {
        pub widget: TagReference
    }
}

tag_definition! {
    pub struct ObjectChangeColorPermutation {
        pub weight: f32,
        pub color_bounds: Bounds<ColorRgb<f32>>,
        pub variant_name: StringId
    }
}

tag_definition! {
    #[flags, repr(i32)]
    pub enum ObjectChangeColorScaleFlags {
        BlendInHsv = 1 << 0,
        MoreColors = 1 << 1
    }
}

tag_definition! {
    pub struct ObjectChangeColorFunction {
        unused: TagPadding<u32>,
        pub scale_flags: TagEnum<i32, ObjectChangeColorScaleFlags>,
        pub color_bounds: Bounds<ColorRgb<f32>>,
        pub darken_by: StringId,
        pub scale_by: StringId
    }
}

tag_definition! {
    pub struct ObjectChangeColor {
        pub initial_permutations: TagBlock<ObjectChangeColorPermutation>,
        pub functions: TagBlock<ObjectChangeColorFunction>
    }
}

tag_definition! {
    pub struct ObjectNodeMap {
        pub target_node: i8
    }
}

tag_definition! {
    #[flags, repr(u16)]
    pub enum ObjectMultiplayerEngineFlags {
        CaptureTheFlag = 1 << 0,
        Slayer = 1 << 1,
        Oddball = 1 << 2,
        KingOfTheHill = 1 << 3,
        Juggernaut = 1 << 4,
        Territories = 1 << 5,
        Assault = 1 << 6,
        Vip = 1 << 7,
        Infection = 1 << 8,
        Bit9 = 1 << 9
    }
}

tag_definition! {
    #[repr(i8)]
    pub enum ObjectMultiplayerType {
        Ordinary,
        Weapon,
        Grenade,
        Projectile,
        Powerup,
        Equipment,
        LightLandVehicle,
        HeavyLandVehicle,
        FlyingVehicle,
        Teleporter2way,
        TeleporterSender,
        TeleporterReceiver,
        PlayerSpawnLocation,
        PlayerRespawnZone,
        HoldSpawnObjective,
        CaptureSpawnObjective,
        HoldDestinationObjective,
        CaptureDestinationObjective,
        HillObjective,
        InfectionHavenObjective,
        TerritoryObjective,
        VipBoundaryObjective,
        VipDestinationObjective,
        JuggernautDestinationObjective
    }
}

tag_definition! {
    #[flags, repr(u8)]
    pub enum ObjectMultiplayerTeleporterFlags {
        DisallowsPlayers = 1 << 0,
        AllowsLandVehicles = 1 << 1,
        AllowsHeavyVehicles = 1 << 2,
        AllowsFlyingVehicles = 1 << 3,
        AllowsProjectiles = 1 << 4
    }
}

tag_definition! {
    #[flags, repr(u8)]
    pub enum ObjectMultiplayerFlags {
        OnlyRenderInEditor = 1 << 0,
        ValidInitialPlayerSpawn = 1 << 1,
        FixedBoundaryOrientation = 1 << 2,
        InheritOwningTeamColor = 1 << 3,
        Bit4 = 1 << 4,
        Bit5 = 1 << 5,
        Bit6 = 1 << 6,
        Bit7 = 1 << 7
    }
}

tag_definition! {
    #[repr(i8)]
    pub enum ObjectMultiplayerShape {
        None,
        Sphere,
        Cylinder,
        Box
    }
}

tag_definition! {
    #[repr(i8)]
    pub enum ObjectMultiplayerSpawnTimerMode {
        Single,
        Multiple
    }
}

tag_definition! {
    pub struct ObjectMultiplayerProperties {
        pub engine_flags: TagEnum<u16, ObjectMultiplayerEngineFlags>,
        pub object_type: TagEnum<i8, ObjectMultiplayerType>,
        pub teleporter_flags: TagEnum<u8, ObjectMultiplayerTeleporterFlags>,
        unused: TagPadding<u8>,
        pub flags: TagEnum<u8, ObjectMultiplayerFlags>,
        pub shape: TagEnum<i8, ObjectMultiplayerShape>,
        pub spawn_timer_mode: TagEnum<i8, ObjectMultiplayerSpawnTimerMode>,
        pub spawn_time: i16,
        pub abandon_time: i16,
        pub width: f32,
        pub length: f32,
        pub top: f32,
        pub bottom: f32,
        unknown1: f32,
        unknown2: f32,
        unknown3: f32,
        unknown4: i32,
        unknown5: i32,
        pub child_object: TagReference,
        unknown6: i32,
        pub shaders: [TagReference; 8]
    }
}

tag_definition! {
    pub struct ObjectRevivingEquipment {
        pub equipment: TagReference
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum ObjectModelDataType {
        NotSet,
        UserDefined,
        AutoGenerated
    }
}

tag_definition! {
    pub struct ObjectModelData {
        pub data_type: TagEnum<i16, ObjectModelDataType>,
        unused: TagPadding<u16>,
        pub offset: Point3d<f32>,
        pub radius: f32
    }
}

tag_definition! {
    #[group_name = "object", group_tag = "obje"]
    pub struct ObjectDefinition {
        pub object_type: TagEnum<i16, ObjectType>,
        pub object_flags: TagEnum<u16, ObjectDefinitionFlags>,
        pub bounding_radius: f32,
        pub bounding_offset: Point3d<f32>,
        pub acceleration_scale: f32,
        pub lightmap_shadow_mode: TagEnum<i16, LightmapShadowMode>,
        pub sweetener_size: TagEnum<i8, SweetenerSize>,
        pub water_density: TagEnum<i8, WaterDensity>,
        pub runtime_flags: TagPadding<i32>,
        pub dynamic_light_sphere_radius: f32,
        pub dynamic_light_sphere_offset: Point3d<f32>,
        pub default_model_variant: StringId,
        pub model: TagReference,
        pub crate_object: TagReference,
        pub collision_damage: TagReference,
        pub early_mover_obb: TagBlock<EarlyMoverObb>,
        pub creation_effect: TagReference,
        pub material_effects: TagReference,
        pub armor_sounds: TagReference,
        pub melee_impact: TagReference,
        pub ai_properties: TagBlock<ObjectAiProperties>,
        pub functions: TagBlock<ObjectFunction>,
        pub hud_text_message_index: i16,
        unused: TagPadding<u16>,
        pub attachments: TagBlock<ObjectAttachment>,
        pub widgets: TagBlock<ObjectWidget>,
        pub change_colors: TagBlock<ObjectChangeColor>,
        pub node_maps: TagBlock<ObjectNodeMap>,
        pub multiplayer_properties: TagBlock<ObjectMultiplayerProperties>,
        pub reviving_equipment: TagBlock<ObjectRevivingEquipment>,
        pub model_data: TagBlock<ObjectModelData>
    }
}
