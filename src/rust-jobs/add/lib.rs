use sea;

// Define a sea_nd function
sea::define_sea_nd!(sea_nd_arg, i32, 42);

// Entry point for the proof
#[no_mangle]
pub extern "C" fn entrypt() {
    let v = sea_nd_arg();
    sea::assume(v >= 1);
    let res = add(v, 7);
    if v > 0 {
        sea::sassert!(res > 7);
    } else {
        sea::sassert!(res <= 7);
    }
}

// Function being verified
#[no_mangle]
fn add(x: i32, y: i32) -> i32 {
    x + y
}
