#![no_std]

use sea;

use tinyvec::ArrayVec;

#[no_mangle]
pub extern "C" fn entrypt() {
    let vec: ArrayVec<[u8; u16::MAX as usize + 1]> = ArrayVec::new();

    sea::sassert!(vec.capacity() == u16::MAX as usize);
}
