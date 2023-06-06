#![no_std]
pub use sea_rs_common::CAllocator;
pub use sea;

extern crate alloc;
use alloc::vec;


#[no_mangle]
pub extern "C" fn entrypt() {
    let mut v: vec::Vec<i32> = vec![1, 2, 3]; // L = 3

    v.push(4); // L = 4
    v.push(5); // L = 5
    v.push(6); // L = 6
    v.pop(); // L = 5

    let result: usize = v.len();

    sea::sassert!(result == 5);
}
