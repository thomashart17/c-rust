use borsh::{BorshSerialize, BorshDeserialize};

use sea;
// use std::collections::HashMap;
// // use std::io::{self};

#[derive(BorshSerialize, BorshDeserialize, PartialEq)]
struct A {
    x: u64,
    y: i8,
}

#[no_mangle]
pub extern "C" fn entrypt() {
    // test_primitives();
    // test_structs();
    // test_fields();
    // test_enums();
    // test_option();

    test_string();
    // add to_sting job
    // sea::sassert!(false);

    // test_fixed_sized_arrays();
    // test_hashmap();    
    // test_combinations();

    // llvm script
    // cont borsh
    // example of non-overalaping
    // to_string
    // 

}



// Issue: does not work, issues with bmcp() when string has more than 1 character
#[no_mangle]
fn test_string() {
    let x: String = "a".to_string();
    
    let encoded: Vec<u8> = x.try_to_vec().unwrap();
    let decoded: String = String::try_from_slice(&encoded).unwrap();

    sea::sassert!(x == decoded);
    // sea::sassert!(false);

    // sea::sassert!(x.len() == decoded.len());
    // sea::sassert!(x == "asdf".to_string());
    // sea::sassert!(false);
    // sea::sassert!(false);

    // sea::sassert!(++)
    // sea::sassert!(
    //     match x.cmp(&decoded) {
    //         std::cmp::Ordering::Equal => true,
    //         _ => false,
    //     }
    // );

    // sea::sassert!(x == decoded);

    // let mut v: Vec<i32> = vec![1, 2, 3, 4];
    // let encoded: Vec<u8> = v.try_to_vec().unwrap();
    // let decoded: Vec<i32> = Vec::<i32>::try_from_slice(&encoded).unwrap();
    // sea::sassert!(v == decoded);
    // sea::sassert!(false);

}

// #[no_mangle] extern "C" fn __rust_probestack () {}


// Issue: does not work (probably in int bcmp() )
#[no_mangle]
fn test_fixed_sized_arrays() {
    let y: i32 = sea::nd_i32();
    sea::assume(y > 0);
    let x: [i32; 3] = [0, 0, y];

    let encoded: Vec<u8> = x.try_to_vec().unwrap();
    let decoded: [i32; 3] = <[i32; 3]>::try_from_slice(&encoded).unwrap();

    sea::sassert!(x.len() == decoded.len());
    sea::sassert!(x[0] == decoded[0]);
    // sea::sassert!(decoded[2] >= 0);
    // sea::sassert!(x[2] == decoded[2]);


    // for i in 0..5 {
    //     sea::sassert!(x[i] == 0);
    //     sea::sassert!(decoded[i] == 0);
    // }
    
    // sea::sassert!(x == decoded);
}

// Issue: does not work, TBD
#[no_mangle]
fn test_hashmap() {
    // use borsh::maybestd::collections::HashMap;
    // let mut map: HashMap<u32, u32> = HashMap::default();
}

// #[no_mangle]
// fn test_combinations() {

// }




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

    // The rest work fine

    // let x: i8 = sea::nd_i8();
    // let encoded = x.try_to_vec().unwrap();
    // let decoded = i8::try_from_slice(&encoded).unwrap();
    // sea::sassert!(x == decoded);
    // let x: u16 = sea::nd_u16();
    // let encoded = x.try_to_vec().unwrap();
    // let decoded = u16::try_from_slice(&encoded).unwrap();
    // sea::sassert!(x == decoded);
    // let x: i16 = sea::nd_i16();
    // let encoded = x.try_to_vec().unwrap();
    // let decoded = i16::try_from_slice(&encoded).unwrap();
    // sea::sassert!(x == decoded);
    // let x: u32 = sea::nd_u32();
    // let encoded = x.try_to_vec().unwrap();
    // let decoded = u32::try_from_slice(&encoded).unwrap();
    // sea::sassert!(x == decoded);
    // let x: i32 = sea::nd_i32();
    // let encoded = x.try_to_vec().unwrap();
    // let decoded = i32::try_from_slice(&encoded).unwrap();
    // sea::sassert!(x == decoded);
    // let x: u64 = sea::nd_u64();
    // let encoded = x.try_to_vec().unwrap();
    // let decoded = u64::try_from_slice(&encoded).unwrap();
    // sea::sassert!(x == decoded);
    // let x: i64 = sea::nd_i64();
    // let encoded = x.try_to_vec().unwrap();
    // let decoded = i64::try_from_slice(&encoded).unwrap();
    // sea::sassert!(x == decoded);
    // let x: usize = sea::nd_usize();
    // let encoded = x.try_to_vec().unwrap();
    // let decoded = usize::try_from_slice(&encoded).unwrap();
    // sea::sassert!(x == decoded);
    // let x: isize = sea::nd_isize();
    // let encoded = x.try_to_vec().unwrap();
    // let decoded = isize::try_from_slice(&encoded).unwrap();
    // sea::sassert!(x == decoded);
    // let x: usize = sea::nd_uintptr();
    // let encoded = x.try_to_vec().unwrap();
    // let decoded = usize::try_from_slice(&encoded).unwrap();
    // sea::sassert!(x == decoded);
    // let x: isize = sea::nd_intptr();
    // let encoded = x.try_to_vec().unwrap();
    // let decoded = isize::try_from_slice(&encoded).unwrap();
    // sea::sassert!(x == decoded);
}
