#[no_mangle]
pub extern "C" fn option_unwrap_or(x: i32) -> i32 {
    let result: Option<i32> = if x % 2 == 1 { 
        None 
    } else { 
        Some(x)
    };

    result.unwrap_or(0)
}
