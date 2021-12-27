use std::io;
fn main(){
    println!("Guessing Game - Try to find the secret number.");
    println!("Please input your guess: ");
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).expect("Erro while reading `input`");
    println!("Good job, you typed {}", user_input);
}