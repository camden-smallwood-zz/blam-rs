use std::marker::PhantomData;
use crate::tags::TagDefinition;

pub trait TagEnumDefinition: TagDefinition {
    type BaseType;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagEnum<BaseType: Sized, EnumType: Sized + TagEnumDefinition>(pub BaseType, PhantomData<EnumType>);