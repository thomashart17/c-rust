use borsh::{BorshSerialize, BorshDeserialize};

use sea;


#[no_mangle]
pub extern "C" fn entrypt() {
    test_enums();
}

#[no_mangle]
fn test_enums() {
    #[derive(BorshSerialize, BorshDeserialize, PartialEq)]
    enum NdType {
        U32(u32),
        I32(i32),
    }
    let x: NdType = if sea::nd_bool() {
        NdType::U32(sea::nd_u32())
    } else {
        NdType::I32(sea::nd_i32())
    };
    let encoded: Vec<u8> = x.try_to_vec().unwrap();
    let decoded: NdType = NdType::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
}
