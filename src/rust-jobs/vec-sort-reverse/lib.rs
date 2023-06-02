#![no_std]
pub use sea;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn vec_sort_reverse(x: i32, y: i32, z: i32) -> i32 {
    let mut values: Vec<i32> = vec![x, y, z];

    values.sort();
    values.reverse();

    values[0]
}
