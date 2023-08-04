use core::ffi::c_char;

extern "C" {
    #[doc = "Marks an error location for the verifier\n\nCatastrophic failure that matters."]
    pub fn __VERIFIER_error();
}
extern "C" {
    #[doc = "A condition to be assumed to be true by the verifier"]
    pub fn __VERIFIER_assume(arg1: i32);
}
extern "C" {
    pub fn __SEA_assume(arg1: bool);
}
extern "C" {
    pub fn __VERIFIER_assert(arg1: bool);
}
extern "C" {
    pub fn __VERIFIER_assert_not(arg1: bool);
}
extern "C" {
    pub fn __VERIFIER_assert_if(arg1: bool, arg2: bool);
}
extern "C" {
    pub fn sea_is_dereferenceable(ptr: *const c_char, offset: isize) -> bool;
}
extern "C" {
    pub fn sea_assert_if(arg1: bool, arg2: bool);
}
extern "C" {
    pub fn sea_is_modified(arg1: *mut c_char) -> bool;
}
extern "C" {
    pub fn sea_tracking_on();
}
extern "C" {
    pub fn sea_tracking_off();
}
extern "C" {
    pub fn sea_reset_modified(arg1: *mut c_char);
}
extern "C" {
    pub fn sea_set_shadowmem(
        arg1: c_char,
        arg2: *mut c_char,
        arg3: c_char,
    );
}
extern "C" {
    pub fn sea_get_shadowmem(
        arg1: c_char,
        arg2: *mut c_char,
    ) -> c_char;
}

extern "C" { pub fn sea_nd_i8() -> i8; }
extern "C" { pub fn sea_nd_u8() -> u8; }
extern "C" { pub fn sea_nd_i16() -> i16; }
extern "C" { pub fn sea_nd_u16() -> u16; }
extern "C" { pub fn sea_nd_i32() -> i32; }
extern "C" { pub fn sea_nd_u32() -> u32; }
extern "C" { pub fn sea_nd_i64() -> i64; }
extern "C" { pub fn sea_nd_u64() -> u64; }
extern "C" { pub fn sea_nd_usize() -> usize; }
extern "C" { pub fn sea_nd_isize() -> isize; }
extern "C" { pub fn sea_nd_uintptr() -> usize; }
extern "C" { pub fn sea_nd_intptr() -> isize; }




extern "C" {
  pub fn sea_nd_bool() -> bool;
}
extern "C" {
    pub fn sea_printf(fmt: *const c_char, ...);
}
