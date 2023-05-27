use sea;


#[no_mangle]
pub extern "C" fn entrypt() {
  let v = sea::nd_i32();
  sea::assume(v >= 1);
  let res = add(v, 7);
  if v > 0 {
    sea::sassert!(res > 7);
  } else {
    sea::sassert!(res <= 7);
  }
}

#[no_mangle]
fn add(x: i32, y: i32) -> i32 {
    x + y
}
