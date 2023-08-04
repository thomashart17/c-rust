#![no_std]

use sea;

use tinyvec::ArrayVec;

// Testing the error fixed here: https://github.com/Lokathor/tinyvec/pull/29
#[no_mangle]
pub extern "C" fn entrypt() {
    let mut v: ArrayVec<[u32; 8]> = ArrayVec::new();
    let len: usize = sea::nd_usize();
    sea::assume(len <= 8);

    for _i in 0..len {
        v.push(sea::nd_u32());
    }

    let remove_point: usize = sea::nd_usize();
    sea::assume(remove_point <= 8);

    v.remove(remove_point);

    // When using previous versions of the library where the error is still present,
    // the removal doesn't panic if the index is out of bounds and the "sea::sassert!f(false)" 
    // will be reached. For newer versions, it will panic and the assertion won't be reached.
    if remove_point < len {
        sea::sassert!(v.len() == len - 1);
    } else {
        // If remove_point is out of bounds, then the call to remove should panic and this assertion should not be reachable.
        sea::sassert!(false);
    }
}
