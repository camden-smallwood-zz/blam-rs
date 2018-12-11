use std::{mem, str, string::ToString};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Tag(pub i32);

impl ToString for Tag {
    fn to_string(&self) -> String {
        let &Tag(value) = self;
        let chars: [u8; 4] = unsafe { mem::transmute(value) };
        let mut vec = chars.to_vec();
        vec.reverse();
        String::from(str::from_utf8(&vec[0..]).unwrap())
    }
}

impl From<&'static str> for Tag {
    fn from(value: &'static str) -> Tag {
        let b = value.as_bytes();
        Tag(((b[0] as i32) << 24) | ((b[1] as i32) << 16) | ((b[2] as i32) << 8) | (b[3] as i32))
    }
}