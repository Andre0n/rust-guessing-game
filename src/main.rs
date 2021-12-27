use std::io;
use rand::Rng;

fn main() {
    let stdin = io::stdin();
    let mut rng = rand::thread_rng();

    println!("Guessing Game - Try to find the secret number.");
    let secret_number:u32 = rng.gen_range(1..=100);

    println!("Please input your guess: ");
    let mut user_input = String::new();
    stdin.read_line(&mut user_input).expect("Erro while reading `input`");
    println!("Good job, you typed {}", user_input);

    println!("The secret number is {}", secret_number);
}