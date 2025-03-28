use rand::{Rng, thread_rng};
use std::cmp::Ordering;
use std::io;

const NUM_ATTEMPTS: i32 = 5;

fn main() {
    println!("Guess the number!");

    let mut rng = thread_rng();
    let tup : (u32, f32) = (1, 2.15);
    let arr: [u32;5] = [2;5];
    let mut i = 0;

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
	if i == NUM_ATTEMPTS {
	   println!("{} attempts done! quitting\n", NUM_ATTEMPTS);
	   break;
	}
        println!("You guessed: {input} but the secret number was {secret_num}\n");
	i = i+1;
    }

    println!("{0}\n", tup.1);
    println!("{0}\n", arr[2]);
}
