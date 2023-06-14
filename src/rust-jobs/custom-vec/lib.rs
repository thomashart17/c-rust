#![no_std]
#![feature(new_uninit)]

use sea;

extern crate alloc;
use alloc::alloc::{Layout, alloc, realloc, dealloc, handle_alloc_error};
use alloc::boxed::Box;

use core::mem;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use core::ptr;
use core::mem::needs_drop;


#[no_mangle]
pub extern "C" fn entrypt() {
    test_new();
    test_grow();
    test_pop();
    test_push();
    test_deref();
    test_deref_mut();
    test_drop();
    // sea::sassert!(false);
}

#[no_mangle]
fn test_push() {
    let original = sea::nd_usize();
    sea::assume(original > 0);

    let mut v: CustomVec<i32> = CustomVec::new();
    v.len = original;
    v.cap = original;

    v.grow();
    v.push(0);   

    sea::sassert!(v.len == original + 1);
    sea::sassert!(v.cap == original * 2);
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
    let original = sea::nd_usize();

    let mut v: CustomVec<i32> = CustomVec::new();
    v.len = original;
    v.cap = original;

    v.grow();

    if original == 0 {
        sea::sassert!(v.cap == 1)
    } else {
        sea::sassert!(v.cap == 2 * original);
    }
    sea::sassert!(v.len == original);
}

#[no_mangle]
fn test_pop() {
    let original = sea::nd_usize();
    sea::assume(original > 0);

    let mut v: CustomVec<i32> = CustomVec::new();
    v.len = original;
    v.cap = original;

    v.grow();
    v.pop();

    sea::sassert!(v.len == original - 1);
    sea::sassert!(v.cap == original * 2);
}

#[no_mangle]
#[inline(never)]
fn test_drop() {
    pub struct DropTest {  _value: i32, }
    impl Drop for DropTest {
        fn drop(&mut self) { unsafe { DROP_COUNT += 1; } }
    }
    static mut DROP_COUNT: usize = 0;

    let original: usize = sea::nd_usize();
    let num_pops: usize = sea::nd_usize();
    sea::assume(num_pops <= original);

    let mut v: CustomVec<DropTest> = CustomVec::new();
    for i in 0..original { v.push(DropTest { _value: i.try_into().unwrap() }); }
    for _i in 0..num_pops { _ = v.pop(); }
    v.push(DropTest { _value: 1 });
    _ = v.pop();
    _ = v.pop();
    _ = v.pop();

    drop(v);
    sea::sassert!(unsafe { DROP_COUNT == original+1 });
}

#[no_mangle]
fn test_deref() {
    let original: usize = sea::nd_usize();
    let num_pops: usize = sea::nd_usize();
    sea::assume(num_pops <= original);

    let mut v: CustomVec<i32> = CustomVec::new();
    for i in 0..original { v.push(i.try_into().unwrap()); }
    for _i in 0..num_pops { _ = v.pop(); }
    v.push(1);
    let slice: &[i32] = &*v;
    sea::sassert!(slice.len() == original - num_pops + 1);
    sea::sassert!(slice[slice.len()-1] == 1);
}

#[no_mangle]
fn test_deref_mut() {
    let mut v: CustomVec<i32> = CustomVec::new();
    v.push(0);
    v.push(3);
    v.push(5);

    let slice: &mut [i32] = &mut *v;
    let length: usize = slice.len();
    slice[0] = 10;
    slice[1] = 40;
    slice.sort();

    sea::sassert!(length == 3);
    sea::sassert!(v.pop() == Some(40));
    sea::sassert!(v.pop() == Some(10));
    sea::sassert!(v.pop() == Some(5));
    sea::sassert!(v.len == 0);
}

pub struct CustomVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> CustomVec<T> {
    pub fn new() -> Self {
        if needs_drop::<T>() {
            // sea::sassert!(false);
            // Cannot handle types that need to be dropped due to loop in Drop()
        }
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
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => handle_alloc_error(new_layout),
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
            // if needs_drop::<T>() {
            //     // This does not work with seahorn
            //     // while let Some(_) = self.pop() { }
            // }
            if core::mem::needs_drop::<T>() {
                let typed_ptr = self.ptr.as_ptr();
                let slice = unsafe { core::slice::from_raw_parts_mut(typed_ptr, self.len) };

                let new_slice: &mut [T] = {
                    let buffer = Box::<[T]>::new_uninit_slice(self.len);
                    let ptr = Box::into_raw(buffer) as *mut T;
                    unsafe { core::slice::from_raw_parts_mut(ptr, self.len) }
                };
                unsafe {
                    ptr::copy_nonoverlapping(slice.as_ptr(), new_slice.as_mut_ptr(), self.len);
                    _ = Box::from_raw(new_slice as *mut [T]);
                }
            }
            
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T> Deref for CustomVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            core::slice::from_raw_parts(self.ptr.as_ptr(), self.len)
        }
    }
}

impl<T> DerefMut for CustomVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len)
        }
    }
}
