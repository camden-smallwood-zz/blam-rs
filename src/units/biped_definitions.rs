use crate::{math::*, physics::*, tags::*, text::*, units::*};

tag_definition! {
    #[repr(flags)]
    pub enum BipedDefinitionFlags {
        TurnsWithoutAnimating = 1 << 0,
        PassesThroughOtherBipeds = 1 << 1,
        ImmuneToFallingDamage = 1 << 2,
        RotateWhileAirborne = 1 << 3,
        UseLimpBodyPhysics = 1 << 4,
        UnusedBit5 = 1 << 5,
        RandomSpeedIncrease = 1 << 6,
        UnusedBit7 = 1 << 7,
        SpawnDeathChildrenOnDestroy = 1 << 8,
        StunnedByEmpDamage = 1 << 9,
        DeadPhysicsWhenStunned = 1 << 10,
        AlwaysRagdollWhenDead = 1 << 11
    }
}

tag_definition! {
    pub struct BipedCameraHeight {
        pub class: StringId,
        pub standing_height_fraction: f32,
        pub crouching_height_fraction: f32,
        pub unknown: u32,
        pub unknown2: u32,
        pub unknown3: u32
    }
}

tag_definition! {
    #[repr(flags)]
    pub enum BipedLockOnFlags {
        LockedByHumanTargeting = 1 << 0,
        LockedByPlasmaTargeting = 1 << 1,
        AlwaysLockedByHumanTargeting = 1 << 2
    }
}

tag_definition! {
    pub struct BipedMovementGate {
        pub period: f32,
        pub z_offset: f32,
        pub constant_z_offset: f32,
        pub y_offset: f32,
        pub default_function: TagFunction
    }
}

tag_definition! {
    #[repr(flags)]
    pub enum BipedPhysicsFlags {
        CenteredAtOrigin = 1 << 0,
        ShapeSpherical = 1 << 1,
        UsePlayerPhysics = 1 << 2,
        Unknown = 1 << 3,
        ClimbAnySurface = 1 << 4,
        Flying = 1 << 5,
        NotPhysical = 1 << 6,
        DeadCharacterCollisionGroup = 1 << 7,
        SuppressGroundPlanesOnBipeds = 1 << 8,
        PhysicalRagdoll = 1 << 9,
        DoNotResizeDeadSpheres = 1 << 10,
        MultipleShapes = 1 << 11,
        ExtremeSlipSurface = 1 << 12,
        SlipsOffMovers = 1 << 13,
        AlignsWithGround = 1 << 14
    }
}

tag_definition! {
    pub struct BipedPhysics {
        pub physics_flags: TagFlags<i32, BipedPhysicsFlags>,
        pub height_standing: f32,
        pub height_crouching: f32,
        pub radius: f32,
        pub mass: f32,
        pub living_material_name: StringId,
        pub dead_material_name: StringId,
        pub living_material_global_index: i16,
        pub dead_material_global_index: i16,
        pub dead_sphere_shapes: TagBlock<PhysicsSphereShape>,
        pub pill_shapes: TagBlock<PhysicsPillShape>,
        pub sphere_shapes: TagBlock<PhysicsSphereShape>,
        pub maximum_slope_angle: Angle<f32>,
        pub downhill_falloff_angle: Angle<f32>,
        pub downhill_cutoff_angle: Angle<f32>,
        pub uphill_falloff_angle: Angle<f32>,
        pub uphill_cutoff_angle: Angle<f32>,
        pub downhill_velocity_scale: f32,
        pub uphill_velocity_scale: f32,
        unknown42: f32,
        unknown43: f32,
        unknown44: f32,
        unknown45: f32,
        unknown46: f32,
        unknown47: f32,
        unknown48: f32,
        unknown49: f32,
        unknown50: f32,
        unknown51: f32,
        pub bank_angle: Angle<f32>,
        pub bank_apply_time: f32,
        pub bank_decay_time: f32,
        pub pitch_ratio: f32,
        pub maximum_velocity: f32,
        pub maximum_sidestep_velocity: f32,
        pub acceleration: f32,
        pub deceleration: f32,
        pub angular_velocity_maximum: Angle<f32>,
        pub angular_acceleration_maximum: Angle<f32>,
        pub crouch_velocity_modifier: f32
    }
}

