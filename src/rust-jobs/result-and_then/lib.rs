#![no_std]
pub use sea;
// use sea::nd_i32;

extern crate alloc;
use alloc::string::String;
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn entrypt() {
    let x = sea::nd_i32();
    unsafe {
        sea::sea_printf("x, res *************************".as_ptr() as *const i8, (x >= 0) as c_int, x);
    }
    let res = check_and_return(x);
    unsafe {
        sea::sea_printf("x, res *************************".as_ptr() as *const i8, x, res);
    }
    if x >= 0 {
        unsafe {
            sea::sea_printf("x, res *************************".as_ptr() as *const i8, (x >= 0) as c_int, x, res);
        }
        sea::sassert!(res == x);
    } else {
        sea::sassert!(res == -1);
    }
   
    // let mut x = sea_nd_arg();
    // let res = check_and_return(x);
    
    // sea::assume(x >= 0);
    // sea::sassert!(res == x);
    // x = sea_nd_arg();
    // sea::assume(x < 0);
    // sea::sassert!(res == -1);
}


#[no_mangle]
extern "C" fn check_and_return(x: i32) -> i32 {
    let result = check(x)
        .and_then(return_value);
    match result {
        Ok(value) => value,
        Err(_) => -1,
    }
}

fn check(x: i32) -> Result<i32, String> {
    if x >= 0 { Ok(x) }
    else { Err(String::from("Error")) }
}

fn return_value(x: i32) -> Result<i32, String> { Ok(x) }
 