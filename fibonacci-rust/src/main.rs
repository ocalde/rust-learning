use std::io;
use std::io::Write;

fn main() {
    println!(":::::::::Fibonacci sequence:::::::::");
    let mut number_limit = String::new();
    
    println!("Enter the number until you want to calculate sequence: ");
    //io::stdout().flush().unwrap();
    io::stdin().read_line(&mut number_limit) //expected mutable reference, found struct `std::string::String` help: consider mutably borrowing here: `&mut number_limit`
        .expect("Enter something");

    let number_limit = loop {
        println!("Current value is {}", number_limit);
        match number_limit.trim().parse::<i64>() {
            Ok(num) => { break num; },
            Err(_) => {
                println!("This is not a number, fix it");
                io::stdin().read_line(&mut number_limit)
                .expect("Enter something");
                io::stdout().flush().unwrap();
            }
        };
    };

    let mut current_number: i64 = 1;
    let mut previous_number: i64 = 1;

    while current_number <= number_limit {
        print!("{} ", current_number);
        current_number += previous_number;
        previous_number = current_number - previous_number;
    }

    io::stdout().flush().unwrap();
}
