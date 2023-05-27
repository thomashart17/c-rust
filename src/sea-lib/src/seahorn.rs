use bindings;

#[no_mangle]
pub fn verifier_error() {
  unsafe { bindings::__VERIFIER_error(); }
}

#[no_mangle]
pub fn assume(v: bool) {
  unsafe { bindings::__VERIFIER_assume(v.into()); }
}

#[no_mangle]
pub fn nd_i32() -> i32 {
  unsafe { bindings::sea_nd_i32() }
}

#[no_mangle]
#[inline(never)]
pub fn nd2_i32() -> i32 { 42 }

#[macro_export]
macro_rules! sassert {
    ($cond:expr) => {{
        if ! $cond {
    	    sea::verifier_error();
        }
    }}
}


