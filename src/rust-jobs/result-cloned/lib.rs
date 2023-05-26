#[no_mangle]
pub extern "C" fn clone(x: i32) -> i32 {
    let mut val = x;
    let x: Result<&mut i32, i32> = Ok(&mut val);
    let cloned = x.cloned();
    cloned.unwrap()*2
}
