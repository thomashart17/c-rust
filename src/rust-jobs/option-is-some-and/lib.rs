#![feature(is_some_with)]
#[no_mangle]
pub extern "C" fn option_is_some_and(x: i32) -> bool {
    let value: Option<i32> = if x % 2 == 0 {
        Some(x)
    } else {
        None
    };

    value.is_some_and(|num| num % 2 == 1)
}
