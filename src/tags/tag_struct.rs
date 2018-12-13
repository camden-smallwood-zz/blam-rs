use crate::tags::TagDefinition;
use std::any::Any;

#[derive(Debug)]
pub enum TagField {
    Tag,
    String,
    LongString,
    StringId,
    CharInteger,
    ShortInteger,
    LongInteger,
    Int64Integer,
    ByteInteger,
    WordInteger,
    DWordInteger,
    QWordInteger,
    Angle,
    Point2d,
    Rectangle2d,
    RgbColor,
    ArgbColor,
    Real,
    RealFraction,
    RealPoint2d,
    RealPoint3d,
    RealVector2d,
    RealVector3d,
    RealQuaternion,
    RealEulerAngles2d,
    RealEulerAngles3d,
    RealPlane2d,
    RealPlane3d,
    RealRgbColor,
    RealArgbColor,
    RealHsvColor,
    RealAhsvColor,
    ShortBounds,
    AngleBounds,
    RealBounds,
    FractionBounds,
    TagReference,
    TagBlock,
    TagData,
    TagStruct,
    Array,
    Explanation,
    Undefined
}

#[derive(Debug)]
pub struct TagFieldInfo {
    pub name: &'static str,
    pub offset: usize,
    pub visible: bool,
    pub field: TagField
}

impl TagFieldInfo {
    pub fn get_value<T: ToString>(&self, owner: &dyn Any) -> &T {
        unsafe {
            ((owner as *const _ as *const u8).offset(self.offset as isize) as *const T).as_ref().unwrap()
        }
    }
}

pub trait TagStructDefinition: TagDefinition {
    type BaseType: TagStructDefinition;
    fn get_fields() -> &'static Vec<TagFieldInfo>;    
}

impl TagDefinition for () {
    fn get_name() -> &'static str { "" }
    fn get_size() -> usize { 0 }
}

impl TagStructDefinition for () {
    type BaseType = Self;
    fn get_fields() -> &'static Vec<TagFieldInfo> {
        unsafe {
            static mut FIELDS: Option<Vec<TagFieldInfo>> = None;
            if FIELDS.is_none() {
                FIELDS = Some(vec![]);
            }
            if let Some(ref fields) = &FIELDS {
                fields
            } else {
                panic!("An unknown error has occurred")
            }
        }
    }
}