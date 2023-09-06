#![no_std]

use sea;

#[no_mangle]
pub extern "C" fn entrypt() {
    string_compare();
}

#[no_mangle]
fn string_compare() {
    let x1: u32 = 10;
    let x2: u32 = 20;

    let y1: u32 = f1(x1);
    let y2: u32 = f2(x2);

    sea::sassert!(y1 <= 12);
    sea::sassert!(y2 <= 23);
}

#[inline]
#[no_mangle]
fn f1(x: u32) -> u32 {
    let mut v: u32  = sea::nd_u32();
    sea::assume(v <= x);
    let original: u32 = v;
    let n: *mut u32 = &mut v;
    unsafe {
        *n = *n + 1;
        *n = *n + 1;
    }
    sea::sassert!(v == original + 2);
    v
}

#[inline(never)]
#[no_mangle]
fn f2(x: u32) -> u32 {
    let mut v: u32  = sea::nd_u32();
    sea::assume(v <= x);
    let original: u32 = v;
    let n: *mut u32 = &mut v;
    unsafe {
        *n = *n + 1;
        *n = *n + 1;
    }
    sea::sassert!(v == original + 2);
    v
}