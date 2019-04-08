use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("::::Guess the number::::");
    let machine_guess = rand::thread_rng().gen_range(1, 100);
    let mut number_of_tries: u8 = 0;
    let mut number_of_invalid_inputs: u8 = 0;
    let max_invalid_inputs: u8 = 5;

    loop {
        println!("Input your guess: ");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess = match guess.trim().parse::<i8>() {
            Ok(num) => num,
            Err(_) => {
                
                number_of_invalid_inputs += 1;
                /*match number_of_invalid_inputs.cmp(&max_invalid_inputs) {
                    Ordering::Equal => {
                        println!("You don't want to play, get away!!");
                        break;
                    },
                    Ordering::Less | Ordering::Greater => println!("Don't be a piece of cake, enter a valid number please!"),
                }*/

                if number_of_invalid_inputs == max_invalid_inputs {
                    println!("You don't want to play, get away!!");
                    break;
                } else {
                    println!("Don't be a piece of cake, enter a valid number please!");
                }
                continue;
            },
        };
            //    .expect("Please type a number little shitty");

        match guess.cmp(&machine_guess) {
            Ordering::Equal => {
                println!("You guessed!! {}", guess);
                println!("It took you {} attempts to guess it", number_of_tries);
                break;
            },
            Ordering::Less => println!("Too low!!"),
            Ordering::Greater => println!("Too high!! "),
            
        }    
        number_of_tries += 1;
    }
}
