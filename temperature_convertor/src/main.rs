use std::io;

fn main() {
    println!("Enter the temperature unit (F or C):");
    let mut unit = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    let unit: char = match unit.trim().parse::<char>() {
        Ok(value) => value.to_ascii_uppercase(),
        Err(_) => {
            println!("Invalid unit");
            return;
        },
    };

    println!("Enter the temperature:");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature");
            return;
        },
    };

    match unit {
        'F' => {
            let celsius = (temperature - 32.0) * 5.0 / 9.0;
            println!("The temperature in Fahrenheit is: {temperature} °F");
            println!("The temperature in Celsius is: {celsius} °C");
        }
        'C' => {
            let fahrenheit = (temperature * 9.0 / 5.0) + 32.0;
            println!("The temperature in Celsius is: {temperature} °C");
            println!("The temperature in Fahrenheit is: {fahrenheit} °F");
        }
        _ => println!("Invalid unit"),
    }
}
