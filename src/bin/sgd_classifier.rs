use rustlearn::prelude::*;
use rustlearn::linear_models::sgdclassifier::Hyperparameters;
use rustlearn::datasets::iris;

fn main() {
    let (x, y) = iris::load_data();

    let mut model = Hyperparameters::new(4)
        .learning_rate(1.0)
        .l2_penalty(0.5)
        .l1_penalty(0.0)
        .one_vs_rest();

    model.fit(&x, &y).unwrap();

    let prediction = model.predict(&x).unwrap();

//    println!("Model: {:?}", model);
    println!("Prediction: {:?}", prediction);
}