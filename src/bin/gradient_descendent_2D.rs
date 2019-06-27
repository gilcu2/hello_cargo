extern crate rusty_machine as rm;

use rm::learning::optim::Optimizable;
use rm::linalg::Matrix;
use rm::learning::optim::grad_desc::GradientDesc;
use rm::learning::optim::OptimAlgorithm;

// f(x,y)=((x,y)-(cx,cy))^2
struct XSqModel2D {
    c: (f64, f64),
}

impl Optimizable for XSqModel2D {
    type Inputs = Matrix<(f64, f64)>;
    type Targets = Matrix<f64>;

    fn compute_grad(&self, params: &[(f64, f64)], _: &Matrix<f64>, _: &Matrix<f64>) -> (f64, Vec<f64>) {
        let value = (params[0] - self.c) * (params[0] - self.c);
        let first_derivate = vec![2.0 * (params[0] - self.c)];
        (value, first_derivate)
    }
}

fn main() {
    let x_sq = XSqModel2D { c: (10.0, 10.0) };

    let gd = GradientDesc::default();
    let test_data = vec![500f64];
    let params = gd.optimize(&x_sq,
                             &test_data[..],
                             &Matrix::zeros(1, 1),
                             &Matrix::zeros(1, 1));

    println!("params[0]: {:?})", params[0]);
}
