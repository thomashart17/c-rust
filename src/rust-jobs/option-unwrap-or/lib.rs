#![no_std]
use sea;


#[no_mangle]
pub extern "C" fn entrypt() {
    let v: i32 = sea::nd_i32();

    let result: Option<i32> = if (v & 1) == 1 { 
        None 
    } else { 
        Some(v)
    };

    let result = result.unwrap_or(0);

    if (v & 1) == 0 {
        sea::sassert!(result == v);
    } else {
        sea::sassert!(result == 0);
    }
}
