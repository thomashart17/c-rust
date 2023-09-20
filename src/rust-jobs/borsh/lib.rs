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
    let x: String = "abcd".to_string();
    
    let encoded: Vec<u8> = x.try_to_vec().unwrap();

    let decoded: String = String::try_from_slice(&encoded).unwrap();

    if decoded != x {
        sea::sassert!(false);
    }

    sea::sassert!(decoded == x);

    // sea::sassert!(x == decoded);
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
