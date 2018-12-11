use std::marker::PhantomData;
use crate::tags::TagDefinition;

pub struct TagEnumOption {
    pub name: &'static str,
    pub value: isize
}

pub trait TagEnumDefinition: TagDefinition {
    fn get_options() -> Vec<TagEnumOption>;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagEnum<BaseType: Sized, EnumType: Sized + TagEnumDefinition>(pub BaseType, PhantomData<EnumType>);