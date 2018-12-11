use crate::tags::*;

tag_definition! {
    #[repr(i16)]
    pub enum ModelDamageState {
        Default,
        MinorDamage,
        MediumDamage,
        MajorDamage,
        Destroyed
    }
}