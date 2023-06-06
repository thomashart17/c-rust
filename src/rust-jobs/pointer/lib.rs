
#[no_mangle]
pub extern "C" fn modify_ptr(n: *mut i32) {
    unsafe {
        *n = *n + 1;
        *n = *n + 1;
    }
}