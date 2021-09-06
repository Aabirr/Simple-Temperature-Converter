use std::io;

fn main() {
    
    println!("Press 1 to convert in Farhenheit: ");
    println!("Press 2 to convert in Celcius");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to readline.");

    let choice: f64 = choice.trim().parse().expect("Failed to readline");

    println!("Enter your Temperature: ");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to readline");

    let temp: f64 = temp.trim().parse().expect("Failed to readline");
    
    if choice == 1.0 {
        println!("Temperature in Fahrenheit is {} degree", celsius(temp));
    } else if choice == 2.0 {
        println!("Temperature in celcius is {} degree", fahrenheit(temp));
    }
    
}

fn celsius(temp: f64) -> f64{
    (temp * 9.0/5.0) + 32.0
}
fn fahrenheit(temp: f64) ->f64 {
    (temp - 32.0) * 5.0/9.0
}