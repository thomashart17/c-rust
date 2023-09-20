use borsh::{BorshSerialize, BorshDeserialize};

use sea;

#[no_mangle]
pub extern "C" fn entrypt() {
    // this takes around 90 seconds to run
    test_primitives();
}

#[no_mangle]
fn test_primitives() {
    let x: () = ();
    let encoded: Vec<u8> = x.try_to_vec().unwrap();
    let decoded: () = <()>::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
    let x: bool = sea::nd_bool();
    let encoded: Vec<u8> = x.try_to_vec().unwrap();
    let decoded: bool = bool::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
    let x: u8 = sea::nd_u8();
    let encoded: Vec<u8> = x.try_to_vec().unwrap();
    let decoded: u8 = u8::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
    let x: i8 = sea::nd_i8();
    let encoded = x.try_to_vec().unwrap();
    let decoded = i8::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
    let x: u16 = sea::nd_u16();
    let encoded = x.try_to_vec().unwrap();
    let decoded = u16::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
    let x: i16 = sea::nd_i16();
    let encoded = x.try_to_vec().unwrap();
    let decoded = i16::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
    let x: u32 = sea::nd_u32();
    let encoded = x.try_to_vec().unwrap();
    let decoded = u32::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
    let x: i32 = sea::nd_i32();
    let encoded = x.try_to_vec().unwrap();
    let decoded = i32::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
    let x: u64 = sea::nd_u64();
    let encoded = x.try_to_vec().unwrap();
    let decoded = u64::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
    let x: i64 = sea::nd_i64();
    let encoded = x.try_to_vec().unwrap();
    let decoded = i64::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
    let x: usize = sea::nd_usize();
    let encoded = x.try_to_vec().unwrap();
    let decoded = usize::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
    let x: isize = sea::nd_isize();
    let encoded = x.try_to_vec().unwrap();
    let decoded = isize::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
    let x: usize = sea::nd_uintptr();
    let encoded = x.try_to_vec().unwrap();
    let decoded = usize::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
    let x: isize = sea::nd_intptr();
    let encoded = x.try_to_vec().unwrap();
    let decoded = isize::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
}
