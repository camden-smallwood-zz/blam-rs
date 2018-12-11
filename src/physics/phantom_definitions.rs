use crate::{math::*, tags::*};

tag_definition! {
    pub struct HavokPhantomShapeSphere {
        pub position: Point3d<f32>,
        pub radius: f32
    }
}

tag_definition! {
    pub struct HavokPhantomShapeMultisphere {
        pub unknown1: i32,
        pub size: i16,
        pub count: i16,
        pub overall_shape_index: i32,
        pub offset: i32,
        pub number_of_spheres: i32,
        pub unknown2: f32,
        pub unknown3: f32,
        pub unknown4: f32,
        pub spheres: [HavokPhantomShapeSphere; 8]
    }
}

tag_definition! {
    pub struct HavokPhantomShape {
        pub unknown1: i32,
        pub size: i16,
        pub count: i16,
        pub overall_shape_index: i32,
        pub offset: i32,
        pub unknown2: f32,
        pub unknown3: f32,
        pub unknown4: f32,
        pub unknown5: i32,
        pub unknown6: f32,
        pub unknown7: f32,
        pub unknown8: f32,
        pub unknown9: f32,
        pub unknown10: f32,
        pub unknown11: f32,
        pub unknown12: f32,
        pub unknown13: f32,
        pub unknown14: f32,
        pub unknown15: f32,
        pub unknown16: f32,
        pub unknown17: f32,
        pub multispheres: [HavokPhantomShapeMultisphere; 4],
        pub unknown18: f32,
        pub unknown19: f32,
        pub unknown20: f32,
        pub unknown21: f32,
        pub unknown22: f32,
        pub unknown23: f32,
        pub unknown24: f32,
        pub unknown25: f32,
        pub unknown26: f32,
        pub unknown27: f32,
        pub unknown28: f32,
        pub unknown29: f32,
        pub unknown30: f32,
        pub unknown31: f32,
        pub unknown32: f32,
        pub unknown33: f32
    }
}