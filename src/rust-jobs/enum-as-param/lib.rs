#![no_std]
pub use sea;

sea::define_sea_nd!(sea_nd_int, i32, 42);

#[repr(C)]
pub enum CEnum {
    KValOne,
    KValTwo,
    KValThree
}

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: i32 = sea_nd_int();
    sea::assume(v == 102);

    let result: i32 = enum_param_test(CEnum::KValTwo);

    sea::sassert!(result == v);
}

#[no_mangle]
fn enum_param_test(param: CEnum) -> i32 {
    match param {
        CEnum::KValOne => 101,
        CEnum::KValTwo => 102,
        CEnum::KValThree => 103
    }
}