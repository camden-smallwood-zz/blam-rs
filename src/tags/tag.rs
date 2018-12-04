use std::{mem, str, string::ToString};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Tag(pub u32);

impl ToString for Tag {
    fn to_string(&self) -> String {
        let &Tag(value) = self;
        let chars: [u8; 4] = unsafe { mem::transmute(value) };
        let mut vec = chars.to_vec();
        vec.reverse();
        String::from(str::from_utf8(&vec[0..]).unwrap())
    }
}

impl<'a> From<&'a str> for Tag {
    fn from(value: &'a str) -> Tag {
        let mut vec = value.to_string().into_bytes();
        vec.reverse();
        unsafe { Tag(*(vec.as_ptr() as *const u32)) }
    }
}