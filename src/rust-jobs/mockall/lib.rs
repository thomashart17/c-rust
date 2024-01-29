// #[cfg(feature = "feature_no_std")]
// #![no_std]
pub use sea;

use mockall::*;
use mockall::predicate::*;

// Entry point for the proof
#[no_mangle]
pub extern "C" fn entrypt() {
    test_test1();
}

#[automock]
trait MyTrait {
  fn foo(&self, x: u32) -> u32;
  fn bar(&self) -> u32;
}

fn call_bar(x: &dyn MyTrait) -> u32 {
    x.bar()
}

fn call_foo_with_four(x: &dyn MyTrait) -> u32 {
    x.foo(4)
}

fn call_foo(x: &dyn MyTrait, val: u32) -> u32 {
    x.foo(val)
}

#[no_mangle]
fn test_test1() {
    let mut x: i32 = sea::nd_i32();
    sea::assume(x < 10);
    x += 4;

    sea::sassert!(x < 14);
}

#[no_mangle]
fn test_test2() {
    let mut x: u32 = sea::nd_u32();
    sea::assume(x < 10);

    let mut mock = MockMyTrait::new();
    mock.expect_bar()
        .return_const(x);
    mock.expect_foo()
        .with(predicate::lt(10))
        .times(1)
        .returning(|x| x + 4);

    sea::sassert!(call_bar(&mock) == x);
    sea::sassert!(call_foo(&mock, x) < 14);
}

// #[cfg(test)]
mod tests {
    use super::*;

    fn i32_func() -> i32 {
        return 2;
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_func() {
        assert_eq!(i32_func(), 2);
    }

    #[test]
    fn test_mocks() {
        let mut mock = MockMyTrait::new();
        mock.expect_bar()
            .return_const(42u32);

        mock.expect_foo()
            .with(predicate::eq(4))
            .times(1)
            .returning(|x| x + 1);

        assert_eq!(42, call_bar(&mock));
        assert_eq!(5, call_foo_with_four(&mock));
    }
}