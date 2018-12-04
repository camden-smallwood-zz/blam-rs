use std::marker::PhantomData;
use crate::tags::TagDefinition;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagBlock<T: TagDefinition> {
    pub count: i32,
    pub address: u32,
    unused: u32,
    phantom: PhantomData<T>
}