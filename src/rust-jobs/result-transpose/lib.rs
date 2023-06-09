#![no_std]
extern crate alloc;
use alloc::string::String;

use sea;

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: i32 = sea::nd_i32();

    let x: Result<Option<i32>, String> = Ok(Some(v));
    let y: Option<Result<i32, String>> = Some(Ok(v));
    let result: i32 = x.transpose().unwrap().unwrap() +  y.unwrap().unwrap();

    sea::sassert!(result == v*2);
}
