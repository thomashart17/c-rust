#![no_std]

use sea;

use tinyvec::ArrayVec;

#[no_mangle]
pub extern "C" fn entrypt() {
    let vec: ArrayVec<[u32; u16::MAX as usize]> = ArrayVec::new();

    sea::sassert!(vec.capacity() == u16::MAX as usize);
}
