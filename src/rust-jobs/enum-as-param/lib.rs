#[repr(C)]
pub enum CEnum {
    KValOne,
    KValTwo,
    KValThree
}

#[no_mangle]
pub extern "C" fn enum_param_test(param: CEnum) -> i32 {
    match param {
        CEnum::KValOne => 101,
        CEnum::KValTwo => 102,
        CEnum::KValThree => 103
    }
}