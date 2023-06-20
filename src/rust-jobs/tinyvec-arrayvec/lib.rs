#![no_std]

use sea;

use tinyvec::ArrayVec;

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: u8 = sea::nd_u8();
    match v {
        0 => test_append(),
        1 => test_clear(),
        2 => test_drain(),
        3 => test_extend_from_slice(),
        4 => test_fill(),
        5 => test_from_array_empty(),
        6 => test_from_array_len(),
        // 7  => test_grab_spare_slice(),
        8 => test_insert(),
        9 => test_new(),
        10 => test_pop(),
        11 => test_push(),
        12 => test_remove(),
        13 => test_resize(),
        14 => test_resize_with(),
        15 => test_retain(),
        16 => test_set_len(),
        // 17 => test_splice(),
        18 => test_split_off(),
        19 => test_swap_remove(),
        20 => test_truncate(),
        21 => test_try_append(),
        22 => test_try_from_array_len(),
        23 => test_try_insert(),
        24 => test_try_push(),
        _ => (),
    }
}

#[no_mangle]
fn test_append() {
    let mut v1: ArrayVec<[u32; 8]> = ArrayVec::new();
    let mut v2: ArrayVec<[u32; 8]> = ArrayVec::new();

    let len1: usize = sea::nd_usize();
    sea::assume(len1 <= 8);

    let len2: usize = sea::nd_usize();
    sea::assume(len2 <= 8);
    
    for _i in 0..len1 {
        v1.push(sea::nd_u32());
    }
    
    sea::sassert!(v1.len() == len1);

    for _i in 0..len2 {
        v2.push(sea::nd_u32());
    }

    sea::sassert!(v2.len() == len2);

    // Panics if the capacity is exceeded.
    v1.append(&mut v2);

    if len1 + len2 <= 8 {
        sea::sassert!(v1.len() == len1 + len2);
        sea::sassert!(v2.len() == 0);
    } else {
        // This assertion should not be reachable since the previous operation should panic.
        sea::sassert!(false);
    }
}

#[no_mangle]
fn test_clear() {
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();

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
fn test_drain() {
    let mut v1: ArrayVec<[u32; 8]> = ArrayVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len >= 2 && len <= 8);

    for _i in 0..len {
        v1.push(sea::nd_u32());
    }

    let drain_point: usize = sea::nd_usize();
    sea::assume(drain_point >= 1 && drain_point < len);
    let mut v2: ArrayVec<[u32; 8]> = v1.drain(drain_point..).collect();

    sea::sassert!(v1.len() == drain_point);
    sea::sassert!(v2.len() == len - drain_point);

    let v3: ArrayVec<[u32; 8]> = v1.drain(drain_point..).collect();

    sea::sassert!(v1.len() == drain_point);
    sea::sassert!(v3.len() == 0);

    let drain_point2: usize = sea::nd_usize();
    sea::assume(drain_point2 < len - drain_point);
    let v4: ArrayVec<[u32; 8]> = v2.drain(drain_point2..len - drain_point).collect();

    sea::sassert!(v2.len() == drain_point2);
    sea::sassert!(v4.len() == len - drain_point - drain_point2);

    if sea::nd_bool() {
        let drain_point3: usize = sea::nd_usize();
        sea::assume(drain_point3 > drain_point);

        // End is greater than length, so this should panic.
        let _: ArrayVec<[u32; 8]> = v1.drain(drain_point3..).collect();
    } else {
        let drain_point3: usize = sea::nd_usize();
        let drain_point4: usize = sea::nd_usize();
        sea::assume(drain_point3 < drain_point);
        sea::assume(drain_point4 < drain_point);
        sea::assume(drain_point4 > drain_point3);

        // Start is greater than end, so this should panic.
        let _: ArrayVec<[u32; 4]> = v1.drain(drain_point4..drain_point3).collect();
    }

    // This assertion should not be reachable since the previous call to drain should panic.
    sea::sassert!(false);
}

