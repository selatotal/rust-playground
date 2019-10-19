use std::io;
use std::process::exit;

fn main() {

    println!("Inform temperature in Farenheit: ");
    let mut farenheit = String::new();
    io::stdin().read_line(&mut farenheit)
            .expect("Failed to read farenheit!");
    let farenheit:i32 = match farenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => exit(1),
        };
    let celsius:f32 = (farenheit - 32) as f32 * 5.0/9.0 as f32;
    println!("Temperature in Celsius: {}", celsius)
    
}
