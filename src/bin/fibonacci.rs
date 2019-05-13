use std::io;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        x if x > 0 => fibonacci(x - 1) + fibonacci(x - 2),
        _ => panic!("fibonacci doesn't defined for negative numbers")
    }
}

fn main() {
    println!("Type a natural number");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read the number");

    let n = n.trim();

    let n: u32 = n.parse().expect("number must be a natural number");

    let fib = fibonacci(n);

    println!("Fibonacci({})={}", n, fib);
}