#[no_mangle]
fn test_extend_from_slice() {
    let mut v1: ArrayVec<[u32; 8]> = ArrayVec::new();
    let mut v2: ArrayVec<[u32; 8]> = ArrayVec::new();

    let len1: usize = sea::nd_usize();
    let len2: usize = sea::nd_usize();

    sea::assume(len1 <= 8);
    sea::assume(len2 <= 8);

    for _i in 0..len1 {
        v1.push(sea::nd_u32());
    }

    for _i in 0..len2 {
        v2.push(sea::nd_u32());
    }

    // This should panic since len1 + len2 > 8.
    v1.extend_from_slice(v2.as_slice());

    if len1 + len2 <= 8 {
        sea::sassert!(v1.len() == len1 + len2);
        sea::sassert!(v2.len() == len2);
    } else {
        // This assertion should not be reachable since the previous operation should panic.
        sea::sassert!(false);
    }
}

#[no_mangle]
fn test_fill() {
    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();

    v.fill(0..len as u32);

    sea::sassert!(v.len() == len);
    sea::sassert!(v.capacity() == 8);

    for n in 0..len {
        sea::sassert!(n as u32 == v[n]);
    }
}

#[no_mangle]
fn test_from_array_empty() {
    let v: ArrayVec<[u32; 8]> = ArrayVec::from_array_empty([0; 8]);

    sea::sassert!(v.len() == 0);

    // Necessary to make seahorn work.
    let x: u32 = sea::nd_u32();
    let result: u32 = x * 2;
    sea::sassert!(result >= x);
}

#[no_mangle]
fn test_from_array_len() {
    let len: usize = sea::nd_usize();

    if len <= 8 {
        let v: ArrayVec<[u32; 8]> = ArrayVec::from_array_len([0; 8], len);

        sea::sassert!(v.len() == len);
    } else {
        // Specified length is larger than capacity of array, so this should panic.
        let _: ArrayVec<[u32; 8]> = ArrayVec::from_array_len([0; 8], len);

        // This assertion should be unreachable since the previous operation panics.
        sea::sassert!(false);
    }

    // Necessary to make seahorn work.
    let x: u32 = sea::nd_u32();
    let result: u32 = x * 2;
    sea::sassert!(result >= x);
}

// Documentation lists this as a function, but the compiler says it doesn't exist.
// https://docs.rs/tinyvec/latest/tinyvec/struct.ArrayVec.html#method.grab_spare_slice
/*
#[no_mangle]
fn test_grab_spare_slice() {
    let mut v: ArrayVec<[u32; 4]> = ArrayVec::new();

    let slice = v.grab_spare_slice();

    sea::sassert!(slice.len() == 4);

    v.push(1);
    v.push(2);

    let slice = v.grab_spare_slice();

    sea::sassert!(slice.len() == 2);

    v.push(3);
    v.push(4);

    let slice = v.grab_spare_slice();

    sea::sassert!(slice.len() == 0);
}
*/

#[no_mangle]
fn test_insert() {
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len > 0 && len <= 7);
    
    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let insert_point: usize = sea::nd_usize();
    sea::assume(insert_point < len);
    v.insert(insert_point, sea::nd_u32());

    sea::sassert!(v.len() == len + 1);
    sea::sassert!(v.capacity() == 8);

    if len < 6 {
        let insert_point2: usize = sea::nd_usize();
        sea::assume(insert_point2 > len + 1);

        // Index is greater than length, so insertion should panic.
        v.insert(insert_point2, sea::nd_u32());
    } else {
        if len == 6 { v.push(sea::nd_u32()); }
        let insert_point2: usize = sea::nd_usize();
        sea::assume(insert_point2 <= 7);

        // Vector is at capacity, so insertion should panic.
        v.insert(insert_point2, sea::nd_u32());
    }

    // This assertion should not be reachable as the previous insertion should panic.
    sea::sassert!(false);
}

#[no_mangle]
fn test_new() {
    let v: ArrayVec<[u32; 8]> = ArrayVec::new();

    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == 8);

    // Necessary to make seahorn work.
    let x: u32 = sea::nd_u32();
    let result: u32 = x * 2;
    sea::sassert!(result >= x);
}

