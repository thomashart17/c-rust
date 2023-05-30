#![no_std]
pub use sea_rs_common::CAllocator;

extern crate core;
use core::result::Result;


#[no_mangle]
pub extern "C" fn copied(x: i32) -> i32 {
    let val: i32 = x;
    let x: Result<&i32, i32> = Ok(&val);
    let copied: Result<i32, i32> = x.copied();
    copied.unwrap()*2
}
