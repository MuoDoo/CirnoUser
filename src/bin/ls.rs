#![no_std]
#![no_main]

#[macro_use]
extern crate cirno_user;
extern crate alloc;

use cirno_user::{ls};

#[no_mangle]
pub fn main(argc: usize, argv: &[&str]) -> i32 {
    ls();
    0
}