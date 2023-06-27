/*
Tests that have issues with memory allocation:
- test_append
- test_drain
- test_extend_from_slice
- test_from_buf_and_len_unchecked
- test_from_raw_parts
- test_insert
- test_insert_from_slice
- test_insert_many
- test_remove
- test_reserve
- test_resize
- test_resize_with
- test_shrink_to_fit
- test_swap_remove
- test_try_reserve
*/

#![no_std]

use sea;

use core::mem::{self, MaybeUninit};
use smallvec::SmallVec;

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: u8 = sea::nd_u8();
    match v {
        0 => test_append(),
        1 => test_clear(),
        2 => test_dedup(),
        3 => test_dedup_by(),
        4 => test_dedup_by_key(),
        5 => test_drain(),
        6 => test_extend_from_slice(),
        7 => test_from_buf(),
        8 => test_from_buf_and_len(),
        9 => test_from_buf_and_len_unchecked(),
        10 => test_from_const(),
        11 => test_from_elem(),
        12 => test_from_raw_parts(),
        13 => test_from_slice(),
        14 => test_grow(),
        15 => test_insert(),
        16 => test_insert_from_slice(),
        17 => test_insert_many(),
        18 => test_new(),
        19 => test_new_const(),
        20 => test_pop(),
        21 => test_push(),
        22 => test_remove(),
        23 => test_reserve(),
        24 => test_reserve_exact(),
        25 => test_resize(),
        26 => test_resize_with(),
        27 => test_retain(),
        28 => test_retain_mut(),
        29 => test_set_len(),
        30 => test_shrink_to_fit(),
        31 => test_swap_remove(),
        32 => test_truncate(),
        33 => test_try_grow(),
        34 => test_try_reserve(),
        35 => test_try_reserve_exact(),
        36 => test_with_capacity(),
        _ => (),
    }
}

#[no_mangle]
fn test_append() {
    const CAP: usize = 1;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let mut v2: SmallVec<[u32; CAP]> = SmallVec::new();

    let len2: usize = sea::nd_usize();
    sea::assume(len2 <= CAP);

    for _i in 0..len2 {
        v2.push(sea::nd_u32());
    }

    v.append(&mut v2);

    sea::sassert!(v.len() == len + len2);
    sea::sassert!(v.capacity() == CAP);
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
fn test_dedup() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

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
fn test_drain() {
    let mut v1: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len >= 2 && len <= 8);

    for _i in 0..len {
        v1.push(sea::nd_u32());
    }

    let drain_point: usize = sea::nd_usize();
    sea::assume(drain_point >= 1 && drain_point < len);
    let mut v2: SmallVec<[u32; 8]> = v1.drain(drain_point..).collect();

    sea::sassert!(v1.len() == drain_point);
    sea::sassert!(v2.len() == len - drain_point);

    let v3: SmallVec<[u32; 8]> = v1.drain(drain_point..).collect();

    sea::sassert!(v1.len() == drain_point);
    sea::sassert!(v3.len() == 0);

    let drain_point2: usize = sea::nd_usize();
    sea::assume(drain_point2 < len - drain_point);
    let v4: SmallVec<[u32; 8]> = v2.drain(drain_point2..len - drain_point).collect();

    sea::sassert!(v2.len() == drain_point2);
    sea::sassert!(v4.len() == len - drain_point - drain_point2);

    if sea::nd_bool() {
        let drain_point3: usize = sea::nd_usize();
        sea::assume(drain_point3 > drain_point);

        // End is greater than length, so this should panic.
        let _: SmallVec<[u32; 8]> = v1.drain(drain_point3..).collect();
    } else {
        let drain_point3: usize = sea::nd_usize();
        let drain_point4: usize = sea::nd_usize();
        sea::assume(drain_point3 < drain_point);
        sea::assume(drain_point4 < drain_point);
        sea::assume(drain_point4 > drain_point3);

        // Start is greater than end, so this should panic.
        let _: SmallVec<[u32; 8]> = v1.drain(drain_point4..drain_point3).collect();
    }

    // This assertion should not be reachable since the previous call to drain should panic.
    sea::sassert!(false);
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
    sea::sassert!(v1.capacity() == 8);
}

