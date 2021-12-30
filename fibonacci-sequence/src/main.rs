use std::io::{self, Write};

fn main() {
    print!("Enter the Number: ");
    io::stdout().flush().unwrap();

    let mut nth_number = String::new();

    io::stdin()
        .read_line(&mut nth_number)
        .expect("Failed to read number");
    let nth_number: u16 = nth_number
        .trim()
        .parse()
        .expect("Failed to parse Nth Number");
    let mut previous_number: u16 = 0;
    let mut current_number: u16 = 1;
    let mut count: u16 = 0;
    println!("=========================================");
    print!("\t{}, ", 0);
    count += 1;

    while count != nth_number {
        let result = current_number + previous_number;
        print!("{}, ", result);
        count += 1;
        previous_number = current_number;
        current_number = result;
    }
    println!("");
    println!("=========================================");
}
