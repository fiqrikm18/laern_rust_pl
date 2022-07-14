use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // generate random number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("The secret number is {}", secret_number);
    
    loop {
        println!("Please input your guess.");

        // var to storing data from input user
        let mut guess = String::new();

        // get input from user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input");

        // check if input type is numeric
        let guess: u32 = match guess.trim().parse() {
            Ok(asd) => asd,
            Err(_) => {
                println!("Input must be a number!");
                break;
            },
        };

        println!("You guessed: {guess}");

        // matching the input with secret number that has been generated
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
