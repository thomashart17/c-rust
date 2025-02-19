#![feature(is_some_with)]
#![no_std]
use sea;


#[no_mangle]
pub extern "C" fn entrypt() {
    let v: i32 = sea::nd_i32();
    sea::assume(v <= 0);
    
    let value: Option<i32> = if (v & 1) == 0 {
        Some(v)
    } else {
        None
    };

    let result: bool = value.is_some_and(|num: &i32| *num > 0);

    sea::sassert!(result == false);
}
