#[no_mangle]
pub extern "C" fn option_or(x: i32, y: i32) -> i32 {
    let val1: Option<i32> = if x % 2 == 1 { 
        None 
    } else { 
        Some(x)
    };

    let val2: Option<i32> = Some(y);

    val1.or(val2).unwrap()
}
