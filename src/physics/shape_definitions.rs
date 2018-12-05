use crate::{math::*, tags::*, text::*};

tag_definition! {
    pub struct PhysicsShape {
        pub name: StringId,
        pub material_index: i16,
        pub global_material_index: i16,
        pub relative_mass_scale: f32,
        pub friction: f32,
        pub restitution: f32,
        pub volume: f32,
        pub mass: f32,
        pub index: i16,
        pub phantom_index: i8,
        pub interaction_unknown: i8,
        unknown1: i32,
        pub size: i16,
        pub count: i16,
        pub offset: i32,
        unknown2: i32,
        pub radius: f32,
        unknown3: u32,
        unknown4: u32,
        unknown5: u32
    }
}

tag_definition! {
    pub struct PhysicsSphereShape : PhysicsShape {
        unknown1: i32,
        pub size: i16,
        pub count: i16,
        pub offset: i32,
        unknown2: i32,
        pub radius: f32,
        unknown3: u32,
        unknown4: u32,
        unknown5: u32,
        pub translation: Vector3d<f32>,
        pub translation_radius: f32
    }
}

tag_definition! {
    pub struct PhysicsPillShape : PhysicsShape {
        pub bottom: Vector3d<f32>,
        pub bottom_radius: f32,
        pub top: Vector3d<f32>,
        pub top_radius: f32
    }
}