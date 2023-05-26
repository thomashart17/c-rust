#[no_mangle]
pub extern "C" fn option_and_then(x: i32) -> i32 {
    let result: Option<i32> = if (x & 1) == 1 {
        None
    } else {
        Some(x)
    };

    match double(result) {
        Some(val) => val,
        None => 0
    }
}

fn double(x: Option<i32>) -> Option<i32> {
    x.and_then(|num| Some(num * 2))
}
