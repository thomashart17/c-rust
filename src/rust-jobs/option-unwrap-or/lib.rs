#![no_std]
use sea;

sea::define_sea_nd!(sea_nd_int, i32, 42);

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: i32 = sea_nd_int();

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
