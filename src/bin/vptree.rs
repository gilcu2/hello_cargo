extern

fn main() {}

#[cfg(test)]
mod tests {
    use crate::sum;

    fn sum_inputs_and_outputs() -> Vec<((i8, i8), i8)> {
        vec![
            ((1, 1), 2),
            ((0, 0), 0),
            ((2, -2), 0),
        ]
    }

    #[test]
    fn test_sums() {
        for (input, output) in sum_inputs_and_outputs() {
            assert_eq!(sum(input.0, input.1), output);
        }
    }
}