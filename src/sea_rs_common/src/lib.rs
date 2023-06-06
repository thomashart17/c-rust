// C function signature:
// #[no_mangle]
// pub extern "C" fn _____(...) -> ... {}
// #![feature(lang_items)]

// **************************
// Global Allocator is automatically read by std
//
// When using no_std, this lib must be used to
// have an Allocator, panic handler, and error handler 
// **************************

#![no_std]


#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]
use core::panic::PanicInfo;
use core::intrinsics;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort();
}
#[alloc_error_handler]
fn alloc_error_handler(_layout: core::alloc::Layout) -> ! {
    intrinsics::abort();
}


use core::alloc::{GlobalAlloc, Layout};
use core::fmt::{self, Arguments};
use libc::c_void;
pub struct CAllocator {}

#[global_allocator]
static ALLOCATOR: CAllocator = CAllocator {};

unsafe impl Sync for CAllocator {}

unsafe impl GlobalAlloc for CAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        libc::malloc(layout.size()) as *mut u8
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        libc::free(_ptr as *mut c_void);
    }
}

pub struct NullWriter;

impl NullWriter {
    pub fn write_fmt(&mut self, _: Arguments<'_>) -> fmt::Result { Ok(()) }
}

#[macro_export]
macro_rules! define_custom_print {
    () => {
        custom_print::define_macros!(
            {cprint, cprintln, ceprint, ceprintln},
            NullWriter
        );

        use cprint as print;
        use cprintln as println;
        use ceprint as eprint;
        use ceprintln as eprintln;
    };
}
