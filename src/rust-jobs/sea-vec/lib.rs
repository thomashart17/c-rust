#![no_std]
use sea;
use sea::{ SeaVec, sea_vec, sassert };


#[no_mangle]
pub extern "C" fn entrypt() {
    test_macros();
    test_push_pop();
    test_indexing();
    test_deref_deref_mut();
    test_insert_remove();

    test_drop();
    test_iter();
    test_drain();
    test_zst();

    // sassert!(false);
}


#[no_mangle]
fn test_drop() {
    static mut DROP_COUNT: usize = 0;
    struct DropTest { _x: usize }
    impl Drop for DropTest {
        fn drop(&mut self) { unsafe { DROP_COUNT += 1; } }
    }

    let mut v: SeaVec<DropTest> = SeaVec::new(8);
    for i in 0..5 { v.push( DropTest { _x: i }); }
    _ = v.pop();
    _ = v.pop();
    sassert!(unsafe { DROP_COUNT == 2 });
    drop(v);
    sassert!(unsafe{ DROP_COUNT == 5 });
}

#[no_mangle]
fn test_macros() {
    let mut v: SeaVec<i32> = sea_vec![1, 2, 3, 4];
    sassert!(v.len() == 4);
    sassert!(v.cap() == 8);
    for i in 0..4 { sassert!(v.pop() == Some(4-i)); }
    sassert!(v.pop() == None && v.len() == 0);

    let mut v: SeaVec<i32> = sea_vec!([1, 2, 3]; 12);
    sassert!(v.len() == 3);
    sassert!(v.cap() == 12);
    for i in 0..3 { sassert!(v.pop() == Some(3-i)); }
    sassert!(v.pop() == None && v.len() == 0);

    let mut v: SeaVec<i32> = sea_vec!(0; 3; 8);
    sassert!(v.len() == 3);
    sassert!(v.cap() == 8);
    for _ in 0..3 { sassert!(v.pop() == Some(0)); }
    sassert!(v.pop() == None && v.len() == 0);

    let mut v = sea_vec!(0; 4);
    sassert!(v.len() == 4);
    sassert!(v.cap() == 8);
    for _ in 0..4 { sassert!(v.pop() == Some(0)); }
    sassert!(v.pop() == None && v.len() == 0);
}

#[no_mangle]
fn test_push_pop() {
    let mut v: SeaVec<usize> = SeaVec::new(10);
    for i in 0..10 {
        sassert!(v.len() == i);
        v.push(i);
    }
    sassert!(v.cap() == 10);

    for i in 0..10 {
        sassert!(v.len() == 10-i);
        sassert!(v.pop() == Some(9-i));
    }
    sassert!(v.pop() == None);
    sassert!(v.cap() == 10 && v.len() == 0);
}

#[no_mangle]
fn test_indexing() {
    let mut v: SeaVec<usize> = sea_vec![0, 1, 2, 3, 4, 5];
    for i in 0..6 {
        sassert!(v[i] == i);
        v[i] = i*10;
        sassert!(v[i] == i*10);
    }
    sassert!(v.len() == 6 && v.cap() == 12);
}

#[no_mangle]
fn test_deref_deref_mut() {
    let mut v: SeaVec<usize> = sea_vec![0, 1, 2, 3];

    let mut i: usize = 0;
    for elem in &*v {
        sassert!(*elem == i);
        i += 1;
    }

    let slice: &mut [usize] = &mut *v;
    for i in 0..4 { slice[i] *= 10; }
    
    let mut i: usize = 0;
    for elem in &*v {
        sassert!(*elem == 10*i);
        i += 1;
    }

    sassert!(v.len() == 4 && v.cap() == 8);
}

#[no_mangle]
fn test_insert_remove() {
    let mut v: SeaVec<usize> = sea_vec![0, 1, 2, 3, 4];
    v.insert(5, 5);
    sassert!(v.len() == 6);
    sassert!(v.pop() == Some(5));
    
    sassert!(v.remove(2) == 2);
    sassert!(v.len() == 4);

    v.insert(2, 20);
    sassert!(v.remove(2) == 20);
    v.insert(0, 0);
    sassert!(v.remove(4) == 4);
    sassert!(v.len() == 4);
}

