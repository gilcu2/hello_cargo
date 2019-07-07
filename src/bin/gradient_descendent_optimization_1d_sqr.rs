extern crate optimization;

use optimization::{Minimizer, GradientDescent, NumericalDifferentiation, Func};

fn main() {
    let pi = 3.14;
    let sqr = NumericalDifferentiation::new(Func(|x: &[f64]| {
        (x[0] - pi).powi(2)
    }));

    let minimizer = GradientDescent::new();

    let solution = minimizer.minimize(&sqr, vec![3.0, 4.0]);

    println!("Found solution for pi function at f({:?}) = {:?}",
             solution.position, solution.value);
}
