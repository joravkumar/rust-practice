// use std::io;
use std::io::{self, Write};

fn main() {
    loop {
        println!("1. Convert the temperature Fahrenheit to Celsius");
        println!("2. Convert the temperature Celsius to Fahrenheit");
        print!("Enter the Option: ");
        io::stdout().flush().unwrap();
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read temperature");
        let option: u32 = option.trim().parse().expect("Failed to parse temperature");
        if option == 0 || option > 2 {
            println!("=========================================");
            println!("\tYou chose wrong option!!");
            println!("=========================================");
            continue;
        }
        println!("You chose option {}", option);
        let mut temp = String::new();
        print!("Enter the temperature: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read temperature");
        let temp: f32 = temp.trim().parse().expect("Failed to parse temperature");
        println!("=========================================");
        if option == 1 {
            println!("Temperature in Fahrenheit is {}", temp);
            let changed_temp: f32 = (temp - 32.0) * 5.0 / 9.0;
            println!("Temperature in Celsius is {:.2}", changed_temp);
        }
        if option == 2 {
            println!("Temperature in Celsius is {}", temp);
            let changed_temp = (temp * 9.0 / 5.0) + 32.0;
            println!("Temperature in fahrenheit is {:.2}", changed_temp);
        }
        println!("=========================================");
        print!("Do you want to continue [y|n]?");
        io::stdout().flush().unwrap();
        let mut is_continue = String::new();
        io::stdin()
            .read_line(&mut is_continue)
            .expect("Failed to read is contniue");
        let is_continue: Vec<_> = is_continue.trim().chars().collect();
        if is_continue[0] == 'y' {
            println!("You want to continue");
            continue;
        } else {
            break;
        }
    }
}