#[no_mangle]
fn test_pop() {
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len > 0 && len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    for i in 0..len {
        let result: Option<u32> = v.pop();
        sea::sassert!(result.is_some());
        sea::sassert!(v.len() == len - i - 1);
    }

    let result: Option<u32> = v.pop();

    sea::sassert!(result.is_none());
}

#[no_mangle]
fn test_push() {
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();
    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for i in 0..len {
        v.push(sea::nd_u32());
        sea::sassert!(v.len() == i + 1);
    }

    sea::sassert!(v.len() == len);
    sea::sassert!(v.capacity() == 8);

    if len == 8 {
        // Vector is at capacity, so push should panic.
        v.push(sea::nd_u32());

        // This assertion should not be reachable since the previous push panics.
        sea::sassert!(false);
    }
}

#[no_mangle]
fn test_remove() {
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();
    let len: usize = sea::nd_usize();
    sea::assume(2 <= len && len <= 8);

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
fn test_resize() {
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let resize_point: usize = sea::nd_usize();
    sea::assume(resize_point <= 8);
    v.resize(resize_point, sea::nd_u32());

    sea::sassert!(v.len() == resize_point);

    let resize_point2: usize = sea::nd_usize();
    v.resize(resize_point2, sea::nd_u32());

    sea::sassert!(v.len() == resize_point2);

    let resize_point3: usize = sea::nd_usize();
    sea::assume(resize_point3 > 8);
    // This is larger than the capacity of the vector and should panic.
    v.resize(resize_point3, sea::nd_u32());

    // This assertion should not be reachable since the previous operation should panic.
    sea::sassert!(false);
}

#[no_mangle]
fn test_resize_with() {
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();

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
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();

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

    v.retain(|&x| (x & 1) == 0);

    sea::sassert!(v.len() == len / 2);
    sea::sassert!(v.capacity() == 8);

    v.retain(|&x| (x & 1) == 1);

    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == 8);
}

#[no_mangle]
fn test_set_len() {
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();

    let len: usize = sea::nd_usize();

    v.set_len(len);

    if len > 8 {
        // This assertion should not be reachable since the previous operation panics.
        sea::sassert!(false);
    } else {
        sea::sassert!(v.len() == len);
        sea::sassert!(v.capacity() == 8);
    }
}

// #[no_mangle]
// fn test_splice() {
//     let mut v1: ArrayVec<[u32; 4]> = ArrayVec::new();

//     v1.push(1);
//     v1.push(2);
//     v1.push(3);
//     v1.push(4);

//     let v2: ArrayVec<[u32; 4]> = v1.splice(1..3, 5..=6).collect();

//     sea::sassert!(v1.len() == 4);
//     sea::sassert!(v2.len() == 2);
//     sea::sassert!(v1[1] == 5);
//     sea::sassert!(v1[2] == 6);
//     sea::sassert!(v2[0] == 2);
//     sea::sassert!(v2[1] == 3);

//     let v2: ArrayVec<[u32; 4]> = v1.splice(1..1, 5..5).collect();

//     sea::sassert!(v1.len() == 4);
//     sea::sassert!(v2.len() == 0);

//     if sea::nd_bool() {
//         // Start is greater than end, so panic should occur.
//         let _: ArrayVec<[u32; 4]> = v1.splice(2..1, 1..2).collect();
//     } else if sea::nd_bool() {
//         // End is past end of vector, so panic should occur.
//         let _: ArrayVec<[u32; 4]> = v1.splice(1..8, 1..2).collect();
//     } else {
//         // New length would overflow the vector, so panic should occur.
//         let _: ArrayVec<[u32; 4]> = v1.splice(1..2, 1..4).collect();
//     }

//     // This assertion should not be reachable since the previous assertion should panic.
//     sea::sassert!(false);
// }

