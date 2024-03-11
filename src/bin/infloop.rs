#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

extern crate cirno_user;

#[no_mangle]
pub fn main(_argc: usize, _argv: &[&str]) -> ! {
    loop {}
}
