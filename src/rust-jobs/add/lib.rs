// #[cfg(feature = "feature_no_std")]
#![no_std]
pub use sea;

sea::define_custom_print!();

// Entry point for the proof
#[no_mangle]
pub extern "C" fn entrypt() {
    test_test1();
    test_test2();
}

#[no_mangle]
fn test_test1() {
    println!("Hello, test1!");
    let v = sea::nd_i32();
    sea::assume(v >= 1);
    let res = add(v, 7);

    if v > 0 {
        sea::sassert!(res > 7);
    } else {
        sea::sassert!(res <= 7);
    }
}

#[no_mangle]
fn test_test2() {
    println!("Hello, test2!");
    let v = sea::nd_i32();
    sea::assume(v >= 1);
    let res = add(v, 8);

    if v > 0 {
        sea::sassert!(res > 8);
    } else {
        sea::sassert!(res <= 8);
    }
}

// Function being verified
#[no_mangle]
fn add(x: i32, y: i32) -> i32 {
    x + y
}
