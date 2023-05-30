#![no_std]
pub use sea_rs_common::CAllocator;

extern crate alloc;
use alloc::string::String;

extern crate core;
use core::result::Result;


#[no_mangle]
pub extern "C" fn divide_and_multiply(x: i32, y: i32) -> i32 {
    let result = divide(x, y)
        .and_then(multiply_by_two);

    match result {
        Ok(value) => value,
        Err(_) => -1,
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else if a < 0 || b < 0 {
        Err(String::from("Numbers must be positive"))
    } else {
        Ok(a / b)
    }
}

fn multiply_by_two(n: i32) -> Result<i32, String> {
    Ok(n * 2)
}
