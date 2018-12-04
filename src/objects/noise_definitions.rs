use crate::tags::*;

tag_definition! {
    #[repr(i16)]
    pub enum ObjectNoiseLevel {
        Silent,
        Medium,
        Loud,
        Shout,
        Quiet
    }
}