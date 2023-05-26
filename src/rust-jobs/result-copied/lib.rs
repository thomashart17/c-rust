#[no_mangle]
pub extern "C" fn copied(x: i32) -> i32 {
    let val = x;
    let x: Result<&i32, i32> = Ok(&val);
    let copied = x.copied();
    copied.unwrap()*2
}
