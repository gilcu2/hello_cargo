fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1="perro";
    let s2="gato";
    println!("The longest is {}",longest(s1,s2))
}