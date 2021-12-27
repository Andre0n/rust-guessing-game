use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let stdin = io::stdin();
    let mut rng = rand::thread_rng();

    println!("Guessing Game - Try to find the secret number.");
    let secret_number:u32 = rng.gen_range(1..=100);

    loop {
        println!("Please input your guess: ");
        let mut user_input = String::new();
        stdin.read_line(&mut user_input).expect("Erro while reading `input`");
        let user_input:u32 = match user_input.trim().parse::<u32>() {
            Ok(result) => result,
            Err(..) => {
                println!("Nice try, but enter a valid number.");
                continue;
            },
        };
        match user_input.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again!"),
            Ordering::Greater => println!("Too big! Try again!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
