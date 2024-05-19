use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut count : i32 = 0;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                count = count + 1;
            },
            Ordering::Greater => {
                println!("Too big!");
                count = count + 1;
            },
            Ordering::Equal => {
                println!("You win!");
                println!("You took {} chances to guess the number.", count);
                break;
            }
        }
    }
}