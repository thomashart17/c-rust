// #![no_std]

// use sea;

// extern crate alloc;
// use alloc::vec::Vec;
// use alloc::vec;


#[no_mangle]
pub extern "C" fn entrypt() {
    let mut reader: &[u8] = b"hello";
    let mut writer: Vec<u8> = vec![];
    std::io::copy(&mut reader, &mut writer);
}


// #[no_mangle] extern "C" fn __rust_probestack () {}
