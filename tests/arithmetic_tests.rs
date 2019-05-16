extern crate hello_cargo;

use hello_cargo::arithmetic;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_integration() {
        assert_eq!(arithmetic::sum(6, 8), 14);
    }
}