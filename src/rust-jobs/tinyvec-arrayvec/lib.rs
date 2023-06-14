#![no_std]

use sea;

use tinyvec::ArrayVec;

#[no_mangle]
pub extern "C" fn entrypt() {
    test_append();
    test_clear();
    test_fill();
    test_new();
    test_pop();
    test_push();
    test_set_len();
    test_truncate();
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
