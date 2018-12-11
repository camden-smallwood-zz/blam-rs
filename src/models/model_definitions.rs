use crate::tags::*;

tag_definition! {
    pub enum ModelDamageState {
        Default,
        MinorDamage,
        MediumDamage,
        MajorDamage,
        Destroyed
    }
}