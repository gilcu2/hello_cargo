pub fn sum(a: i8, b: i8) -> i8 { private_sum(a, b) }

fn private_sum(a: i8, b: i8) -> i8 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_integration() {
        assert_eq!(private_sum(6, 8), 14);
    }
}