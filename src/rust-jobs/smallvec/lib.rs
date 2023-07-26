#![no_std]

use sea;

use core::mem::{self, MaybeUninit};
use smallvec::SmallVec;

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: u8 = sea::nd_u8();
    match v {
        0 => test_clear(),
        1 => test_extend_from_slice(),
        2 => test_from_buf(),
        3 => test_from_buf_and_len(),
        4 => test_from_buf_and_len_unchecked(),
        5 => test_from_const(),
        6 => test_from_elem(),
        7 => test_from_raw_parts(),
        8 => test_from_slice(),
        9 => test_grow(),
        10 => test_insert(),
        11 => test_insert_from_slice(),
        12 => test_new(),
        13 => test_new_const(),
        14 => test_pop(),
        15 => test_reserve(),
        16 => test_reserve_exact(),
        17 => test_set_len(),
        18 => test_truncate(),
        19 => test_try_reserve(),
        20 => test_try_reserve_exact(),
        21 => test_with_capacity(),
        _ => (),
    }
}

#[no_mangle]
fn test_clear() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    v.clear();

    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_extend_from_slice() {
    let mut v1: SmallVec<[u32; 8]> = SmallVec::new();
    let mut v2: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v1.push(sea::nd_u32());
    }

    let len2: usize = sea::nd_usize();
    sea::assume(len2 <= 8);

    for _i in 0..len2 {
        v2.push(sea::nd_u32());
    }

    v1.extend_from_slice(&v2);

    sea::sassert!(v1.len() == len + len2);
    sea::sassert!(v1.capacity() >= 8);
}

#[no_mangle]
fn test_from_buf() {
    let buf: [u32; 8] = [sea::nd_u32(); 8];

    let v: SmallVec<[u32; 8]> = SmallVec::from_buf(buf);

    sea::sassert!(v.len() == 8);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_from_buf_and_len() {
    let buf: [u32; 8] = [sea::nd_u32(); 8];

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    let v: SmallVec<[u32; 8]> = SmallVec::from_buf_and_len(buf, len);

    sea::sassert!(v.len() == len);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_from_buf_and_len_unchecked() {
    let buf: [u32; 8] = [sea::nd_u32(); 8];

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    let v: SmallVec<[u32; 8]> =
        unsafe { SmallVec::from_buf_and_len_unchecked(MaybeUninit::new(buf), len) };

    sea::sassert!(v.len() == len);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_from_const() {
    let v: SmallVec<[u32; 8]> = SmallVec::from_const([sea::nd_u32(); 8]);

    sea::sassert!(v.len() == 8);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_from_elem() {
    let elem: u32 = sea::nd_u32();

    let v: SmallVec<[u32; 8]> = SmallVec::from_elem(elem, 8);

    sea::sassert!(v.len() == 8);
    sea::sassert!(v.capacity() == 8);

    for i in 0..8 {
        sea::sassert!(v[i] == elem);
    }
}

#[no_mangle]
fn test_from_raw_parts() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let ptr: *mut u32 = v.as_mut_ptr();

    unsafe {
        mem::forget(v);

        // Capacity has to be greater than original capacity for this to work.
        let v2: SmallVec<[u32; 8]> = SmallVec::from_raw_parts(ptr, len, 16);

        sea::sassert!(v2.len() == len);
        sea::sassert!(v2.capacity() == 16);
    }
}

#[no_mangle]
fn test_from_slice() {
    let mut buf: [u32; 8] = [0; 8];

    for i in 0..8 {
        buf[i] = sea::nd_u32();
    }

    let v: SmallVec<[u32; 8]> = SmallVec::from_slice(&buf);

    sea::sassert!(v.len() == 8);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_grow() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let new_cap: usize = sea::nd_usize();
    sea::assume(new_cap >= 8 && new_cap <= 16);

    v.grow(new_cap);

    sea::sassert!(v.len() == len);
    sea::sassert!(v.capacity() == new_cap);
}

#[no_mangle]
fn test_insert() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len > 0 && len < 6);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let insert_point: usize = sea::nd_usize();
    sea::assume(insert_point < len);
    v.insert(insert_point, sea::nd_u32());

    sea::sassert!(v.len() == len + 1);
    sea::sassert!(v.capacity() == 8);

    let insert_point2: usize = sea::nd_usize();
    sea::assume(insert_point2 > len + 1);

    // Index is out of bounds so this should panic.
    v.insert(insert_point2, sea::nd_u32());

    // Previous insertion should panic so this shouldn't be reachable.
    sea::sassert!(false);
}

#[no_mangle]
fn test_insert_from_slice() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();
    let mut v2: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let len2: usize = sea::nd_usize();
    sea::assume(len + len2 <= 8);

    for _i in 0..len2 {
        v2.push(sea::nd_u32());
    }

    let insert_point: usize = sea::nd_usize();
    sea::assume(insert_point < len);

    v.insert_from_slice(insert_point, &v2);

    sea::sassert!(v.len() == len + len2);
    sea::sassert!(v2.len() == len2);
    sea::sassert!(v.capacity() == 8);
    sea::sassert!(v2.capacity() == 8);

    let insert_point2: usize = sea::nd_usize();
    sea::assume(insert_point2 > len + len2);

    // Index is out of bounds so this should panic.
    v.insert_from_slice(insert_point2, &v2);

    // This assertion should not be reachable since the previous operation should panic.
    sea::sassert!(false);
}

