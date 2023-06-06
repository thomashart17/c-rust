#![no_std]
use sea;

sea::define_sea_nd!(sea_nd_int, i32, 42);

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: i32 = sea_nd_int();
    sea::assume(v > 0);

    let result: Option<i32> = if (v & 1) == 1 {
        None
    } else {
        Some(v)
    };

    let result = match double(result) {
        Some(val) => val,
        None => 0
    };

    if (v & 1) == 0 {
        sea::sassert!(result > v);
    } else {
        sea::sassert!(result == 0);
    }
}

fn double(x: Option<i32>) -> Option<i32> {
    x.and_then(|num: i32| Some(num * 2))
}
