use crate::{math::*, objects::*, tags::*, text::*};

tag_definition! {
    #[flags, repr(i32)]
    pub enum ProjectileDefinitionFlags {
        OrientedAlongVelocity = 1,
        AiMustUseBallisticAiming = 2,
        DetonationMaxTimeIfAttached = 4,
        HasSuperCombiningExplosion = 8,
        DamageScalesBasedOnDistance = 16,
        TravelsInstantaneously = 32,
        SteeringAdjustsOrientation = 64,
        DoNotNoiseUpSteering = 128,
        CanTrackBehindItself = 256,
        RobotronSteering = 512,
        FasterWhenOwnedByPlayer = 1024
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum ProjectileDetonationTimerStart {
        Immediately,
        AfterFirstBounce,
        WhenAtRest,
        AfterFirstBounceOffAnySurface
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum ProjectileNoiseLevel {
        Silent,
        Medium,
        Loud,
        Shout,
        Quiet
    }
}

tag_definition! {
    #[flags, repr(u16)]
    pub enum ProjectileMaterialFlags {
        CannotBeOverpenetrated = 1
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum ProjectileMaterialResponseType {
        ImpactDetonate,
        Fizzle,
        Overpenetrate,
        Attach,
        Bounce,
        BounceDud,
        FizzleRicochet
    }
}

tag_definition! {
    #[flags, repr(u16)]
    pub enum ProjectileMaterialResponseFlags {
        OnlyAgainstUnits = 1,
        NeverAgainstUnits = 2,
        OnlyAgainstBipeds = 4,
        OnlyAgainstVehicles = 8,
        NeverAgainstWussPlayers = 16,
        OnlyWhenTethered = 32,
        OnlyWhenNotTethered = 64,
        OnlyAgainstDeadBipeds = 128,
        NeverAgainstDeadBipeds = 256,
        OnlyAiProjectiles = 512,
        NeverAiProjectiles = 1024
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum ProjectileMaterialResponseEffectScale {
        Damage,
        Angle
    }
}

tag_definition! {
    pub struct ProjectileMaterialResponse {
        pub material_flags: TagEnum<u16, ProjectileMaterialFlags>,
        pub default_response: TagEnum<i16, ProjectileMaterialResponseType>,
        pub material_name: StringId,
        pub global_material_index: i16,
        unused1: TagPadding<u16>,
        pub potential_response: TagEnum<i16, ProjectileMaterialResponseType>,
        pub response_flags: TagEnum<u16, ProjectileMaterialResponseFlags>,
        pub chance_fraction: f32,
        pub between_angle: Bounds<Angle>,
        pub and_velocity: Bounds<f32>,
        pub scale_effects_by: TagEnum<i16, ProjectileMaterialResponseEffectScale>,
        unused2: TagPadding<u16>,
        pub angular_noise: Angle,
        pub velocity_noise: f32,
        pub initial_friction: f32,
        pub maximum_distance: f32,
        pub parallel_friction: f32,
        pub perpendicular_friction: f32
    }
}

tag_definition! {
    pub struct ProjectileBruteGrenadeDefinition {
        pub angular_velocity_range: Bounds<Angle>,
        pub spin_angular_velocity: Angle,
        pub angular_damping: f32,
        pub drag_angle_k: f32,
        pub drag_speed_k: f32,
        pub drag_exponent: f32,
        pub attach_sample_radius: f32,
        pub attach_acc_k: f32,
        pub attach_acc_s: f32,
        pub attach_acc_e: f32,
        pub attach_acc_damping: f32
    }
}

tag_definition! {
    pub struct ProjectileFirebombGrenadeDefinition {
        pub projection_offset: f32
    }
}

tag_definition! {
    pub struct ProjectileShotgunDefinition {
        pub amount: i16,
        pub distance: i16,
        pub accuracy: f32,
        pub spread_cone_angle: Angle
    }
}

tag_definition! {
    #[group_name = "projectile", group_tag = "proj"]
    pub struct ProjectileDefinition {
        pub object_definition: ObjectDefinition,
        pub projectile_flags: TagEnum<i32, ProjectileDefinitionFlags>,
        pub detonation_timer_starts: TagEnum<i16, ProjectileDetonationTimerStart>,
        pub impact_noise: TagEnum<i16, ProjectileNoiseLevel>,
        pub collision_radius: f32,
        pub arming_time: f32,
        pub danger_radius: f32,
        pub timer: Bounds<f32>,
        pub minimum_velocity: f32,
        pub maximum_range: f32,
        pub detonation_charge_time: f32,
        pub detonation_noise: TagEnum<i16, ProjectileNoiseLevel>,
        pub super_detonation_projectile_count: i16,
        pub super_detonation_delay: f32,
        pub detonation_started: TagReference,
        pub airborne_detonation_effect: TagReference,
        pub ground_detonation_effect: TagReference,
        pub detonation_damage: TagReference,
        pub attached_detonation_damage: TagReference,
        pub super_detonation: TagReference,
        pub super_detonation_damage: TagReference,
        pub detonation_sound: TagReference,
        pub damage_reporting_type: TagEnum<i8, DamageReportingType>,
        unused: TagPadding<[u8; 3]>,
        pub attached_super_detonation_damage: TagReference,
        pub material_effect_radius: f32,
        pub flyby_sound: TagReference,
        pub flyby_response: TagReference,
        pub impact_effect: TagReference,
        pub impact_damage: TagReference,
        pub boarding_detonation_time: f32,
        pub boarding_detonation_damage: TagReference,
        pub boarding_attached_detonation_damage: TagReference,
        pub air_gravity_scale: f32,
        pub air_damage_range: Bounds<f32>,
        pub water_gravity_scale: f32,
        pub water_damage_range: Bounds<f32>,
        pub initial_velocity: f32,
        pub final_velocity: f32,
        pub indirect_fire_velocity: f32,
        pub ai_velocity_scale: f32,
        pub ai_guided_angular_velocity_scale: f32,
        pub guided_angular_velocity: Bounds<Angle>,
        pub guided_angular_velocity_at_rest: Angle,
        pub acceleration_range: Bounds<f32>,
        pub ai_target_leading_scale: f32,
        pub targeted_leading_fraction: f32,
        pub guided_outer_range_error_radius: f32,
        pub autoaim_leading_max_lead_time: f32,
        pub material_responses: TagBlock<ProjectileMaterialResponse>,
        pub brute_grenade_properties: TagBlock<ProjectileBruteGrenadeDefinition>,
        pub firebomb_grenade_properties: TagBlock<ProjectileFirebombGrenadeDefinition>,
        pub shotgun_properties: TagBlock<ProjectileShotgunDefinition>
    }
}