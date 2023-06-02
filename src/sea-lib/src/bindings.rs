
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
    pub fn sea_is_dereferenceable(ptr: *const i8, offset: isize) -> bool;
}
extern "C" {
    pub fn sea_assert_if(arg1: bool, arg2: bool);
}
extern "C" {
    pub fn sea_is_modified(arg1: *mut i8) -> bool;
}
extern "C" {
    pub fn sea_tracking_on();
}
extern "C" {
    pub fn sea_tracking_off();
}
extern "C" {
    pub fn sea_reset_modified(arg1: *mut ::core::ffi::c_char);
}
extern "C" {
    pub fn sea_set_shadowmem(
        arg1: ::core::ffi::c_char,
        arg2: *mut ::core::ffi::c_char,
        arg3: ::core::ffi::c_char,
    );
}
extern "C" {
    pub fn sea_get_shadowmem(
        arg1: ::core::ffi::c_char,
        arg2: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_char;
}

extern "C" {
  pub fn sea_nd_i32() -> i32;
}

extern "C" {
  pub fn sea_nd_bool() -> bool;
}
