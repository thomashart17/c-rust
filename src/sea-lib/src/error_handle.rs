use core::panic::PanicInfo;
use core::intrinsics;

#[cfg(feature = "panic_error")]
use crate::seahorn::verifier_error;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
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