use std::{mem, str, string::ToString};

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Tag(pub i32);

impl Tag {
    pub const fn from_str<'a>(v: &'a str) -> Tag {
        Tag(((v.as_bytes()[0] as i32) << 24) |
            ((v.as_bytes()[1] as i32) << 16) |
            ((v.as_bytes()[2] as i32) << 8) |
             (v.as_bytes()[3] as i32))
    }
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        let &Tag(value) = self;
        let chars: [u8; 4] = unsafe { mem::transmute(value) };
        let mut vec = chars.to_vec();
        vec.reverse();
        String::from(str::from_utf8(&vec[0..]).unwrap())
    }
}