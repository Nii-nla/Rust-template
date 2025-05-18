// tests/marcotestlib.rs

use hello_marco::marco_polo;

#[test]
fn test_exact_marco() {
    assert_eq!(marco_polo("Marco"), "Polo");
}

#[test]
fn test_not_marco() {
    assert_eq!(marco_polo("Alice"), "What's your name?");
    assert_eq!(marco_polo("marco"), "What's your name?");
    assert_eq!(marco_polo(" Marco "), "What's your name?");
}