#[no_mangle]
fn test_from_buf() {
    let mut buf: [u32; 8] = [0; 8];

    for i in 0..8 {
        buf[i] = sea::nd_u32();
    }

    let v: SmallVec<[u32; 8]> = SmallVec::from_buf(buf);

    sea::sassert!(v.len() == 8);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_from_buf_and_len() {
    let mut buf: [u32; 8] = [0; 8];

    for i in 0..8 {
        buf[i] = sea::nd_u32();
    }

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    let v: SmallVec<[u32; 8]> = SmallVec::from_buf_and_len(buf, len);

    sea::sassert!(v.len() == len);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_from_buf_and_len_unchecked() {
    let mut buf: [u32; 8] = [0; 8];

    for i in 0..8 {
        buf[i] = sea::nd_u32();
    }

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
    let cap: usize = v.capacity();

    unsafe {
        mem::forget(v);

        // TODO: Seahorn doesn't get past this point.
        let v2: SmallVec<[u32; 8]> = SmallVec::from_raw_parts(ptr, len, cap);

        sea::sassert!(v2.len() == len);
        sea::sassert!(v2.capacity() == cap);
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

    v.insert(insert_point2, sea::nd_u32());

    sea::sassert!(false);
}

#[no_mangle]
fn test_insert_from_slice() {
    // Test the insert from slice function using seahorn.
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

    v.insert_from_slice(insert_point2, &v2);

    // This assertion should not be reachable since the previous operation should panic.
    sea::sassert!(false);
}

#[no_mangle]
fn test_insert_many() {
    // Test the insert from slice function using seahorn.
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

    v.insert_many(insert_point, v2.clone());

    sea::sassert!(v.len() == len + len2);
    sea::sassert!(v2.len() == len2);
    sea::sassert!(v.capacity() == 8);
    sea::sassert!(v2.capacity() == 8);

    let insert_point2: usize = sea::nd_usize();
    sea::assume(insert_point2 > len + len2);

    v.insert_many(insert_point2, v2.clone());

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
fn test_push() {
    const CAP: usize = 1;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for i in 0..len {
        v.push(sea::nd_u32());
        sea::sassert!(v.len() == i + 1);
    }

    sea::sassert!(v.len() == len);
    sea::sassert!(v.capacity() == CAP);

    sea::sea_printf!("len", len);
    if len == CAP {
        v.push(sea::nd_u32());
        // TODO: This point is only reached when CAP = 1.
        sea::sassert!(v.len() == CAP + 1);
        sea::sassert!(v.capacity() > CAP);
    }
}

#[no_mangle]
fn test_remove() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();
    let len: usize = sea::nd_usize();
    sea::assume(2 <= len && len <= 8);

    // TODO: Seahorn doesn't reach past this loop.
    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let remove_point1: usize = sea::nd_usize();
    sea::assume(remove_point1 < len);

    v.remove(remove_point1);

    sea::sassert!(v.len() == len - 1);
    sea::sassert!(v.capacity() == 8);

    let remove_point2: usize = sea::nd_usize();
    sea::assume(remove_point2 < len);

    v.remove(remove_point2);

    sea::sassert!(v.len() == len - 2);
    sea::sassert!(v.capacity() == 8);

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
fn test_resize() {
    // TODO: Seahorn gets stuck on this test.
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    // let resize_point: usize = sea::nd_usize();
    // sea::assume(resize_point <= 8);
    // v.resize(resize_point, sea::nd_u32());

    // sea::sassert!(v.len() == resize_point);

    // let resize_point2: usize = sea::nd_usize();
    // v.resize(resize_point2, sea::nd_u32());

    // sea::sassert!(v.len() == resize_point2);

    let resize_point3: usize = sea::nd_usize();
    sea::assume(resize_point3 == 9);
    v.resize(resize_point3, sea::nd_u32());

    sea::sassert!(v.len() == resize_point3);
}

#[no_mangle]
fn test_resize_with() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let resize_point: usize = sea::nd_usize();
    sea::assume(resize_point <= 8);
    v.resize_with(resize_point, || sea::nd_u32());

    sea::sassert!(v.len() == resize_point);

    let resize_point2: usize = sea::nd_usize();
    v.resize_with(resize_point2, || sea::nd_u32());

    sea::sassert!(v.len() == resize_point2);

    let resize_point3: usize = sea::nd_usize();
    sea::assume(resize_point3 > 8);
    // This is larger than the capacity of the vector and should panic.
    v.resize_with(resize_point3, || sea::nd_u32());

    // This assertion should not be reachable since the previous operation should panic.
    sea::sassert!(false);
}

#[no_mangle]
fn test_retain() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

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
    sea::sassert!(v.capacity() == 8);

    v.retain(|x| (*x & 1) == 1);

    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_retain_mut() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

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
    sea::sassert!(v.capacity() == 8);

    v.retain(|x| (*x & 1) == 1);

    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == 8);
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
fn test_shrink_to_fit() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    v.shrink_to_fit();

    sea::sassert!(v.len() == len);
    sea::sassert!(v.capacity() == 8);

    for _i in len..9 {
        v.push(sea::nd_u32());
    }

    v.pop();

    v.shrink_to_fit();

    sea::sassert!(v.len() == 8);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_swap_remove() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();
    let len: usize = sea::nd_usize();
    sea::assume(2 <= len && len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let remove_point1: usize = sea::nd_usize();
    sea::assume(remove_point1 < len);

    v.swap_remove(remove_point1);

    sea::sassert!(v.len() == len - 1);
    sea::sassert!(v.capacity() == 8);

    let remove_point2: usize = sea::nd_usize();
    sea::assume(remove_point2 < len - 1);

    v.swap_remove(remove_point2);

    sea::sassert!(v.len() == len - 2);
    sea::sassert!(v.capacity() == 8);

    for i in 0..len - 2 {
        v.swap_remove(0);
        sea::sassert!(v.len() == len - 3 - i);
    }

    // v is empty, so this should panic
    v.swap_remove(0);

    // This assertion should not be reachable since the call to remove panics.
    sea::sassert!(false);
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
fn test_try_grow() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let new_cap: usize = sea::nd_usize();
    sea::assume(new_cap >= 8 && new_cap <= 16);

    let result: Result<(), smallvec::CollectionAllocErr> = v.try_grow(new_cap);

    sea::sassert!(result.is_ok());
    sea::sassert!(v.len() == len);
    sea::sassert!(v.capacity() == new_cap);

    let new_cap2: usize = sea::nd_usize();
    sea::assume(new_cap2 < len);

    let result2: Result<(), smallvec::CollectionAllocErr> = v.try_grow(new_cap2);

    sea::sassert!(result2.is_err());
}

#[no_mangle]
fn test_try_reserve() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let new_cap: usize = sea::nd_usize();
    sea::assume(new_cap >= 8);

    let result: Result<(), smallvec::CollectionAllocErr> = v.try_reserve(new_cap);

    sea::sassert!(result.is_ok());
    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() >= new_cap);
}

#[no_mangle]
fn test_try_reserve_exact() {
    let mut v: SmallVec<[u32; 8]> = SmallVec::new();

    let new_cap: usize = sea::nd_usize();
    sea::assume(new_cap >= 8 && new_cap <= 16);

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
