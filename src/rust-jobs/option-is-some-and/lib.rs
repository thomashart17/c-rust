#![feature(is_some_with)]

// #![no_std]
// pub use sea_rs_common::CAllocator;

// extern crate core;
// use core::option::Option;


#[no_mangle]
pub extern "C" fn option_is_some_and(x: i32) -> bool {
    let value: Option<i32> = if (x & 1) == 0 {
        Some(x)
    } else {
        None
    };

    value.is_some_and(|num| *num > 0)
}