tag_definition! {
    pub struct BipedContactPoint {
        pub marker_name: StringId
    }
}

tag_definition! {
    #[repr(flags)]
    pub enum BipedLeapingFlags {
        ForceEarlyRoll = 1 << 0
    }
}

tag_definition! {
    pub struct BipedLeaping {
        pub flags: TagFlags<i32, BipedLeapingFlags>,
        pub dampening_scale: f32,
        pub roll_delay: f32,
        pub cannonball_off_axis_scale: f32,
        pub cannonball_off_track_scale: f32,
        pub cannonball_roll_bounds: Bounds<Angle<f32>>,
        pub anticipation_ratio_bounds: Bounds<f32>,
        pub reaction_force_bounds: Bounds<f32>,
        pub lobbing_desire: f32
    }
}

tag_definition! {
    pub struct BipedGroundFitting {
        unknown65: f32,
        unknown66: f32,
        unknown67: f32,
        unknown68: f32,
        unknown69: f32,
        unknown70: Angle<f32>,
        unknown71: Angle<f32>,
        unknown72: f32,
        unknown73: f32,
        unknown74: f32,
        unknown75: f32,
        unknown76: f32
    }
}

tag_definition! {
    #[group_name = "biped", group_tag = "bipd"]
    pub struct BipedDefinition : UnitDefinition {
        pub moving_turning_speed: Angle<f32>,
        pub biped_flags: TagFlags<i32, BipedDefinitionFlags>,
        pub stationary_turning_threshold: Angle<f32>,
        unknown1: u32,
        unknown2: StringId,
        pub jump_velocity: f32,
        pub maximum_soft_landing_time: f32,
        pub minimum_hard_landing_time: f32,
        pub minimum_soft_landing_velocity: f32,
        pub minimum_hard_landing_velocity: f32,
        pub maximum_hard_landing_velocity: f32,
        pub death_hard_landing_velocity: f32,
        pub stun_duration: f32,
        pub stationary_standing_camera_height: f32,
        pub moving_standing_camera_height: f32,
        pub stationary_crouching_camera_height: f32,
        pub moving_crouching_camera_height: f32,
        pub crouch_transition_time: f32,
        pub crouching_camera_function: TagFunction,
        pub camera_heights: TagBlock<BipedCameraHeight>,
        pub camera_interpolation_start: Angle<f32>,
        pub camera_interpolation_end: Angle<f32>,
        unknown22: u32,
        unknown23: u32,
        unknown24: u32,
        unknown25: u32,
        pub autoaim_width: f32,
        pub lock_on_flags: TagFlags<i32, BipedLockOnFlags>,
        pub lock_on_distance: u32,
        pub physics_control_node_index: i16,
        unknown29: i16,
        unknown30: u32,
        unknown31: u32,
        unknown32: u32,
        pub pelvis_node_index: i16,
        pub head_node_index: i16,
        unknown33: u32,
        pub headshot_acceleration_scale: f32,
        pub area_damage_effect: TagReference,
        pub movement_gates: TagBlock<BipedMovementGate>,
        pub movement_gates_crouching: TagBlock<BipedMovementGate>,
        unknown36: u32,
        unknown37: u32,
        unknown38: u32,
        unknown39: u32,
        unknown40: u32,
        unknown41: u32,
        pub biped_physics: BipedPhysics,
        pub contact_points: TagBlock<BipedContactPoint>,
        pub reanimation_character: TagReference,
        pub transformation_muffin: TagReference,
        pub death_spawn_character: TagReference,
        pub death_spawn_count: i16,
        unused1: TagPadding<u16>,
        pub leaping: BipedLeaping,
        pub ground_fitting: BipedGroundFitting,
        unused2: TagPadding<u32>
    }
}