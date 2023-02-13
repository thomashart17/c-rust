
#[no_mangle]
pub extern "C" fn test_func() {
    priv_func();
}

#[no_mangle]
pub extern "C" fn priv_func() {
    println!("Rust in C");
}

#[no_mangle]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}