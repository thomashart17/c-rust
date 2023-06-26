use core::panic::PanicInfo;
use core::intrinsics;
use crate::bindings::*;

macro_rules! sea_printf {
    ($message:expr $(, $args:expr)*) => {{
        use crate::bindings::sea_printf;
        use core::ffi::c_char;
        unsafe { sea_printf($message.as_ptr() as *const c_char, $($args),*); }
    }}
}

#[cfg(feature = "panic_error")]
use crate::seahorn::verifier_error;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(feature = "panic_error")]
    sea_printf!("PANIC ERROR***");

    #[cfg(feature = "panic_error")]
    verifier_error();

    intrinsics::abort();
}

#[alloc_error_handler]
fn alloc_error_handler(_layout: core::alloc::Layout) -> ! {
    #[cfg(feature = "panic_error")]
    verifier_error();

    intrinsics::abort();
}

