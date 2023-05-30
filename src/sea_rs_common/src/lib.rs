// C function signature:
// #[no_mangle]
// pub extern "C" fn _____(...) -> ... {}
#![feature(lang_items)]
#![no_std]


#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]
extern crate alloc;
use core::panic::PanicInfo;
use core::intrinsics;



use core::alloc::{GlobalAlloc, Layout};

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

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    intrinsics::abort();
}

#[alloc_error_handler]
fn alloc_error_handler(_layout: core::alloc::Layout) -> ! {
    intrinsics::abort();
}

