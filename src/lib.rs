#![feature(const_str_as_bytes, const_transmute)]

extern crate memmap;

#[macro_use]
pub mod tags;

pub mod items;
pub mod math;
pub mod objects;
pub mod physics;
pub mod render;
pub mod text;
pub mod units;
