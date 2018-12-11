#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagData {
    pub size: i32,
    unused1: u32,
    unused2: u32,
    pub address: u32,
    unused3: u32
}

impl Default for TagData {
    fn default() -> Self {
        Self {
            size: 0,
            unused1: 0,
            unused2: 0,
            address: 0,
            unused3: 0
        }
    }
}