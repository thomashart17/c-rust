#![no_std]
#![feature(core_panic)]


use serde::{Serialize, Deserialize, Serializer, Deserializer};
// use heapless::Vec;

use sea;
// use serde::{Serialize, Deserialize};
// use core::fmt::Error;

// use serde_cbor::{to_vec, from_slice};
// use serde::__private::Vec;

extern crate alloc;
use alloc::vec::Vec;
use serde_json_core::to_string;
use serde_json_core::to_vec;


#[no_mangle]
pub extern "C" fn entrypt() {
    test();


    // test_primitive_struct_enum();
    // panic!();
}
// use heapless::String;
use alloc::string::String;
use crate::alloc::string::ToString;
#[no_mangle]
fn test() {
    let mut v: i32  = sea::nd_i32();
    sea::assume(v > 0);
    let original: i32 = v;

    let n: *mut i32 = &mut v;

    unsafe {
        *n = *n + 1;
        *n = *n + 1;
    }

    sea::sassert!(v == original + 2);

    let x: bool = sea::nd_bool();
    if x { panic!(); }
    // sea::sassert!(false);
    let data: u32 = 10;

    // let serialized: serde_json_core::heapless::String<1> = serde_json_core::to_string(&data).unwrap();
    // let serialized: String = serde_json_core::to_string::<u32, 1>(&data).unwrap().to_string();
    // panic!();
    // // sea::sassert!(false);

    // let deserialized: u32 = serde_json_core::from_str(&serialized).unwrap().0;
    

    // sea::sassert!(data == deserialized);


    // sea::sassert!(data == deserialized.1);
    // sea::sassert!(false);


    // let serialized: String = serde_json_core::to_string::<Point, 21>(&data).unwrap().to_string();
    // let deserialized: Point = serde_json_core::from_str(&serialized).unwrap().0;
}

// #[no_mangle]
// fn test_primitive_struct_enum() -> Result<(), Error> {
    // #[derive(Serialize, Deserialize, PartialEq)]
    // struct Point {
    //     i: NumType,
    //     j: NumType,
    //     k: NumType,
    // }
    // #[derive(Serialize, Deserialize, PartialEq)]
    // enum NumType {
    //     U8(u8),
    //     U32(u32),
    //     I8(i8),
    //     I32(i32),
    //     USIZE(usize),
    //     ISIZE(isize),
    // }
    // #[derive(Serialize, Deserialize, PartialEq)]
    // struct Quaternion {
    //     r: NumType,
    //     c: Point,
    // }
    // fn nd_num_type() -> NumType {
    //     let num_type: u8 = sea::nd_u8();
    //     sea::assume(num_type < 6);
    //     match num_type {
    //         0 => NumType::U8(sea::nd_u8()),
    //         1 => NumType::U32(sea::nd_u32()),
    //         2 => NumType::I8(sea::nd_i8()),
    //         3 => NumType::I32(sea::nd_i32()),
    //         4 => NumType::USIZE(sea::nd_usize()),
    //         5 => NumType::ISIZE(sea::nd_isize()),
    //         _ => panic!(),
    //     }
    // }
    // let q: Quaternion = Quaternion {
    //     // r: nd_num_type(),
    //     // c: Point {
    //     //     i: nd_num_type(),
    //     //     j: nd_num_type(),
    //     //     k: nd_num_type(),
    //     // }
    //     r: NumType::I32(0),
    //     c: Point {
    //         i: NumType::I32(0),
    //         j: NumType::I32(0),
    //         k: NumType::I32(0),
    //     }
    // };

    // let serialized = serde_json_core::to_string(&q).unwrap();
    // let deserialized: Quaternion = serde_json_core::from_str(&serialized).unwrap();
    
    // let serialized = serde_json::to_string(&q).unwrap();
    // let deserialized: Quaternion = serde_json::from_str(&serialized).unwrap();
    

    // sea::sassert!(q == deserialized);
    // panic!();

    // serialized: Quaternion = serde_json::from_str(&serialized).unwrap();

    // #[derive(Serialize, Deserialize, Debug)]
    // struct MyData {
    //     value: u32,
    // }

    // let data: MyData = MyData { value: 42 };

    // let encoded: Vec<u8> = serde_cbor::to_vec(&data).unwrap();
    // let decoded: MyData = serde_cbor::from_slice(&encoded).unwrap();

    // Ok(())
    // Print the deserialized value
    // println!("Deserialized value: {}", decoded.value);
    // sea::sassert!(false);
// }