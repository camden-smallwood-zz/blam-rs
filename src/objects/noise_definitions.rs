use crate::tags::*;

tag_definition! {
    pub enum ObjectNoiseLevel {
        Silent,
        Medium,
        Loud,
        Shout,
        Quiet
    }
}