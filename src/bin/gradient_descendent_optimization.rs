extern crate optimization;

use optimmization::{Minimizer, GradientDescent, NumericalDifferentiation, Func};

fn main() {
    let function = NumericalDifferentiation::new(Func(|x: &[f64]| {
        (1.0 - x[0]).powi(2) + 100.0*(x[1] - x[0].powi(2)).powi(2)
    }));

    let minimizer = GradientDescent::new();

    let solution = minimizer.minimize(&function, vec![-3.0, -4.0]);

    println!("Found solution for Rosenbrock function at f({:?}) = {:?}",
             solution.position, solution.value);

}
