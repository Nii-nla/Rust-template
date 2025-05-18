// tests/calculator_tests.rs

// Import your crate (replace `calc` with your actual crate name)
use calc::{add, subtract, multiply, divide};

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(5, 3), 2);
    assert_eq!(subtract(0, 5), -5);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(4, 3), 12);
    assert_eq!(multiply(-2, 3), -6);
}

#[test]
fn test_divide() {
    assert_eq!(divide(10, 2), 5);
    assert_eq!(divide(9, 3), 3);
}

#[test]
#[should_panic(expected = "attempt to divide by zero")]
fn test_divide_by_zero() {
    divide(1, 0);
}
