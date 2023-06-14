#![no_std]

use sea;

use tinyvec::ArrayVec;

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: u8 = sea::nd_u8();
    sea::assume(v < 11);
    match v {
        0 => test_append(),
        1 => test_clear(),
        2 => test_fill(),
        3 => test_insert(),
        4 => test_new(),
        5 => test_pop(),
        6 => test_push(),
        7 => test_remove(),
        8 => test_set_len(),
        9 => test_split_off(),
        10 => test_truncate(),
        _ => ()
    }
}

#[no_mangle]
fn test_append() {
    let mut v1: ArrayVec<[u32; 6]> = ArrayVec::new();
    let mut v2: ArrayVec<[u32; 6]> = ArrayVec::new();

    v1.push(1);
    v1.push(2);
    v1.push(3);

    v2.push(4);
    v2.push(5);
    v2.push(6);

    v1.append(&mut v2);

    sea::sassert!(v1.len() == 6);
    sea::sassert!(v2.len() == 0);
}

#[no_mangle]
fn test_clear() {
    let mut v: ArrayVec<[u32; 10]> = ArrayVec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    v.clear();

    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == 10);

    // Necessary to make seahorn work.
    let x: u32 = sea::nd_u32();
    let result: u32 = x * 2;
    sea::sassert!(result >= x);
}

#[no_mangle]
fn test_fill() {
    let val: u32 = sea::nd_u32();
    sea::assume(val <= 10);
    let mut v: ArrayVec<[u32; 10]> = ArrayVec::new();

    v.fill(0..val);

    sea::sassert!(v.len() == val as usize);
    sea::sassert!(v.capacity() == 10);

    for n in 0..val {
        sea::sassert!(n == v[n as usize]);
    }
}

#[no_mangle]
fn test_insert() {
    let mut v: ArrayVec<[u32; 5]> = ArrayVec::new();
    v.push(1);
    v.push(3);
    
    v.insert(1, 2);

    sea::sassert!(v.len() == 3);
    sea::sassert!(v.capacity() == 5);

    if sea::nd_bool() {
        // Index is greater than length, so insertion should panic.
        v.insert(4, 4);
    } else {
        v.push(4);
        v.push(5);

        // Vector is at capacity, so insertion should panic.
        v.insert(1, 1);
    }

    // This assertion should not be reachable as the previous insertion should panic.
    sea::sassert!(false);
}

#[no_mangle]
fn test_new() {
    let v: ArrayVec<[u32; 0]> = ArrayVec::new();
    
    sea::sassert!(v.len() == 0);
    sea::sassert!(v.capacity() == 0);

    // Necessary to make seahorn work.
    let x: u32 = sea::nd_u32();
    let result: u32 = x * 2;
    sea::sassert!(result >= x);
}

#[no_mangle]
fn test_pop() {
    let val: u32 = sea::nd_u32();
    let mut v: ArrayVec<[u32; 1]> = ArrayVec::new();

    v.push(val);
    let result: Option<u32> = v.pop();

    sea::sassert!(v.len() == 0);
    sea::sassert!(result.unwrap() == val);

    let result: Option<u32> = v.pop();

    sea::sassert!(result.is_none());

    // Necessary to make seahorn work.
    let x: u32 = sea::nd_u32();
    let result: u32 = x * 2;
    sea::sassert!(result >= x);
}

#[no_mangle]
fn test_push() {
    let val: u32 = sea::nd_u32();
    let mut v: ArrayVec<[u32; 1]> = ArrayVec::new();

    v.push(val);
    sea::sassert!(v.len() == 1);

    // Necessary to make seahorn work.
    let x: u32 = sea::nd_u32();
    let result: u32 = x * 2;
    sea::sassert!(result >= x);
    // *******************************

    v.push(val);

    // This assertion should not be reachable since the previous push panics.
    sea::sassert!(false);
}

#[no_mangle]
fn test_remove() {
    let mut v: ArrayVec<[u32; 2]> = ArrayVec::new();
    v.push(1);
    v.push(2);

    v.remove(1);

    sea::sassert!(v.len() == 1);
    sea::sassert!(v.capacity() == 2);

    // Index is out of range, so removal should panic.
    v.remove(1);

    // This assertion should not be reachable since the call to remove panics.
    sea::sassert!(false);
}

#[no_mangle]
fn test_set_len() {
    let val: usize = sea::nd_usize();
    sea::assume(val <= 10);
    
    let mut v: ArrayVec<[u32; 10]> = ArrayVec::new();

    v.set_len(val);

    sea::sassert!(v.len() == val);
    sea::sassert!(v.capacity() == 10);

    v.set_len(20);

    // This assertion should not be reachable since the previous operation panics.
    sea::sassert!(false);
}

#[no_mangle]
fn test_split_off() {
    let mut v: ArrayVec<[u32; 5]> = ArrayVec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let v2: ArrayVec<[u32; 5]> = v.split_off(2);

    sea::sassert!(v.len() == 2);
    sea::sassert!(v2.len() == 3);

    // Index is out of range, so this should panic.
    let _: ArrayVec<[u32; 5]> = v.split_off(3);

    // This assertion should not be reachable since the previous opration should panic.
    sea::sassert!(false);
}

#[no_mangle]
fn test_truncate() {
    let val: usize = sea::nd_usize();
    sea::assume(val <= 5);

    let mut v: ArrayVec<[u32; 5]> = ArrayVec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    v.truncate(val);

    sea::sassert!(v.len() == val);
    sea::sassert!(v.capacity() == 5);

    v.truncate(10);

    sea::sassert!(v.len() == val);
    sea::sassert!(v.capacity() == 5);
}
