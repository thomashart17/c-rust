#![no_std]
pub use sea;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn entrypt() {
    let x: i32 = sea::nd_i32();
    let y: i32 = sea::nd_i32();
    let z: i32 = sea::nd_i32();

    let values: Vec<i32> = vec![x, y, z];

    let result: i32 = values.iter().filter(|&x| (x & 1) == 0).sum();

    sea::sassert!((result & 1) == 0);
}
