// #![no_std]
// pub use sea_rs_common::CAllocator;

// extern crate core;
// use core::option::Option;


#[no_mangle]
pub extern "C" fn option_or(x: i32, y: i32) -> i32 {
    let val1: Option<i32> = if (x & 1) == 1 { 
        None 
    } else { 
        Some(x)
    };

    let val2: Option<i32> = Some(y);

    val1.or(val2).unwrap()
}
