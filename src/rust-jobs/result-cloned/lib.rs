
#[no_mangle]
pub extern "C" fn clone(x: i32) -> i32 {
    let mut val: i32 = x;
    let x: Result<&mut i32, i32> = Ok(&mut val);
    let cloned: Result<i32, i32> = x.cloned();
    cloned.unwrap()*2
}
