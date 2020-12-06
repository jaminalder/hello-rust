mod min_winit_example;
mod image_example;

use std::io::{stdin};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // min_winit_example::draw_window();
    image_example::image_example();
}

fn guessing() {
    println!("Guess the numbrrr!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("the secret number is: {}", secret_number);

    loop {
        println!("please input your guess.");

        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
