#![no_std]

use sea;

sea::define_sea_nd!(sea_nd_arg, i32, 42);

macro_rules! print { ($($args:tt)*) => { } }
macro_rules! println { ($($args:tt)*) => { } }
macro_rules! eprint { ($($args:tt)*) => { } }
macro_rules! eprintln { ($($args:tt)*) => { } }

#[no_mangle]
pub extern "C" fn entrypt() {
    let v: i32 = sea_nd_arg();
    sea::assume(v >= 1);
    let result: i32 = v * 2;

    print!("test");
    println!("test");
    print!("test {}", 42);
    println!("test {}", 42);
    eprint!("test");
    eprintln!("test");
    eprint!("test {}", 42);
    eprintln!("test {}", 42);

    sea::sassert!(result > v);
}
