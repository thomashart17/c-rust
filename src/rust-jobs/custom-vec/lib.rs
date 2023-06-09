use sea;

use std::alloc::{self, Layout};
use std::mem;
use std::ptr::NonNull;
use std::ptr;

sea::define_sea_nd!(sea_nd_usize, usize, 42);

pub struct CustomVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> CustomVec<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        CustomVec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // This can't overflow since self.cap <= isize::MAX.
            let new_cap = 2 * self.cap;

            // `Layout::array` checks that the number of bytes is <= usize::MAX,
            // but this is redundant since old_layout.size() <= isize::MAX,
            // so the `unwrap` should never fail.
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap { self.grow(); }
    
        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), elem);
        }
    
        // Can't fail, we'll OOM first.
        self.len += 1;
    }
    
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.ptr.as_ptr().add(self.len)))
            }
        }
    }
}

impl<T> Drop for CustomVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() { }
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn entrypt() {
    test_new();
    test_grow();
    test_push();
    test_pop();
    test_drop();
}

#[no_mangle]
fn test_new() {
    let v: CustomVec<i32> = CustomVec::new();
    sea::sassert!(v.len == 0);
    sea::sassert!(v.cap == 0);
    sea::sassert!(!v.ptr.as_ptr().is_null());
}

#[no_mangle]
fn test_grow() {
    let original = sea_nd_usize();

    let mut v: CustomVec<i32> = CustomVec::new();
    v.len = original;
    v.cap = original;

    v.grow();

    sea::sassert!(v.len == original);
    sea::sassert!(v.cap == 2 * original);
}

#[no_mangle]
fn test_push() {
    let original = sea_nd_usize();

    let mut v: CustomVec<i32> = CustomVec::new();
    v.len = original;
    v.cap = original;

    v.push(0);

    sea::sassert!(v.len == original + 1);
    sea::sassert!(v.cap == original * 2);
}

#[no_mangle]
fn test_pop() {
    let original = sea_nd_usize();

    let mut v: CustomVec<i32> = CustomVec::new();
    v.len = original;
    v.cap = original;

    v.pop();

    sea::sassert!(v.len == original - 1);
    sea::sassert!(v.cap == original);
}

#[no_mangle]
fn test_drop() {
    let original = sea_nd_usize();

    let mut v: CustomVec<i32> = CustomVec::new();
    v.len = original;
    v.cap = original;

    drop(v);
}
