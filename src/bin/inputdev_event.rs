#![no_std]
#![no_main]

use cirno_user::{event_get, DecodeType, Key, KeyType};

#[macro_use]
extern crate cirno_user;

#[no_mangle]
pub fn main() -> i32 {
    println!("Input device event test");
    loop {
        if let Some(event) = event_get() {
            if let Some(decoder_type) = event.decode() {
                println!("{:?}", decoder_type);
                if let DecodeType::Key(key, keytype) = decoder_type {
                    if key == Key::Enter && keytype == KeyType::Press {
                        break;
                    }
                }
            }
        }
    }
    0
}