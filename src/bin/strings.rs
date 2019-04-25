fn print_string(s: &str) {
    println!("&str: {}", s)
}

fn print_second_word(s: &str) {
    let second_word = s.split(" ")
        .collect::<Vec<&str>>()[1];

    println!("second: {}", second_word)
}

fn main() {
    let str = "Hello, world &str!";
    let string = String::from("Hello, world String!");

    print_string(str);
    print_string(&string);
    print_second_word(str)
}
