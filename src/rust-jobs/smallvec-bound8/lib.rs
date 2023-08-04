#![no_std]

use sea;

use smallvec::SmallVec;

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: u8 = sea::nd_u8();
    match v {
        1 => test_dedup(),
        2 => test_dedup_by(),
        3 => test_dedup_by_key(),
        4 => test_remove(),
        5 => test_swap_remove(),
        _ => ()
    }
}

#[no_mangle]
fn test_dedup() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    v.dedup();

    sea::sassert!(v.len() <= len);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_dedup_by() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    v.dedup_by(|a: &mut u32, b: &mut u32| a == b);

    sea::sassert!(v.len() <= len);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_dedup_by_key() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    v.dedup_by_key(|a: &mut u32| *a);

    sea::sassert!(v.len() <= len);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_remove() {
    const CAP: usize = 8;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();
    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let remove_point1: usize = sea::nd_usize();
    sea::assume(remove_point1 < len);

    v.remove(remove_point1);

    sea::sassert!(v.len() == len - 1);
    sea::sassert!(v.capacity() == CAP);

    let remove_point2: usize = sea::nd_usize();
    sea::assume(remove_point2 < len - 1);

    v.remove(remove_point2);

    sea::sassert!(v.len() == len - 2);
    sea::sassert!(v.capacity() == CAP);

    for i in 0..len - 2 {
        v.remove(0);
        sea::sassert!(v.len() == len - 3 - i);
    }

    // v is empty, so this should panic
    v.remove(0);

    // This assertion should not be reachable since the call to remove panics.
    sea::sassert!(false);
}

#[no_mangle]
fn test_swap_remove() {
    const CAP: usize = 8;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();
    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let remove_point1: usize = sea::nd_usize();
    sea::assume(remove_point1 < len);

    v.swap_remove(remove_point1);

    sea::sassert!(v.len() == len - 1);
    sea::sassert!(v.capacity() == CAP);

    let remove_point2: usize = sea::nd_usize();
    sea::assume(remove_point2 < len - 1);

    v.swap_remove(remove_point2);

    sea::sassert!(v.len() == len - 2);
    sea::sassert!(v.capacity() == CAP);

    for i in 0..len - 2 {
        v.swap_remove(0);
        sea::sassert!(v.len() == len - 3 - i);
    }

    // v is empty, so this should panic
    v.swap_remove(0);

    // This assertion should not be reachable since the call to remove panics.
    sea::sassert!(false);
}
