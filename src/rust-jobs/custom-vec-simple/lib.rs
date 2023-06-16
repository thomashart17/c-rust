#![no_std]
use sea;


#[no_mangle]
pub extern "C" fn entrypt() {
    let v: i32 = sea::nd_i32();

    sea::assume(v > 0);

    let result: i32 = match double_if_even(v) {
        Some(v) => v,
        None => 0
    };

    if (v & 1) == 0 {
        sea::sassert!(result > v);
    } else {
        sea::sassert!(result == 0);
    }
}

fn double_if_even(num: i32) -> Option<i32> {
    if (num & 1) == 0 {
        Some(2 * num)
    } else {
        None
    }
}