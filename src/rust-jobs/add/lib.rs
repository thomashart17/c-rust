// #[cfg(feature = "feature_no_std")]
#![no_std]
pub use sea;

// Entry point for the proof
#[no_mangle]
pub extern "C" fn entrypt() {
    test_test1();
}

#[no_mangle]
fn test_test1() {
    let mut x: i32 = sea::nd_i32();
    sea::assume(x < 10);
    x += 4;

    sea::sassert!(x < 14);
}
