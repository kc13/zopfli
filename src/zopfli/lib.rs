#![deny(trivial_casts, trivial_numeric_casts)]

extern crate libc;

pub mod cache;
pub mod deflate;
pub mod hash;
pub mod katajainen;
pub mod lz77;
pub mod squeeze;
pub mod symbols;
pub mod tree;
pub mod util;
pub mod zopfli;
