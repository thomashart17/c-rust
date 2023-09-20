use borsh::{BorshSerialize, BorshDeserialize};

use sea;

#[no_mangle]
pub extern "C" fn entrypt() {
    test_option();
}

#[no_mangle]
fn test_option() {
    let x: Option<u32> = Some(sea::nd_u32());
    let encoded: Vec<u8> = x.try_to_vec().unwrap();
    let decoded: Option<u32> = Option::<u32>::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);

    #[derive(BorshSerialize, BorshDeserialize, PartialEq)]
    struct TestStruct { a: i32, b: u8, }

    let x: Option<TestStruct> = Some(TestStruct { a: sea::nd_i32(), b: sea::nd_u8() });
    let encoded: Vec<u8> = x.try_to_vec().unwrap();
    let decoded: Option<TestStruct> = Option::<TestStruct>::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);

    let x: Option<u8> = None;
    let encoded: Vec<u8> = x.try_to_vec().unwrap();
    let decoded: Option<u8> = Option::<u8>::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
}