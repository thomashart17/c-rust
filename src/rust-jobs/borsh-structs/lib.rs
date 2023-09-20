use borsh::{BorshSerialize, BorshDeserialize};

use sea;

#[no_mangle]
pub extern "C" fn entrypt() {
    test_structs();
    test_fields();
}


#[no_mangle]
fn test_structs() {
    #[derive(BorshSerialize, BorshDeserialize, PartialEq)]
    struct TestPair { a: bool, b: () }
    #[derive(BorshSerialize, BorshDeserialize, PartialEq)]
    struct TestStruct { x: u32, y: i64, z: TestPair }

    let t: TestStruct = TestStruct {
        x: sea::nd_u32(),
        y: sea::nd_i64(),
        z: TestPair { 
            a: sea::nd_bool(),
            b: (),
        },
    };
    let encoded: Vec<u8> = t.try_to_vec().unwrap();
    let decoded: TestStruct = TestStruct::try_from_slice(&encoded).unwrap();
    sea::sassert!(t == decoded);
}

#[no_mangle]
fn test_fields() {
    let x: (i64, u8, bool, ()) = {
        (sea::nd_i64(), sea::nd_u8(), sea::nd_bool(), ())
    };
    let encoded: Vec<u8> = x.try_to_vec().unwrap();
    let decoded: (i64, u8, bool, ()) = <(i64, u8, bool, ())>::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);

    #[derive(BorshSerialize, BorshDeserialize, PartialEq)]
    struct TestField(u32, bool, i64);
    let x: TestField = TestField(sea::nd_u32(), sea::nd_bool(), sea::nd_i64());
    let encoded: Vec<u8> = x.try_to_vec().unwrap();
    let decoded: TestField = TestField::try_from_slice(&encoded).unwrap();
    sea::sassert!(x == decoded);
}
