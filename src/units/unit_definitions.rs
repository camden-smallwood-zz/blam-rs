use crate::{math::*, objects::*, tags::*, text::*};

tag_definition! {
    #[flags, repr(i32)]
    pub enum UnitDefinitionFlags {
        CircularAiming = 1,
        DestroyedAfterDying = 2,
        HalfSpeedInterpolation = 4,
        FiresFromCamera = 8,
        EntranceInsideBoundingSphere = 16,
        DoesntShowReadiedWeapon = 32,
        CausesPassengerDialogue = 64,
        ResistsPings = 128,
        MeleeAttackIsFatal = 512,
        DontRefaceDuringPings = 1024,
        HasNoAiming = 2048,
        SimpleCreature = 4096,
        ImpactMeleeAttachesToUnit = 8192,
        ImpactMeleeDiesOnShield = 16384,
        CannotOpenDoorsAutomatically = 32768,
        MeleeAttackersCannotAttach = 65536,
        NotInstantlyKilledByMelee = 131072,
        ShieldSapping = 262144,
        RunsAroundFlaming = 524288,
        Inconsequential = 1048576,
        SpecialCinematicUnit = 2097152,
        IgnoredByAutoaiming = 4194304,
        ShieldsFryInfectionForms = 8388608,
        CanDualWield = 16777216,
        ActsAsGunnerForParent = 33554432,
        ControlledByParentGunner = 67108864,
        ParentsPrimaryWeapon = 134217728,
        ParentsSecondaryWeapon = 268435456,
        UnitHasBoost = 536870912
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum UnitMetagameTeam {
        Default,
        Player,
        Human,
        Covenant,
        Flood,
        Sentinel,
        Heretic,
        Prophet,
        Guilty,
        Unused1,
        Unused2,
        Unused3,
        Unused4,
        Unused5,
        Unused6,
        Unused7
    }
}

tag_definition! {
    #[repr(i8)]
    pub enum UnitMetagameKind {
        Actor,
        Vehicle
    }
}

tag_definition! {
    #[repr(i8)]
    pub enum UnitMetagameType {
        Brute,
        Grunt,
        Jackal,
        Marine,
        Bugger,
        Hunter,
        FloodInfection,
        FloodCarrier,
        FloodCombat,
        FloodPureform,
        Sentinel,
        Elite,
        Turret,
        Mongoose,
        Warthog,
        Scorpion,
        Hornet,
        Pelican,
        Shade,
        Watchtower,
        Ghost,
        Chopper,
        Mauler,
        Wraith,
        Banshee,
        Phantom,
        Scarab,
        Guntower,
        Engineer,
        EngineerRechargeStation
    }
}

tag_definition! {
    #[repr(i8)]
    pub enum UnitMetagameClass {
        Infantry,
        Leader,
        Hero,
        Specialist,
        LightVehicle,
        HeavyVehicle,
        GiantVehicle,
        StandardVehicle
    }
}

tag_definition! {
    pub struct UnitMetagameDefinition {
        pub unit_kind: TagEnum<u8, UnitMetagameKind>,
        pub unit_type: TagEnum<i8, UnitMetagameType>,
        pub unit_class: TagEnum<i8, UnitMetagameClass>,
        unknown1: i8,
        pub points: i16,
        unknown2: i16
    }
}

tag_definition! {
    pub struct UnitCameraTrack {
        pub camera_track: TagReference
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum UnitMotionSensorBlipSize {
        Medium,
        Small,
        Large
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum UnitItemScale {
        Small,
        Medium,
        Large,
        Huge
    }
}

tag_definition! {
    pub struct UnitPosture {
        pub name: StringId,
        pub pill_offset: Vector3d<f32>
    }
}

tag_definition! {
    pub struct UnitHudInterface {
        pub hud_interface: TagReference
    }
}

tag_definition! {
    pub struct UnitDialogueVariant {
        pub variant_number: i16,
        unknown: i16,
        pub dialogue: TagReference
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum UnitGrenadeType {
        HumanFragmentation,
        CovenantPlasma,
        BruteSpike,
        Firebomb
    }
}

tag_definition! {
    pub struct UnitPoweredSeat {
        pub driver_powerup_time: f32,
        pub driver_powerdown_time: f32
    }
}

tag_definition! {
    pub struct UnitWeapon {
        pub weapon: TagReference
    }
}

tag_definition! {
    pub struct UnitTargetTrackingType {
        pub tracking_type: StringId
    }
}

tag_definition! {
    pub struct UnitTargetTracking {
        pub tracking_types: TagBlock<UnitTargetTrackingType>,
        pub acquire_time: f32,
        pub grace_time: f32,
        pub decay_time: f32,
        pub tracking_sound: TagReference,
        pub locked_sound: TagReference
    }
}

tag_definition! {
    #[flags, repr(i32)]
    pub enum UnitSeatFlags {
        Invisible = 1 << 0,
        Locked = 1 << 1,
        Driver = 1 << 2,
        Gunner = 1 << 3,
        ThirdPersonCamera = 1 << 4,
        AllowsWeapons = 1 << 5,
        ThirdPersonOnEnter = 1 << 6,
        FirstPersonCameraSlavedToGun = 1 << 7,
        AllowVehicleCommunicationAnimations = 1 << 8,
        NotValidWithoutDriver = 1 << 9,
        AllowAiNonCombatants = 1 << 10,
        BoardingSeat = 1 << 11,
        AiFiringDisabledByMaxAcceleration = 1 << 12,
        BoardingEntersSeat = 1 << 13,
        BoardingNeedAnyPassenger = 1 << 14,
        ControlsOpenAndClose = 1 << 15,
        InvalidForPlayer = 1 << 16,
        InvalidForNonPlayer = 1 << 17,
        GunnerPlayerOnly = 1 << 18,
        InvisibleUnderMajorDamage = 1 << 19,
        MeleeInstantKillable = 1 << 20,
        LeaderPreference = 1 << 21,
        AllowsExitAndDetach = 1 << 22
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum UnitAiSeatType {
        None,
        Passenger,
        Gunner,
        SmallCargo,
        LargeCargo,
        Driver
    }
}

tag_definition! {
    #[flags, repr(u16)]
    pub enum UnitCameraFlags {
        PitchBoundsAbsoluteSpace = 1,
        OnlyCollidesWithEnvironment = 2,
        HidesPlayerUnitFromCamera = 4,
        UseAimingVectorInsteadOfMarkerForward = 8
    }
}

tag_definition! {
    pub struct UnitCameraAxisAcceleration {
        unknown1: f32,
        unknown2: f32,
        unknown3: f32,
        unknown4: f32,
        unknown5: f32,
        unknown6: f32
    }
}

tag_definition! {
    pub struct UnitCameraAcceleration {
        pub maximum_camera_velocity: f32,
        pub axes_acceleration: [UnitCameraAxisAcceleration; 3]
    }
}

tag_definition! {
    pub struct UnitCamera {
        pub camera_flags: TagEnum<u16, UnitCameraFlags>,
        unused: TagPadding<u16>,
        pub camera_marker_name: StringId,
        pub camera_submerged_marker_name: StringId,
        pub pitch_auto_level: Angle,
        pub pitch_range: Bounds<Angle>,
        pub camera_tracks: TagBlock<UnitCameraTrack>,
        pub pitch_spring_bounds: Bounds<Angle>,
        pub spring_velocity: Angle,
        pub camera_acceleration: TagBlock<UnitCameraAcceleration>
    }
}

tag_definition! {
    pub struct UnitSeatAcceleration {
        pub range: Vector3d<f32>,
        pub action_scale: f32,
        pub attach_scale: f32
    }
}

tag_definition! {
    pub struct UnitSeat {
        pub flags: TagEnum<i32, UnitSeatFlags>,
        pub seat_animation: StringId,
        pub seat_marker_name: StringId,
        pub entry_marker_name: StringId,
        pub boarding_grenade_marker: StringId,
        pub boarding_grenade_string: StringId,
        pub boarding_melee_string: StringId,
        pub detach_weapon_string: StringId,
        pub ping_scale: f32,
        pub turnover_time: f32,
        pub seat_acceleration: UnitSeatAcceleration,
        pub ai_scariness: f32,
        pub ai_seat_type: TagEnum<i16, UnitAiSeatType>,
        pub boarding_seat: i16,
        pub listener_interpolation_factor: f32,
        pub yaw_rate_bounds: Bounds<f32>,
        pub pitch_rate_bounds: Bounds<f32>,
        pub pitch_interpolation_time: f32,
        pub speed_reference_bounds: Bounds<f32>,
        pub speed_exponent: f32,
        pub unit_camera: UnitCamera,
        pub hud_interface: TagBlock<UnitHudInterface>,
        pub enter_seat_string: StringId,
        pub yaw_range: Bounds<Angle>,
        pub built_in_gunner: TagReference,
        pub entry_radius: f32,
        pub entry_marker_cone_angle: Angle,
        pub entry_marker_facing_angle: Angle,
        pub maximum_relative_velocity: f32,
        pub invisible_seat_region: StringId,
        pub runtime_invisible_seat_region_index: i32
    }
}

tag_definition! {
    #[group_name = "unit", group_tag = "unit"]
    pub struct UnitDefinition : ObjectDefinition {
        pub unit_flags: TagEnum<i32, UnitDefinitionFlags>,
        pub default_team: TagEnum<i16, UnitMetagameTeam>,
        pub constant_sound_volume: TagEnum<i16, ObjectNoiseLevel>,
        pub hologram_unit: TagReference,
        pub metagame_properties: TagBlock<UnitMetagameDefinition>,
        pub integrated_light_toggle: TagReference,
        pub camera_field_of_view: Angle,
        pub camera_stiffness: f32,
        pub unit_camera: UnitCamera,
        pub sync_action_camera: UnitCamera,
        pub assassination_start_damage_response: TagReference,
        pub assassination_weapon: TagReference,
        pub assassination_weapon_stow_marker: StringId,
        pub assassination_weapon_out_marker: StringId,
        pub assassination_weapon_anchor_marker: StringId,
        pub seat_acceleration: UnitSeatAcceleration,
        pub soft_ping_threshold: f32,
        pub soft_ping_interrupt_time: f32,
        pub hard_ping_threshold: f32,
        pub hard_ping_interrupt_time: f32,
        pub feign_death_threshold: f32,
        pub feign_death_time: f32,
        pub distance_of_evade_animation: f32,
        pub distance_of_dive_animation: f32,
        pub stunned_movement_threshold: f32,
        pub feign_death_chance: f32,
        pub feign_repeat_chance: f32,
        pub spawned_turret_character: TagReference,
        pub spawned_actor_count_min: i16,
        pub spawned_actor_count_max: i16,
        pub spawned_velocity: f32,
        pub aiming_velocity_maximum: Angle,
        pub aiming_acceleration_maximum: Angle,
        pub casual_aiming_modifier: f32,
        pub looking_velocity_maximum: Angle,
        pub looking_acceleration_maximum: Angle,
        pub right_hand_node: StringId,
        pub left_hand_node: StringId,
        pub preferred_gun_node: StringId,
        pub melee_damage: TagReference,
        pub boarding_melee_damage: TagReference,
        pub boarding_melee_response: TagReference,
        pub ejection_melee_damage: TagReference,
        pub ejection_melee_response: TagReference,
        pub landing_melee_damage: TagReference,
        pub flurry_melee_damage: TagReference,
        pub obstacle_smash_melee_damage: TagReference,
        pub shield_pop_damage: TagReference,
        pub assassination_damage: TagReference,
        pub motion_sensor_blip_size: TagEnum<i16, UnitMotionSensorBlipSize>,
        pub item_scale: TagEnum<i16, UnitItemScale>,
        pub postures: TagBlock<UnitPosture>,
        pub hud_interfaces: TagBlock<UnitHudInterface>,
        pub dialogue_variants: TagBlock<UnitDialogueVariant>,
        pub motion_tracker_range_modifier: f32,
        pub grenade_angle: Angle,
        pub grenade_angle_max_elevation: Angle,
        pub grenade_angle_min_elevation: Angle,
        pub grenade_velocity: f32,
        pub grenade_type: TagEnum<i16, UnitGrenadeType>,
        pub grenade_count: i16,
        pub powered_seats: TagBlock<UnitPoweredSeat>,
        pub weapons: TagBlock<UnitWeapon>,
        pub target_tracking: TagBlock<UnitTargetTracking>,
        pub seats: TagBlock<UnitSeat>,
        pub emp_radius: f32,
        pub emp_effect: TagReference,
        pub boost_collision_damage: TagReference,
        pub boost_peak_power: f32,
        pub boost_rise_power: f32,
        pub boost_peak_time: f32,
        pub boost_fall_power: f32,
        pub boost_dead_time: f32,
        pub lipsync_attack_weight: f32,
        pub lipsync_decay_weight: f32,
        pub detach_damage: TagReference,
        pub detached_weapon: TagReference
    }
}