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

    sea::assume((x <= y) && (x <= z));

    let mut values: Vec<i32> = vec![x, y, z];

    values.sort();
    values.reverse();

    let result: i32 = values[0];

    sea::sassert!(result >= x);
}
