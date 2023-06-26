#![no_std]

use sea;

use smallvec::SmallVec;

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: u8 = sea::nd_u8();
    match v {
        // 0 => test_new(),
        1 => test_push(),
        _ => (),
    }
}

#[no_mangle]
fn test_new() {
    let v: SmallVec<[u32; 8]> = SmallVec::new();

    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_push() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for i in 0..len {
        v.push(sea::nd_u32());
        sea::sassert!(v.len() == i + 1);
    }

    sea::sassert!(v.len() == len);
    sea::sassert!(v.capacity() == 8);
}
