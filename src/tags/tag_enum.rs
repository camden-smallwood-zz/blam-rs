use std::marker::PhantomData;
use crate::tags::TagDefinition;

pub struct TagEnumOption<T> {
    pub name: &'static str,
    pub value: T
}

pub trait TagEnumDefinition: TagDefinition {
    type BaseType: 'static;
    fn get_options() -> Vec<TagEnumOption<Self::BaseType>>;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagEnum<BaseType: Sized, EnumType: Sized + TagEnumDefinition>(pub BaseType, PhantomData<EnumType>);