use std::io;
use std::io::Read;
use std::fs::File;
use std::error::Error;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn main() -> Result<(), Box<dyn Error>> {
    let name = read_username_from_file().unwrap();

    println!("Name: {}", name);

    Ok(())
}
