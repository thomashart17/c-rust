#![no_std]

use sea;

use smallvec::SmallVec;

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: u8 = sea::nd_u8();
    match v {
        0 => test_append(),
        1 => test_drain(),
        2 => test_drain_panic(),
        3 => test_insert_many(),
        4 => test_insert_many_panic(),
        5 => test_resize(),
        6 => test_resize2(),
        7 => test_resize_with(),
        8 => test_resize_with2(),
        9 => test_shrink_to_fit(),
        _ => ()
    }
}

#[no_mangle]
fn test_append() {
    const CAP: usize = 2;
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
    sea::sassert!(v.capacity() >= CAP);
}

#[no_mangle]
fn test_drain() {
    const CAP: usize = 2;
    let mut v1: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for _i in 0..len {
        v1.push(sea::nd_u32());
    }

    let drain_point: usize = sea::nd_usize();
    sea::assume(drain_point < len);
    let mut v2: SmallVec<[u32; CAP]> = v1.drain(drain_point..).collect();

    sea::sassert!(v1.len() == drain_point);
    sea::sassert!(v2.len() == len - drain_point);

    let v3: SmallVec<[u32; CAP]> = v1.drain(drain_point..).collect();

    sea::sassert!(v1.len() == drain_point);
    sea::sassert!(v3.len() == 0);

    let drain_point2: usize = sea::nd_usize();
    sea::assume(drain_point2 < len - drain_point);
    let v4: SmallVec<[u32; CAP]> = v2.drain(drain_point2..len - drain_point).collect();

    sea::sassert!(v2.len() == drain_point2);
    sea::sassert!(v4.len() == len - drain_point - drain_point2);
}

#[no_mangle]
fn test_drain_panic() {
    const CAP: usize = 2;
    let mut v1: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for _i in 0..len {
        v1.push(sea::nd_u32());
    }

    if sea::nd_bool() {
        let drain_point: usize = sea::nd_usize();
        sea::assume(drain_point > len);

        // End is greater than length, so this should panic.
        let _: SmallVec<[u32; CAP]> = v1.drain(drain_point..).collect();
    } else {
        let drain_point: usize = sea::nd_usize();
        let drain_point2: usize = sea::nd_usize();
        sea::assume(drain_point < len);
        sea::assume(drain_point2 < len);
        sea::assume(drain_point2 > drain_point);

        // Start is greater than end, so this should panic.
        let _: SmallVec<[u32; CAP]> = v1.drain(drain_point2..drain_point).collect();
    }

    // This assertion should not be reachable since the previous call to drain should panic.
    sea::sassert!(false);
}

#[no_mangle]
fn test_insert_many() {
    const CAP: usize = 2;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();
    let mut v2: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let len2: usize = sea::nd_usize();
    sea::assume(len + len2 <= CAP);

    for _i in 0..len2 {
        v2.push(sea::nd_u32());
    }

    let insert_point: usize = sea::nd_usize();
    sea::assume(insert_point < len);

    v.insert_many(insert_point, v2.clone());

    sea::sassert!(v.len() == len + len2);
    sea::sassert!(v2.len() == len2);
    sea::sassert!(v.capacity() == CAP);
    sea::sassert!(v2.capacity() == CAP);
}

#[no_mangle]
fn test_insert_many_panic() {
    const CAP: usize = 2;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();
    let mut v2: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let len2: usize = sea::nd_usize();
    sea::assume(len + len2 <= CAP);

    for _i in 0..len2 {
        v2.push(sea::nd_u32());
    }

    let insert_point: usize = sea::nd_usize();
    sea::assume(insert_point > len + len2);
    
    // Index is out of bounds so this should panic.
    v.insert_many(insert_point, v2.clone());

    // This assertion should not be reachable since the previous operation should panic.
    sea::sassert!(false);
}

#[no_mangle]
fn test_resize() {
    const CAP: usize = 2;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let resize_point: usize = sea::nd_usize();
    sea::assume(resize_point <= CAP);
    v.resize(resize_point, sea::nd_u32());

    sea::sassert!(v.len() == resize_point);
}

#[no_mangle]
fn test_resize2() {
    const CAP: usize = 2;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let resize_point: usize = sea::nd_usize();
    sea::assume(resize_point > CAP && resize_point <= 2*CAP);

    v.resize(resize_point, sea::nd_u32());

    sea::sassert!(v.len() == resize_point);
}

#[no_mangle]
fn test_resize_with() {
    const CAP: usize = 2;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let resize_point: usize = sea::nd_usize();
    sea::assume(resize_point <= CAP);
    v.resize_with(resize_point, || sea::nd_u32());

    sea::sassert!(v.len() == resize_point);
}

#[no_mangle]
fn test_resize_with2() {
    const CAP: usize = 2;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let resize_point: usize = sea::nd_usize();
    sea::assume(resize_point > CAP && resize_point <= 2*CAP);

    v.resize_with(resize_point, || sea::nd_u32());

    sea::sassert!(v.len() == resize_point);
}

#[no_mangle]
fn test_shrink_to_fit() {
    const CAP: usize = 2;
    let mut v: SmallVec<[u32; CAP]> = SmallVec::new();

    let len: usize = sea::nd_usize();
    sea::assume(len <= CAP);
 
    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    v.shrink_to_fit();

    sea::sassert!(v.len() == len);
    sea::sassert!(v.capacity() == CAP);

    for _i in len..CAP + 1 {
        v.push(sea::nd_u32());
    }

    sea::sassert!(v.len() == CAP + 1);
    sea::sassert!(v.capacity() > CAP);

    v.pop();

    v.shrink_to_fit();

    sea::sassert!(v.len() == CAP);
    sea::sassert!(v.capacity() == CAP);
}
