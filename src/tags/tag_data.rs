#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TagData {
    pub size: i32,
    unused1: u32,
    unused2: u32,
    pub address: u32,
    unused3: u32
}