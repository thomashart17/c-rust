#![no_std]
pub use sea_rs_common::CAllocator;

extern crate alloc;
use alloc::string::String;

#[no_mangle]
pub extern "C" fn string_parse() -> i32 {
    let value: String = String::from("42");

    value.parse().unwrap()
}
