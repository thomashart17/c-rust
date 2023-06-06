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

