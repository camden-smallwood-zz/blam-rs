use crate::{math::*, tags::*};

tag_definition! {
    pub struct HavokPhantomShapeSphere {
        pub position: Point3d<f32>,
        pub radius: f32
    }
}

tag_definition! {
    pub struct HavokPhantomShapeMultisphere {
        unknown1: i32,
        pub size: i16,
        pub count: i16,
        pub overall_shape_index: i32,
        pub offset: i32,
        pub number_of_spheres: i32,
        unknown2: f32,
        unknown3: f32,
        unknown4: f32,
        pub spheres: [HavokPhantomShapeSphere; 8]
    }
}

tag_definition! {
    pub struct HavokPhantomShape {
        unknown1: i32,
        pub size: i16,
        pub count: i16,
        pub overall_shape_index: i32,
        pub offset: i32,
        unknown2: f32,
        unknown3: f32,
        unknown4: f32,
        unknown5: i32,
        unknown6: f32,
        unknown7: f32,
        unknown8: f32,
        unknown9: f32,
        unknown10: f32,
        unknown11: f32,
        unknown12: f32,
        unknown13: f32,
        unknown14: f32,
        unknown15: f32,
        unknown16: f32,
        unknown17: f32,
        pub multispheres: [HavokPhantomShapeMultisphere; 4],
        unknown18: f32,
        unknown19: f32,
        unknown20: f32,
        unknown21: f32,
        unknown22: f32,
        unknown23: f32,
        unknown24: f32,
        unknown25: f32,
        unknown26: f32,
        unknown27: f32,
        unknown28: f32,
        unknown29: f32,
        unknown30: f32,
        unknown31: f32,
        unknown32: f32,
        unknown33: f32
    }
}