#![no_std]

use sea;

use tinyvec::ArrayVec;


// Testing the error fixed here: https://github.com/Lokathor/tinyvec/pull/178
#[no_mangle]
pub extern "C" fn entrypt() {
    let vec: ArrayVec<[u32; u16::MAX as usize + 1]> = ArrayVec::new();

    // When using previous versions of the library where the error is still present,
    // this assertion should fail. For newer versions, it should pass.
    sea::sassert!(vec.capacity() == u16::MAX as usize);
}
