use crate::tags::TagDefinition;

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

pub struct TagFieldInfo {
    pub name: String,
    pub field: TagField
}

pub trait TagStructDefinition: TagDefinition {
    fn get_fields() -> Vec<TagFieldInfo>;
}