#[no_mangle]
fn test_split_off() {
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let split_point: usize = sea::nd_usize();

    // Panics if split_point > len.
    let v2: ArrayVec<[u32; 8]> = v.split_off(split_point);

    if split_point > len {
        // This assertion should not be reachable since the previous operation panics.
        sea::sassert!(false);
    } else {
        sea::sassert!(v.len() == split_point);
        sea::sassert!(v2.len() == len - split_point);
    }
}

#[no_mangle]
fn test_swap_remove() {
    let mut v: ArrayVec<[u32; 5]> = ArrayVec::new();
    let len: usize = sea::nd_usize();
    sea::assume(2 <= len && len <= 5);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let remove_point1: usize = sea::nd_usize();
    sea::assume(remove_point1 < len);

    v.swap_remove(remove_point1);

    sea::sassert!(v.len() == len - 1);
    sea::sassert!(v.capacity() == 5);

    let remove_point2: usize = sea::nd_usize();
    sea::assume(remove_point2 < len);

    v.swap_remove(remove_point2);

    sea::sassert!(v.len() == len - 2);
    sea::sassert!(v.capacity() == 5);

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
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();

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
fn test_try_append() {
    let mut v1: ArrayVec<[u32; 8]> = ArrayVec::new();
    let mut v2: ArrayVec<[u32; 8]> = ArrayVec::new();

    let len1: usize = sea::nd_usize();
    sea::assume(len1 <= 8);

    let len2: usize = sea::nd_usize();
    sea::assume(len2 <= 8);
    
    for _i in 0..len1 {
        v1.push(sea::nd_u32());
    }
    
    sea::sassert!(v1.len() == len1);

    for _i in 0..len2 {
        v2.push(sea::nd_u32());
    }

    sea::sassert!(v2.len() == len2);

    let result: Option<&mut ArrayVec<[u32; 8]>> = v1.try_append(&mut v2);

    if len1 + len2 <= 8 {
        sea::sassert!(result.is_none());
        sea::sassert!(v1.len() == len1 + len2);
        sea::sassert!(v2.len() == 0);
    } else {
        sea::sassert!(result.is_some());
        sea::sassert!(v1.len() == len1);
        sea::sassert!(v2.len() == len2);
    }
}

#[no_mangle]
fn test_try_from_array_len() {
    let len: usize = sea::nd_usize();

    let v: Result<ArrayVec<[u32; 8]>, _>  = ArrayVec::try_from_array_len([0; 8], len);

    if len <= 8 {
        sea::sassert!(v.is_ok());
        sea::sassert!(v.unwrap().len() == len);
    } else {
        sea::sassert!(v.is_err());
    }

    // Necessary to make seahorn work.
    let x: u32 = sea::nd_u32();
    let result: u32 = x * 2;
    sea::sassert!(result >= x);
}

#[no_mangle]
fn test_try_insert() {
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();
    
    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let insert_point: usize = sea::nd_usize();

    let result: Option<u32> = v.try_insert(insert_point, sea::nd_u32());

    if insert_point > len {
        // This assertion should not be reachable since the previous operation panics.
        sea::sassert!(false);
    } else if len == 8 {
        sea::sassert!(result.is_some());
        sea::sassert!(v.len() == 8);
        sea::sassert!(v.capacity() == 8);
    } else {
        sea::sassert!(result.is_none());
        sea::sassert!(v.len() == len + 1);
        sea::sassert!(v.capacity() == 8)
    }    
}

#[no_mangle]
fn test_try_push() {
    // NOTE: Create a vector of fixed size capacity
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();

    // NOTE: Create a ND number of elements to push
    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    // NOTE: INVARIANT: We should always be able to push len elements since
    // len is <= capacity
    for i in 0..len {
        let result: Option<u32> = v.try_push(sea::nd_u32());
        sea::sassert!(result.is_none());
        sea::sassert!(v.len() == i + 1); // len is 1-based, iterator is 0-based
    }

    // NOTE: INVARIANT: When len == capacity then another push fails
    if v.len() == v.capacity() {
        let result: Option<u32> = v.try_push(sea::nd_u32());
        sea::sassert!(result.is_some());
    }
}
