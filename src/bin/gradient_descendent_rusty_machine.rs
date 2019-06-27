extern crate rusty_machine as rm;

use rm::learning::optim::Optimizable;
use rm::linalg::Matrix;
use rm::learning::optim::grad_desc::GradientDesc;
use rm::learning::optim::OptimAlgorithm;

struct XSqModel {
    c: f64,
}

impl Optimizable for XSqModel {
    type Inputs = Matrix<f64>;
    type Targets = Matrix<f64>;

    fn compute_grad(&self, params: &[f64], _: &Matrix<f64>, _: &Matrix<f64>) -> (f64, Vec<f64>) {
        ((params[0] - self.c) * (params[0] - self.c),
         vec![2f64 * (params[0] - self.c)])
    }
}

fn main() {
    let x_sq = XSqModel { c: 20f64 };

    let gd = GradientDesc::default();
    let test_data = vec![500f64];
    let params = gd.optimize(&x_sq,
                             &test_data[..],
                             &Matrix::zeros(1, 1),
                             &Matrix::zeros(1, 1));

    assert!(params[0] - 20f64 < 1e-10);
    assert!(x_sq.compute_grad(&params, &Matrix::zeros(1, 1), &Matrix::zeros(1, 1)).0 < 1e-10);

    println!("params[0]: {:?})", params[0]);
}
