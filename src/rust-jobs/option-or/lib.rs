#![no_std]
use sea;


#[no_mangle]
pub extern "C" fn entrypt() {
    let v: i32 = sea::nd_i32();
    let w: i32 = sea::nd_i32();

    let val1: Option<i32> = if (v & 1) == 1 { 
        None 
    } else { 
        Some(v)
    };

    let val2: Option<i32> = Some(w);

    let result: i32 = val1.or(val2).unwrap();

    if (v & 1) == 0 {
        sea::sassert!(result == v);
    } else {
        sea::sassert!(result == w);
    }
}
