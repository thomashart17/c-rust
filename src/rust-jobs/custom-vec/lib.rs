#![no_std]
#![feature(new_uninit)]

use sea;

extern crate alloc;
use alloc::alloc::{Layout, alloc, realloc, dealloc, handle_alloc_error};
use alloc::boxed::Box;

use core::{mem, iter};
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use core::ptr;
use core::mem::needs_drop;


#[no_mangle]
pub extern "C" fn entrypt() {
// Needs to be reevaluated
    // test_new();
    // test_grow();
    // test_pop();
    // test_push();
    // test_drop();
    // test_deref();
    // test_deref_mut();
    // test_insert();
    // test_remove();
// Needs to be analyzed
    test_into_iter();
    test_into_iter_size();
// Seems ok
    test_into_iter_drop();
    // sea::sassert!(false);
}

#[no_mangle]
fn test_into_iter_size() {
    let n: u32 = sea::nd_u32();
    sea::assume(n < 100);
    // let n = 10;

    let mut v: CustomVec<u32> = CustomVec::new();
    for i in 0..n { v.push(i); }
    
    let mut iter: IntoIter<u32> = v.into_iter();

    // This test is broken. Incorrect size can still give unsat
    for i in 0..n {
        let front: bool = sea::nd_bool();
        if front {
            _ = iter.next();
        } else {
            _ = iter.next_back();
        }
        let size: usize = (n-i-1).try_into().unwrap();
        sea::sassert!(iter.size_hint().0 == size);
    }
    // sea::sassert!(false);
}

#[no_mangle]
fn test_into_iter_drop() {
    static mut DROP_COUNT: usize = 0;
    pub struct DropTest { _value: i32, }
    impl Drop for DropTest {
        fn drop(&mut self) { unsafe { DROP_COUNT += 1; } }
    }

    let n: usize = sea::nd_usize();
    let mut v: CustomVec<DropTest> = CustomVec::new();
    for i in 0..n {
        v.push(DropTest { _value: i.try_into().unwrap() });
    }

    let mut iter: IntoIter<DropTest> = v.into_iter();
    let x: usize = sea::nd_usize();
    sea::assume(x <= n);
    for _i in 0..x {
        let front: bool = sea::nd_bool();
        if front { _ = iter.next(); }
        else { _ = iter.next_back(); }
    }
    sea::sassert!(unsafe { DROP_COUNT == x });

    drop(iter);
    sea::sassert!(unsafe { DROP_COUNT == n });
}

#[no_mangle]
fn test_into_iter() {
    let n: u32 = sea::nd_u32();

    // iterate forwards
    let mut v: CustomVec<u32> = CustomVec::new();
    for i in 0..n { v.push(i); }

    let mut iter: IntoIter<u32> = v.into_iter();
    let i: u32 = 0;
    for number in iter {
        let x: u32 = i + i;
        sea::sassert!(number == x);
    }
}



// #[no_mangle]
// fn test_into_iter() {
//     let n: u32 = sea::nd_u32();

//     // iterate forwards
//     let mut v: CustomVec<u32> = CustomVec::new();
//     for i in 0..n { v.push(i); }

//     let mut iter: IntoIter<u32> = v.into_iter();
//     let i: u32 = 0;
//     for number in iter {
//         sea::sassert!(number == i-1);
//     }

    // iterate backwards
    // let mut v: CustomVec<u32> = CustomVec::new();
    // for i in 0..n { v.push(i); }

    // let mut iter: IntoIter<u32> = v.into_iter();
    // let mut i: u32 = 1;
    // while let Some(number) = iter.next_back() {
    //     sea::sassert!(number == n - i-1);
    //     i -= 1;
    // }

    // randomly iterate
    // let mut v: CustomVec<u32> = CustomVec::new();
    // for i in 0..n { v.push(i); }

    // let mut h: u32 = v.len.try_into().unwrap();

    // let mut iter: IntoIter<u32> = v.into_iter();
    // let mut l: u32 = 0;
    // // let mut h: u32 = v.len.try_into().unwrap();
    // let mut x = 4;
    // for i in 0..n {
    //     let front: bool = sea::nd_bool();
    //     if front {
    //         sea::sassert!(iter.next().unwrap() == l);
    //         l += 1;
    //     } else {
    //         // sea::sassert!(false);
    //         let x: u32 = iter.next_back().unwrap();
    //         sea::sassert!(x == l*h);
    //         h -= 1;
    //     }
    // }
    // sea::sassert!(false);
// }


#[no_mangle]
fn test_insert() {
    let mut v: CustomVec<i32> = CustomVec::new();
    let n: usize = sea::nd_usize();
    let index: usize = sea::nd_usize();
    sea::assume(index <= n);

    for _i in 0..n { v.push(1); }
    
    v.insert(index, -1);
    let slice: &mut [i32] = &mut *v;
    sea::sassert!(slice[index] == -1);
}

#[no_mangle]
fn test_remove() {
    let mut v: CustomVec<i32> = CustomVec::new();
    let n: usize = sea::nd_usize();
    let index: usize = sea::nd_usize();
    sea::assume(index <= n);

    for i in 0..n { v.push(i.try_into().unwrap()); }
    
    let res: i32 = v.remove(index);
    sea::sassert!(res == index.try_into().unwrap());
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
    pub struct DropTest { _value: i32, }
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

pub struct IntoIter<T> {
    buf: NonNull<T>,
    cap: usize,
    start: *const T,
    end: *const T,
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

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bounds");
        if self.cap == self.len { self.grow(); }
    
        unsafe {
            ptr::copy(
                self.ptr.as_ptr().add(index),
                self.ptr.as_ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr.as_ptr().add(index), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr.as_ptr().add(index));
            ptr::copy(
                self.ptr.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.len - index,
            );
            result
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

impl<T> IntoIterator for CustomVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        let vec: ManuallyDrop<CustomVec<T>> = ManuallyDrop::new(self);

        let ptr: NonNull<T> = vec.ptr;
        let cap: usize = vec.cap;
        let len: usize = vec.len;

        unsafe {
            IntoIter {
                buf: ptr,
                cap: cap,
                start: ptr.as_ptr(),
                end: if cap == 0 {
                    ptr.as_ptr()
                } else {
                    ptr.as_ptr().add(len)
                },
            }
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result: T = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize)
                         / mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            for _ in &mut *self {}
            let layout: Layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                dealloc(self.buf.as_ptr() as *mut u8, layout);
            }
        }
    }
}