#![no_std]
pub use sea_rs_common::CAllocator;

extern crate core;
use core::result::Result;

#[no_mangle]
pub extern "C" fn iter_mut(input: i32) -> i32 {
    if input >= 0 {
        let mut x: Result<i32, &str> = Ok(input);
        match x.iter_mut().next() {
            Some(v) => *v = input,
            None => {},
        };
        let y: Result<i32, i32> = Err(x.unwrap());
        match y {
            Ok(value) => value,
            Err(err) => err,
        }
    } else {
        let y: Result<i32, i32> = Err(input);
        let mut x: Result<i32, &str> = Ok(match y {
            Ok(value) => value,
            Err(err) => err,
        });
        
        match x.iter_mut().next() {
            Some(v) => *v = input,
            None => {},
        };
        x.unwrap()*2
    }

}

// #[no_mangle]
// #[inline(never)]
// pub extern "C" fn sea_nd_foo() -> i32 {
//     0
// }

