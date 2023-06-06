#![no_std]
pub use sea_rs_common::CAllocator;

extern crate core;
use core::option::Option;

use sea;

sea::define_sea_nd!(sea_nd_int, i32, 42);

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: i32 = sea_nd_int();
    let w: i32 = sea_nd_int();

    let val1: Option<i32> = if (v & 1) == 1 { 
        None 
    } else { 
        Some(v)
    };

    let val2: Option<i32> = Some(w);

    let result: i32 = val1.or(val2).unwrap();

    if (v & 1) == 0 {
        sea::sassert!(result == v);
    } else {
        sea::sassert!(result == w);
    }
}