#[no_mangle]
fn test_iter() {
    let v: SeaVec<usize> = sea_vec![0, 1, 2, 3, 4, 5];

    let mut iter: sea::IntoIter<usize> = v.into_iter();
    for i in 0..6 {
        if i%2 == 0 {
            sassert!(iter.next() == Some(i/2));
        } else {
            sassert!(iter.next_back() == Some(5-i/2));
        }
        sassert!(iter.size_hint() == (5-i, Some(5-i)));
    }


    static mut DROP_COUNT: usize = 0;
    struct DropTest { _x: usize }
    impl Drop for DropTest {
        fn drop(&mut self) { unsafe { DROP_COUNT += 1; } }
    }

    let mut v: SeaVec<DropTest> = SeaVec::new(6);
    for i in 0..6 { v.push(DropTest { _x: i }); }
    let mut iter = v.into_iter();
    _ = iter.next();
    _ = iter.next();
    _ = iter.next();
    sassert!(unsafe { DROP_COUNT == 3 });

    drop(iter);
    sassert!(unsafe { DROP_COUNT == 6 });
}

#[no_mangle]
fn test_drain() {
    let mut v: SeaVec<usize> = sea_vec![0, 1, 2, 3, 4, 5];
    let mut drain: sea::Drain<'_, usize> = v.drain(0..6);
    for i in 0..6 {
        if i%2 == 0 { sassert!(drain.next() == Some(i/2)); }
        else { sassert!(drain.next_back() == Some(5-i/2)); }
        sassert!(drain.size_hint() == (5-i, Some(5-i)));
    }
    drop(drain);
    sassert!(v.len() == 0 && v.cap() == 12 && v.pop() == None);

    static mut DROP_COUNT: usize = 0;
    struct DropTest { x: usize }
    impl Drop for DropTest {
        fn drop(&mut self) { unsafe { DROP_COUNT += 1; } }
    }
    let cap = 8;
    let mut v: SeaVec<DropTest> = SeaVec::new(cap);
    for i in 0..4 { v.push(DropTest { x: i }); }

    let mut drain: sea::Drain<'_, DropTest> = v.drain(1..3);
    sassert!(drain.next().unwrap().x == 1);
    sassert!(drain.size_hint() == (1, Some(1)));
    sassert!(unsafe { DROP_COUNT == 1 });
    drop(drain);
    sassert!(unsafe { DROP_COUNT == 2 });

    sassert!(v[0].x == 0 && v[1].x == 3 && v.len() == 2);
    
    // elements must be popped before drop, or else SeaHorn will always give unsat
    for _ in 0..4 { v.pop(); }
    drop(v);

    sassert!(unsafe { DROP_COUNT == 4 });
}

#[no_mangle]
fn test_zst() {
    static mut DROP_COUNT: usize = 0;
    #[derive(PartialEq)]
    struct ZST {}
    impl Drop for ZST {
        fn drop(&mut self) { unsafe { DROP_COUNT += 1; } }
    }

    let mut v: SeaVec<ZST> = SeaVec::new(5);
    v.push(ZST {});
    v.push(ZST {});
    v.insert(2, ZST {});
    v[0] = ZST {};

    sassert!(v[0] == v[1] && v[1] == v[2]);
    sassert!(v.remove(1) == ZST {}); // checking for equivalence increases drop count by one

    sassert!(v.len() == 2);
    v.insert(0, ZST {});
    sassert!(unsafe { DROP_COUNT == 3 });

    let mut iter: sea::IntoIter<ZST> = v.into_iter();
    sassert!(iter.next() == Some(ZST {})); // checking for equivalence increases drop count by one
    sassert!(iter.size_hint().0 == 2);
    sassert!(iter.next_back() == Some(ZST {})); // checking for equivalence increases drop count by one
    sassert!(iter.next() == Some(ZST {})); // checking for equivalence increases drop count by one
    sassert!(iter.next() == None);
    sassert!(iter.size_hint().0 == 0);
    sassert!(unsafe { DROP_COUNT == 9 });

    let mut v: SeaVec<ZST> = SeaVec::new(5);
    for _ in 0..5 { v.push(ZST {}); }
    let mut drain = v.drain(1..4);
    sassert!(drain.next() == Some(ZST {})); // checking for equivalence increases drop count by one
    sassert!(drain.size_hint().0 == 2);
    sassert!(drain.next_back() == Some(ZST {})); // checking for equivalence increases drop count by one
    sassert!(drain.next() == Some(ZST {})); // checking for equivalence increases drop count by one
    sassert!(drain.next() == None);
    sassert!(drain.size_hint().0 == 0);
    sassert!(unsafe { DROP_COUNT == 15 });
    drop(drain);
    v.pop();
    v.pop();
    sassert!(v.pop() == None && v.len() == 0);
    drop(v);
}

