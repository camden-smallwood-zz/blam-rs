use std::{fmt::UpperHex, marker::PhantomData};
use crate::tags::TagDefinition;

pub struct TagFlagsBit {
    pub name: &'static str,
    pub value: isize
}

pub trait TagFlagsDefinition: TagDefinition {
    fn get_bits() -> Vec<TagFlagsBit>;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagFlags<BaseType: Sized, FlagsType: Sized + TagFlagsDefinition>(pub BaseType, PhantomData<FlagsType>);

impl<BaseType: UpperHex + Sized, FlagsType: Sized + TagFlagsDefinition> ToString for TagFlags<BaseType, FlagsType> {
    fn to_string(&self) -> String {
        format!("0x{:X}", self.0).to_string()
    }
}

impl<BaseType: Default + Sized, EnumType: Sized + TagFlagsDefinition> Default for TagFlags<BaseType, EnumType> {
    fn default() -> Self {
        TagFlags(Default::default(), PhantomData)
    }
}