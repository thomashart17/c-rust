#![no_std]
extern crate alloc;
use alloc::string::String;

extern crate core;
use core::result::Result;

use sea;

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: i32 = sea::nd_i32();

    let x: Result<String, i32> = Err(v);
    let y: Result<&str, &i32> = Err(&v);

    let x_error: i32 = match x {
        Err(err) => err,
        _ => 0,
    };

    let y_error: i32 = match y {
        Err(err) => *err,
        _ => 0,
    };

    let result: i32 = x_error + y_error;

    sea::sassert!(result == v*2);
}
