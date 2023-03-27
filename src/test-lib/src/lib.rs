// C function signature:
// #[no_mangle]
// pub extern "C" fn _____(...) -> ... {}

#![feature(lang_items)]

use std::alloc::{GlobalAlloc, Layout};

use libc::c_void;

struct CAllocator {}

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

// #[lang = "panic_fmt"]
// #[no_mangle]
// pub extern "C" fn panic_fmt() -> ! {
//     loop {}
// }

#[no_mangle]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[no_mangle]
pub extern "C" fn vec_test() -> u32 {
    let mut v = vec![1, 2, 3]; // L = 3

    v.push(4); // L = 4
    v.push(5); // L = 5
    v.push(6); // L = 6

    v.pop(); // L = 5

    v.len() as u32
}

#[no_mangle]
pub extern "C" fn modify_ptr(n: *mut i32) {
    unsafe {
        *n = *n + 1;
        *n = *n + 1;
    }
}

#[repr(C)]
pub enum CEnum {
    KValOne,
    KValTwo,
    KValThree
}

#[no_mangle]
pub extern "C" fn enum_param_test(param: CEnum) -> i32 {
    match param {
        CEnum::KValOne => 101,
        CEnum::KValTwo => 102,
        CEnum::KValThree => 103
    }
}

#[no_mangle]
pub extern "C" fn option_test(num: i32) -> i32 {
    match double_if_even(num) {
        Some(x) => x,
        None => 0
    }
}

fn double_if_even(num: i32) -> Option<i32> {
    if num % 2 == 0 {
        Some(2 * num)
    } else {
        None
    }
}