
#[no_mangle]
pub extern "C" fn transpose(input: i32) -> i32 {
    let x: Result<Option<i32>, String> = Ok(Some(input));
    let y: Option<Result<i32, String>> = Some(Ok(input));
    x.transpose().unwrap().unwrap() +  y.unwrap().unwrap()
}
