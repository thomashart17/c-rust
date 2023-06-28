use core::panic::PanicInfo;
use core::intrinsics;


#[cfg(feature = "panic_error")]
use crate::seahorn::verifier_error;

#[no_mangle]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(feature = "panic_error")]
    verifier_error();

    intrinsics::abort();
}

#[no_mangle]
#[alloc_error_handler]
fn alloc_error_handler(_layout: core::alloc::Layout) -> ! {
    #[cfg(feature = "panic_error")]
    verifier_error();

    intrinsics::abort();
}

