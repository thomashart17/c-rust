#![no_std]
pub use sea_rs_common;
extern crate alloc;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn option_vec_map(n: u8) -> u32 {
    let capacity: usize = n as usize;
    let mut nums: Vec<Option<u32>> = Vec::with_capacity(capacity);

    for i in 1..=capacity {
        let value: u32 = i as u32 - 1;
        nums[i - 1] = Some(value);
    }

    let result: Vec<Option<u32>> = nums.into_iter().map(square).collect();

    let mut sum: u32 = 0;
    for val in result {
        if let Some(x) = val {
            sum += x;
        }
    }

    sum
}

fn square(val: Option<u32>) -> Option<u32> {
    val.map(|x| x * x)
}
