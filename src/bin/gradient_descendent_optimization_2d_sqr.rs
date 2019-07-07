extern crate optimization;

use optimization::{Minimizer, GradientDescent, NumericalDifferentiation, Func};

fn main() {
    let center = [2.0, 3.0];
    let sqr2d = NumericalDifferentiation::new(Func(|x: &[f64]| {
        (x[0] - center[0]).powi(2) + (x[1] - center[1]).powi(2)
    }));

    let minimizer = GradientDescent::new();

    let solution = minimizer.minimize(&sqr2d, vec![3.0, 4.0]);

    println!("Found solution for 2d sqr function at f({:?}) = {:?}",
             solution.position, solution.value);
}
