use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //1..100 is exclusive of 100 and 1..=100 is inclusive to both bounds
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {}", secret_number);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line from console");

        //using match instead of expect lets us handle the error instead of crashing the program
        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter numeric value!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too small"),
            Ordering::Greater => println!("Your guess is too big"),
            Ordering::Equal => {
                println!("You guessed correct! Congratulations 😇");
                break;
            }
        }
    }
}
