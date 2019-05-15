extern crate hello_cargo;

#[test]
fn test_sum_integration() {
    assert_eq!(hello_cargo::sum(6, 8), 14);
}