#![no_std]
extern crate core;
use core::result::Result;

use sea;

sea::define_sea_nd!(sea_nd_int, i32, 42);

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: i32 = sea_nd_int();
    sea::assume(v > 0);

    let x: Result<&i32, i32> = Ok(&v);
    let copied: Result<i32, i32> = x.copied();
    let result: i32 = copied.unwrap()*2;

    sea::sassert!(result > v);
}
