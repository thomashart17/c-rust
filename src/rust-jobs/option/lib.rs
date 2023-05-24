#[no_mangle]
pub extern "C" fn option_test(num: i32) -> i32 {
    match double_if_even(num) {
        Some(x) => x,
        None => 0
    }
}

fn double_if_even(num: i32) -> Option<i32> {
    if (num & 1) == 0 {
        Some(2 * num)
    } else {
        None
    }
}