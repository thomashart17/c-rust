use crate::bindings::*;

#[no_mangle]
pub fn verifier_error() {
    unsafe {
        __VERIFIER_error();
    }
}

#[no_mangle]
pub fn assume(v: bool) {
    unsafe {
        __VERIFIER_assume(v.into());
    }
}

#[no_mangle]
pub fn nd_i8() -> i8 { unsafe { sea_nd_i8() } }
#[no_mangle]
pub fn nd_u8() -> u8 { unsafe { sea_nd_u8() } }
#[no_mangle]
pub fn nd_i16() -> i16 { unsafe { sea_nd_i16() } }
#[no_mangle]
pub fn nd_u16() -> u16 { unsafe { sea_nd_u16() } }
#[no_mangle]
pub fn nd_i32() -> i32 { unsafe { sea_nd_i32() } }
#[no_mangle]
pub fn nd_u32() -> u32 { unsafe { sea_nd_u32() } }
#[no_mangle]
pub fn nd_i64() -> i64 { unsafe { sea_nd_i64() } }
#[no_mangle]
pub fn nd_u64() -> u64 { unsafe { sea_nd_u64() } }
#[no_mangle]
pub fn nd_usize() -> usize { unsafe { sea_nd_usize() } }
#[no_mangle]
pub fn nd_uintptr() -> usize { unsafe { sea_nd_uintptr() } }

#[no_mangle]
pub fn nd_bool() -> bool { unsafe { sea_nd_bool() } }

#[macro_export]
macro_rules! sea_printf {
    ($message:expr $(, $args:expr)*) => {{
        use crate::sea::bindings::sea_printf;
        use core::ffi::c_char;

        unsafe {
            sea_printf($message.as_ptr() as *const c_char, $($args),*);
        }
    }}
}


/// Defines `sea_nd` function that returns nd value
///
/// The implementation generates a sea_nd function that
/// attempts to confuse Rust optimizer from understanding what
/// is the return of the function.
///
/// The first argument specifies the defined function name.
/// It must start with the prefix `sea_nd`.
///
/// The second argument is the type for nondet values. The type
/// is assumed to implement the Default trait
///
/// The third argument is a value to return in the implementation.
/// It must be any value that is different from default value for the type.
///
/// # Example
///
/// define_sea_nd!(sea_nd_foo, i32, 42);

#[macro_export]
macro_rules! sassert {
    ($cond:expr) => {{
        if !$cond {
            sea::verifier_error();
        }
    }};
}


// #[macro_export]
// macro_rules! define_sea_nd {
//     ($name:ident,$typ:ty,$val:expr) => {
//         #[no_mangle]
//         #[inline(never)]
//         pub extern "C" fn $name() -> $typ {
//             if sea::nd_bool() {
//                 <$typ>::default()
//             } else {
//                 $val
//             }
//         }
//     };
// }