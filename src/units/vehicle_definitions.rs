use crate::{math::*, tags::*, text::*, units::*};

tag_definition! {
    #[flags, repr(i32)]
    pub enum VehicleDefinitionFlags {
        NoFrictionWithDriver = 1 << 0,
        CanTriggerAutomaticOpeningDoors = 1 << 1,
        AutoaimWhenTeamless = 1 << 2,
        AiWeaponCannotRotate = 1 << 3,
        AiDoesNotRequireDriver = 1 << 4,
        AiDriverEnable = 1 << 5,
        AiDriverFlying = 1 << 6,
        AiDriverCanSidestep = 1 << 7,
        AiDriverHovering = 1 << 8,
        NoncombatVehicle = 1 << 9,
        VehicleIsChild = 1 << 10,
        BouncesAtDeathBarriers = 1 << 11,
        Hydraulics = 1 << 12
    }
}

tag_definition! {
    pub struct VehicleGear {
        pub loaded_torque_range: Bounds<f32>,
        pub peak_loaded_torque_scale: f32,
        pub past_peak_loaded_torque_exponent: f32,
        pub loaded_torque_at_max_angular_velovity: f32,
        pub loaded_torque_at_2x_max_angular_velocity: f32,
        pub cruising_torque_range: Bounds<f32>,
        pub peak_cruising_torque_scale: f32,
        pub past_peak_cruising_torque_exponent: f32,
        pub cruising_torque_at_max_angular_velovity: f32,
        pub cruising_torque_at_2x_max_angular_velocity: f32,
        pub min_time_to_upshift: f32,
        pub engine_upshift_scale: f32,
        pub gear_ratio: f32,
        pub min_time_to_downshift: f32,
        pub engine_downshift_scale: f32
    }
}

tag_definition! {
    pub struct VehicleSteeringAnimation {
        pub interpolation_scale: f32,
        pub maximum_angle: Angle
    }
}

tag_definition! {
    pub struct VehicleSteeringControl {
        pub overdampen_cusp_angle: f32,
        pub overdampen_exponent: f32
    }
}

tag_definition! {
    pub struct VehicleTurningControl {
        pub maximum_left_turn: f32,
        pub maximum_right_turn: f32,
        pub turn_rate: f32
    }
}

tag_definition! {
    pub struct VehicleEngine {
        pub engine_momentum: f32,
        pub engine_maximum_angular_velocity: f32,
        pub gears: TagBlock<VehicleGear>,
        pub gear_shift_sound: TagReference
    }
}

tag_definition! {
    pub struct VehicleHumanTankPhysics {
        pub forward_arc: Angle,
        pub flip_window: f32,
        pub pegged_fraction: f32,
        pub maximum_left_differential: f32,
        pub maximum_right_differential: f32,
        pub differential_acceleration: f32,
        pub differential_deceleration: f32,
        pub maximum_left_reverse_differential: f32,
        pub maximum_right_reverse_differential: f32,
        pub differential_reverse_acceleration: f32,
        pub differential_reverse_deceleration: f32,
        pub engine: VehicleEngine,
        pub wheel_circumference: f32,
        pub gravity_adjust: f32
    }
}

tag_definition! {
    pub struct VehicleHumanJeepPhysics {
        pub steering_control: VehicleSteeringControl,
        pub turning_control: VehicleTurningControl,
        pub engine: VehicleEngine,
        pub wheel_circumference: f32,
        pub gravity_adjust: f32
    }
}

tag_definition! {
    pub struct VehicleHumanPlanePhysics {
        pub maximum_forward_speed: f32,
        pub maximum_reverse_speed: f32,
        pub speed_acceleration: f32,
        pub speed_deceleration: f32,
        pub maximum_left_slide: f32,
        pub maximum_right_slide: f32,
        pub slide_acceleration: f32,
        pub slide_deceleration: f32,
        pub maximum_up_rise: f32,
        pub maximum_down_rise: f32,
        pub rise_acceleration: f32,
        pub rise_deceleration: f32,
        pub flying_torque_scale: f32,
        pub air_friction_deceleration: f32,
        pub thrust_scale: f32,
        pub turn_rate_scale_when_boosting: f32,
        pub maximum_roll: Angle,
        pub steering_animation: VehicleSteeringAnimation
    }
}

tag_definition! {
    #[flags, repr(u8)]
    pub enum VehicleAlienScoutFlags {
        LockedCamera = 1 << 0
    }
}

tag_definition! {
    pub struct VehicleAlienScoutGravityFunction {
        pub object_function_damage_region: StringId,
        pub anti_gravity_engine_speed_range: Bounds<f32>,
        pub engine_speed_acceleration: f32,
        pub maximum_vehicle_speed: f32
    }
}

