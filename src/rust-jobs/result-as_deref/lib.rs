// #![no_std]
// pub use sea_rs_common::CAllocator;

// extern crate alloc;
// use alloc::string::String;

// extern crate core;
// use core::result::Result;

#[no_mangle]
pub extern "C" fn as_deref(input: i32) -> i32 {
    let x: Result<String, i32> = Err(input);
    let y: Result<&str, &i32> = Err(&input);

    let x_error: i32 = match x {
        Err(err) => err,
        _ => 0,
    };

    let y_error: i32 = match y {
        Err(err) => *err,
        _ => 0,
    };

    x_error + y_error
}
