#![no_std]
pub use sea;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

sea::define_sea_nd!(sea_nd_int, i32, 42);

#[no_mangle]
pub extern "C" fn entrypt() {
    let x: i32 = sea_nd_int();
    let y: i32 = sea_nd_int();
    let z: i32 = sea_nd_int();

    sea::assume((x <= y) && (x <= z));

    let mut values: Vec<i32> = vec![x, y, z];

    values.sort();
    values.reverse();

    let result: i32 = values[0];

    sea::sassert!(result >= x);
}
