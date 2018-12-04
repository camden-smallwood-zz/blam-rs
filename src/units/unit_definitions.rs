use crate::{math::*, objects::*, tags::*, text::*};

tag_definition! {
    #[flags, repr(i32)]
    pub enum UnitDefinitionFlags {
        None = 0
    }
}

tag_definition! {
    #[repr(i16)]
    pub enum UnitTeam {
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
    #[flags, repr(u8)]
    pub enum UnitMetagameFlags {
        None = 0
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
        pub flags: TagEnum<u8, UnitMetagameFlags>,
        pub unit_type: TagEnum<i8, UnitMetagameType>,
        pub unit_class: TagEnum<i8, UnitMetagameClass>,
        pub unknown1: i8,
        pub points: i16,
        pub unknown2: i16
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
        pub unknown: i16,
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
        None = 0
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
    pub enum UnitSeatCameraFlags {
        PitchBoundsAbsoluteSpace = 1,
        OnlyCollidesWithEnvironment = 2,
        HidesPlayerUnitFromCamera = 4,
        UseAimingVectorInsteadOfMarkerForward = 8
    }
}

tag_definition! {
    pub struct UnitCameraAcceleration {
        pub unknown1: u32,
        pub unknown2: u32,
        pub unknown3: u32,
        pub unknown4: u32,
        pub unknown5: u32,
        pub unknown6: u32,
        pub unknown7: u32,
        pub unknown8: u32,
        pub unknown9: u32,
        pub unknown10: u32,
        pub unknown11: u32,
        pub unknown12: u32,
        pub unknown13: u32,
        pub unknown14: u32,
        pub unknown15: u32,
        pub unknown16: u32,
        pub unknown17: u32,
        pub unknown18: u32,
        pub unknown19: u32
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
        pub acceleration_range: Vector3d<f32>,
        pub acceleration_action_scale: f32,
        pub acceleration_attach_scale: f32,
        pub ai_scariness: f32,
        pub ai_seat_type: TagEnum<i16, UnitAiSeatType>,
        pub boarding_seat: i16,
        pub listener_interpolation_factor: f32,
        pub yaw_rate_bounds: Bounds<f32>,
        pub pitch_rate_bounds: Bounds<f32>,
        pub pitch_interpolation_time: f32,
        pub speed_reference_bounds: Bounds<f32>,
        pub speed_exponent: f32,
        pub camera_flags: TagEnum<u16, UnitSeatCameraFlags>,
        unused: TagPadding<u16>,
        pub camera_marker_name: StringId,
        pub camera_submerged_marker_name: StringId,
        pub pitch_auto_level: Angle,
        pub pitch_range: Bounds<Angle>,
        pub camera_tracks: TagBlock<UnitCameraTrack>,
        pub pitch_spring_bounds: Bounds<Angle>,
        pub spring_velocity: Angle,
        pub camera_acceleration: TagBlock<UnitCameraAcceleration>,
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

/*tag_definition! {
    #[group_name = "unit", group_tag = "unit"]
    pub struct UnitDefinition {
        pub object_definition: ObjectDefinition,
        //
        // TODO: Add unit definition fields here
        //
    }
}*/