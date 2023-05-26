#[no_mangle]
pub extern "C" fn unwrap_or_else(x: i32, y: i32) -> i32 {
    let result = divide_result(x, y);

    let value = result.unwrap_or_else(|_| {
        -1
    });
    value
}

fn divide_result(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else if a < 0 || b < 0 {
        Err(String::from("Cannot have negative values"))
    } else {
        Ok(a / b)
    }
}