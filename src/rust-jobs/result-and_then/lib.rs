#![no_std]
pub use sea;

extern crate alloc;
use alloc::string::String;

// sea::define_sea_nd!(sea_nd_int, i32, 42);

#[no_mangle]
pub extern "C" fn entrypt() {
    sea::sea_printf!("sea_printf! macro test", 5, 5, 9);

    let x = sea::nd_i32();
    let res = check_and_return(x);
    if x >= 0 {
        sea::sassert!(res == x);
    } else {
        sea::sassert!(res == -1);
    }
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
 