tag_definition! {
    pub struct VehicleAlienScoutPhysics {
        pub steering_overdampen_cusp_angle: Angle,
        pub steering_overdamen_exponent: f32,
        pub maximum_forward_speed: f32,
        pub maximum_reverse_speed: f32,
        pub speed_acceleration: f32,
        pub speed_deceleration: f32,
        pub maximum_left_slide: f32,
        pub maximum_right_slide: f32,
        pub slide_acceleration: f32,
        pub slide_deceleration: f32,
        pub flags: TagEnum<u8, VehicleAlienScoutFlags>,
        pub unused: TagPadding<[u8; 3]>,
        pub drag_coeficient: f32,
        pub constant_deceleration: f32,
        pub torque_scale: f32,
        pub engine_gravity_function: VehicleAlienScoutGravityFunction,
        pub contrail_object_function: VehicleAlienScoutGravityFunction,
        pub gear_rotation_speed: Bounds<f32>,
        pub steering_animation: VehicleSteeringAnimation
    }
}

tag_definition! {
    pub struct VehicleAlienFighterPhysics {
        // TODO: define VehicleAlienFighterPhysics
    }
}

tag_definition! {
    pub struct VehicleTurretPhysics {
        // TODO: define VehicleTurretPhysics
    }
}

tag_definition! {
    pub struct VehicleBoatPhysics {
        // TODO: define VehicleBoatPhysics
    }
}

tag_definition! {
    pub struct VehicleVtolPhysics {
        // TODO: define VehicleVtolPhysics
    }
}

tag_definition! {
    pub struct VehicleChopperPhysics {
        // TODO: define VehicleChopperPhysics
    }
}

tag_definition! {
    pub struct VehicleGuardianPhysics {
        // TODO: define VehicleGuardianPhysics
    }
}

tag_definition! {
    pub struct VehiclePhysicsTypes {
        pub human_tank: TagBlock<VehicleHumanTankPhysics>,
        pub human_jeep: TagBlock<VehicleHumanJeepPhysics>,
        pub human_plane: TagBlock<VehicleHumanPlanePhysics>,
        pub alien_scout: TagBlock<VehicleAlienScoutPhysics>,
        pub alien_fighter: TagBlock<VehicleAlienFighterPhysics>,
        pub turret: TagBlock<VehicleTurretPhysics>,
        pub boat: TagBlock<VehicleBoatPhysics>,
        pub vtol: TagBlock<VehicleVtolPhysics>,
        pub chopper: TagBlock<VehicleChopperPhysics>,
        pub guardian: TagBlock<VehicleGuardianPhysics>
    }
}

tag_definition! {
    #[flags, repr(i32)]
    pub enum HavokVehiclePhysicsFlags {
        HasSuspension = 1 << 0,
        FrictionPointsTestOnlyEnvironments = 1 << 1
    }
}

tag_definition! {
    pub struct VehicleAntiGravityPoint {
        // TODO: define VehicleAntiGravityPoint
    }
}
tag_definition! {
    pub struct VehicleFrictionPoint {
        // TODO: define VehicleFrictionPoint
    }
}
tag_definition! {
    pub struct VehiclePhantomShape {
        // TODO: define VehiclePhantomShape
    }
}

tag_definition! {
    pub struct HavokVehiclePhysics {
        pub flags: TagEnum<i32, HavokVehiclePhysicsFlags>,
        pub ground_friction: f32,
        pub ground_depth: f32,
        pub ground_damp_factor: f32,
        pub ground_moving_friction: f32,
        pub ground_slope_to_stop_all_traction: f32,
        pub ground_slope_to_start_traction_loss: f32,
        pub maximum_normal_force_contribution: f32,
        pub anti_gravity_bank_lift: f32,
        pub steering_bank_reaction_scale: f32,
        pub gravity_scale: f32,
        pub radius: f32,
        unknown1: f32,
        unknown2: f32,
        unknown3: f32,
        pub anti_gravity_points: TagBlock<VehicleAntiGravityPoint>,
        pub friction_points: TagBlock<VehicleFrictionPoint>,
        pub phantom_shapes: TagBlock<VehiclePhantomShape>
    }
}

tag_definition! {
    #[repr(i8)]
    pub enum VehiclePlayerTrainingType {
        None,
        Warthog,
        WarthogTurret,
        Ghost,
        Banshee,
        Tank,
        Wraith
    }
}

tag_definition! {
    #[repr(i8)]
    pub enum VehicleSize {
        Small,
        Large
    }
}

tag_definition! {
    #[group_name = "vehicle", group_tag = "vehi"]
    pub struct VehicleDefinition : UnitDefinition {
        pub vehicle_flags: TagEnum<i32, VehicleDefinitionFlags>,
        pub physics_types: VehiclePhysicsTypes,
        pub havok_vehicle_physics: HavokVehiclePhysics,
        pub player_training_vehicle_type: TagEnum<i8, VehiclePlayerTrainingType>,
        pub vehicle_size: TagEnum<i8, VehicleSize>,
        pub complex_suspension_sample_count: i8,
        pub unused: TagPadding<u8>,
        pub minimum_flipping_angular_velocity: f32,
        pub maximum_flipping_angular_velocity: f32,
        pub crouch_transition_time: f32,
        pub hoojytsu: f32,
        pub seat_entrance_acceleration_scale: f32,
        pub seat_exit_acceleration_scale: f32,
        pub flip_time: f32,
        pub flip_over_message: StringId,
        pub suspension_sound: TagReference,
        pub special_effect: TagReference,
        pub driver_boost_damage_effect_or_response: TagReference,
        pub rider_boost_damage_effect_or_response: TagReference
    }
}