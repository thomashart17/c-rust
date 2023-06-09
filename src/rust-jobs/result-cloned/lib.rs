#![no_std]
extern crate core;
use core::result::Result;

use sea;

#[no_mangle]
pub extern "C" fn entrypt() {
    let mut x: i32 = sea::nd_i32();
    sea::assume(x > 0);

    let val: Result<&mut i32, i32> = Ok(&mut x);
    let cloned: Result<i32, i32> = val.cloned();
    let result: i32 = cloned.unwrap()*2;

    sea::sassert!(result > x);
}
