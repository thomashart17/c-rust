#![no_std]
extern crate core;
use core::result::Result;

use sea;

sea::define_sea_nd!(sea_nd_int, i32, 42);

#[no_mangle]
pub extern "C" fn entrypt() {
    let mut x: i32 = sea_nd_int();
    sea::assume(x > 0);

    let val: Result<&mut i32, i32> = Ok(&mut x);
    let cloned: Result<i32, i32> = val.cloned();
    let result: i32 = cloned.unwrap()*2;

    sea::sassert!(result > x);
}
