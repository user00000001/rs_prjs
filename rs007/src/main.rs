use std::io;
use std::cmp::Ordering;
use std::io::Write;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng()
        .gen_range(1..101);
    println!("The secret number is: {}", secret_number);
    print!("Please input your guess: ");
    std::io::stdout()
        .flush().expect("stdout flush error.");
    loop {
        let mut guess = String::new();
        {
            io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        }
        println!("You guessed: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
               println!("You win!");
               break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}