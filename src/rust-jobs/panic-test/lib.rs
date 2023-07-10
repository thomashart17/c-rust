#![no_std]
#![feature(core_panic)]

use sea;

#[no_mangle]
pub extern "C" fn entrypt() {
    test();
}

#[no_mangle]
fn test() {
    let mut v: i32  = sea::nd_i32();
    sea::assume(v > 0 && v <= i32::MAX-2);
    let original: i32 = v;
    let n: *mut i32 = &mut v;
    unsafe {
        *n = *n + 1;
        *n = *n + 1;
    }
    sea::sassert!(v == original + 2);

    // panic!();
    // core::panicking::panic("explicit panic");

    // False assertions call panic
    // assert!(false);
}