#[no_mangle]
fn test_new() {
    let v: SmallVec<[u32; 8]> = SmallVec::new();

    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_new_const() {
    let v: SmallVec<[u32; 8]> = SmallVec::new_const();

    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_pop() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len > 0 && len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    for i in 0..len {
        v.pop();
        sea::sassert!(v.len() == len - i - 1);
    }

    let result: Option<u32> = v.pop();
    sea::sassert!(result.is_none());
}

#[no_mangle]
fn test_reserve() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let new_cap: usize = sea::nd_usize();
    sea::assume(new_cap >= 8);

    v.reserve(new_cap);

    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() >= new_cap);
}

#[no_mangle]
fn test_reserve_exact() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let new_cap: usize = sea::nd_usize();
    sea::assume(new_cap >= 8 && new_cap <= 16);

    v.reserve_exact(new_cap);

    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == new_cap);
}

#[no_mangle]
fn test_set_len() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    unsafe {
        v.set_len(len);
    }

    sea::sassert!(v.len() == len);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_truncate() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let truncate_point: usize = sea::nd_usize();
    sea::assume(truncate_point <= len);

    v.truncate(truncate_point);

    sea::sassert!(v.len() == truncate_point);
    sea::sassert!(v.capacity() == 8);

    let truncate_point2: usize = sea::nd_usize();
    sea::assume(truncate_point2 > truncate_point);

    v.truncate(truncate_point2);

    sea::sassert!(v.len() == truncate_point);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_try_reserve() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let new_cap: usize = sea::nd_usize();
    sea::assume(new_cap >= 8 && new_cap <= u16::MAX as usize);

    let result: Result<(), smallvec::CollectionAllocErr> = v.try_reserve(new_cap);

    sea::sassert!(result.is_ok());
    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() >= new_cap);
}

#[no_mangle]
fn test_try_reserve_exact() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let new_cap: usize = sea::nd_usize();
    sea::assume(new_cap >= 8 && new_cap <= u16::MAX as usize);

    let result: Result<(), smallvec::CollectionAllocErr> = v.try_reserve_exact(new_cap);

    sea::sassert!(result.is_ok());
    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == new_cap);
}

#[no_mangle]
fn test_with_capacity() {
    let cap: usize = sea::nd_usize();
    sea::assume(cap >= 8);

    let v: SmallVec<[u32; 8]> = SmallVec::with_capacity(cap);

    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == cap);
}
