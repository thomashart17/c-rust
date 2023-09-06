#![no_std]
pub use sea;

extern crate alloc;
use alloc::string::String;

#[no_mangle]
pub extern "C" fn entrypt() {
    let value: String = String::from("42");
    let result: i32 = value.parse().unwrap();

    sea::sassert!(result == 42);
}
