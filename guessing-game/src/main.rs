use rand::{Rng, thread_rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut rng = thread_rng();

    loop {
        let secret_num: u32 = rng.gen_range(1..100);
        println!("Enter a number:");
        let mut input = String::new();
        assert!(io::stdin().read_line(&mut input).is_ok());

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match input.cmp(&secret_num) {
            Ordering::Less => println!("you guessed lesser!"),
            Ordering::Greater => println!("you guessed larger!"),
            Ordering::Equal => {
                println!("you guessed right!");
                break;
            }
        }
        println!("You guessed: {input} but the secret number was {secret_num}\n");
    }
}
