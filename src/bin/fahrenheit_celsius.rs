use std::io;

enum Unit {
    Celsius,
    Fahrenheit,
}

fn main() {
    println!("Type the temperature with the unit");

    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature).expect("Failed to read temperature");

    let temperature = temperature.trim();

    let value = &temperature[0..temperature.len() - 1];
    let unit = &temperature[temperature.len() - 1..];

    println!("value: '{}' unit: '{}'", value, unit);

    let value: i32 = value.parse().expect("temperature must be a number");

    let unit = match unit {
        "C" => Unit::Celsius,
        "F" => Unit::Fahrenheit,
        _ => panic!("unit must be C or F"),
    };

    match unit {
        Unit::Celsius => println!("{}{}", value * 9 / 5 + 32, "F"),
        Unit::Fahrenheit => println!("{}{}", (value - 32) * 5 / 9, "C"),
    }
}