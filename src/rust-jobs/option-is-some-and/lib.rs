#![feature(is_some_with)]
#![no_std]
pub use sea_rs_common::CAllocator;

extern crate core;
use core::option::Option;

use sea;

sea::define_sea_nd!(sea_nd_int, i32, 42);

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: i32 = sea_nd_int();
    sea::assume(v <= 0);
    
    let value: Option<i32> = if (v & 1) == 0 {
        Some(v)
    } else {
        None
    };

    let result: bool = value.is_some_and(|num: &i32| *num > 0);

    sea::sassert!(result == false);
}
