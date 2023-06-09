#![no_std]
use sea;

#[no_mangle]
pub extern "C" fn entrypt() {
    let mut v: i32  = sea::nd_i32();
    sea::assume(v > 0);
    let original: i32 = v;

    let n: *mut i32 = &mut v;

    unsafe {
        *n = *n + 1;
        *n = *n + 1;
    }

    sea::sassert!(v == original + 2);
}
