#![no_std]

use sea;

use smallvec::SmallVec;

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: u8 = sea::nd_u8();
    match v {
        0 => test_push(),
        1 => test_retain(),
        2 => test_retain_mut(),
        3 => test_try_grow(),
        _ => ()
    }
}

#[no_mangle]
fn test_push() {
    const CAP: usize = 4;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for i in 0..len {
        v.push(sea::nd_u32());
        sea::sassert!(v.len() == i + 1);
    }

    sea::sassert!(v.len() == len);
    sea::sassert!(v.capacity() == CAP);

    for _i in len..CAP + 1 {
        v.push(sea::nd_u32());
    }

    sea::sassert!(v.len() == CAP + 1);
    sea::sassert!(v.capacity() > CAP);
}

#[no_mangle]
fn test_retain() {
    const CAP: usize = 4;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for i in 1..=len {
        let val: u32 = sea::nd_u32();
        if (i & 1) == 0 {
            sea::assume((val & 1) == 0);
        } else {
            sea::assume((val & 1) == 1);
        }
        v.push(val);
    }

    v.retain(|x| (*x & 1) == 0);

    sea::sassert!(v.len() == len >> 1);
    sea::sassert!(v.capacity() == CAP);

    v.retain(|x| (*x & 1) == 1);

    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == CAP);
}

#[no_mangle]
fn test_retain_mut() {
    const CAP: usize = 4;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for i in 1..=len {
        let val: u32 = sea::nd_u32();
        if (i & 1) == 0 {
            sea::assume((val & 1) == 0);
        } else {
            sea::assume((val & 1) == 1);
        }
        v.push(val);
    }

    v.retain(|x| (*x & 1) == 0);

    sea::sassert!(v.len() == len / 2);
    sea::sassert!(v.capacity() == CAP);

    v.retain(|x| (*x & 1) == 1);

    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == CAP);
}

#[no_mangle]
fn test_try_grow() {
    const CAP: usize = 4;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let new_cap: usize = sea::nd_usize();
    sea::assume(new_cap > CAP && new_cap <= CAP*2);

    let result: Result<(), smallvec::CollectionAllocErr> = v.try_grow(new_cap);

    sea::sassert!(result.is_ok());
    sea::sassert!(v.len() == len);
    sea::sassert!(v.capacity() == new_cap);

    let new_cap2: usize = sea::nd_usize();
    sea::assume(new_cap2 < len);

    let result2: Result<(), smallvec::CollectionAllocErr> = v.try_grow(new_cap2);

    sea::sassert!(result2.is_err());
}
