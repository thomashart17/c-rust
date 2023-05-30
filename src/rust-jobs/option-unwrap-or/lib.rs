#![no_std]
pub use sea_rs_common::CAllocator;

extern crate core;
use core::option::Option;

#[no_mangle]
pub extern "C" fn option_unwrap_or(x: i32) -> i32 {
    let result: Option<i32> = if (x & 1) == 1 { 
        None 
    } else { 
        Some(x)
    };

    result.unwrap_or(0)
}
