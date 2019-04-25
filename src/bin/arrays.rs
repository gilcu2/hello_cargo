use std::fmt::Debug;

fn print_slice<T: Debug>(slice: &[T]) {
    println!("{:?}", slice);
}

fn sum(slice: &[u8]) -> u8 {
    let mut sum = 0;
    for i in slice {
        sum += i
    }
    sum
}

fn main() {
    let array: [u8; 5] = [1, 2, 3, 4, 5];

    print!("Whole array just borrowed: ");
    print_slice(&array);

    print!("Whole array sliced: ");
    print_slice(&array[..]);

    print!("Without the first element: ");
    print_slice(&array[1..]);

    print!("One element from the middle: ");
    print_slice(&array[3..4]);

    print!("First three elements: ");
    print_slice(&array[..3]);

    println!("sum: {}", sum(&array))

//print!("Oops, going too far!: ");
//print_slice(&array[..900]);
}