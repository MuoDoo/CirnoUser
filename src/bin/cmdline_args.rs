#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]
extern crate cirno_user;

#[no_mangle]
pub fn main(argc: usize, argv: &[&str]) -> i32 {
    println!("argc = {}", argc);
    for (i, arg) in argv.iter().enumerate() {
        println!("argv[{}] = {}", i, arg);
    }
    0
}