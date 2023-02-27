// C function signature:
// #[no_mangle]
// pub extern "C" fn _____(...) -> ... {}

#[no_mangle]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[no_mangle]
pub extern "C" fn vec_test() -> u32 {
    let mut v = vec![1, 2, 3]; // L = 3

    v.push(4); // L = 4
    v.push(5); // L = 5
    v.push(6); // L = 6

    v.pop(); // L = 5

    v.len() as u32
}

#[no_mangle]
pub extern "C" fn modify_ptr(n: *mut i32) {
    unsafe {
        *n = *n + 1;
        *n = *n + 1;
    }
}