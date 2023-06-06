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

    let values: Vec<i32> = vec![x, y, z];

    let result: i32 = values.iter().filter(|&x| (x & 1) == 0).sum();

    sea::sassert!((result & 1) == 0);
}
