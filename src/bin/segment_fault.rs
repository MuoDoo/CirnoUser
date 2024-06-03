#![no_std]
#![no_main]

#[macro_use]
extern crate cirno_user;

#[no_mangle]
fn main() -> i32 {
    println!("Kernel should kill this application!");
    unsafe {
        core::ptr::null_mut::<u8>().write_volatile(0);
    }
    0
}
