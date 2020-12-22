mod min_winit_example;
mod image_example;
mod minigrep;
mod my_image;

use std::io::{stdin};
use rand::Rng;
use std::cmp::Ordering;
use std::f64::consts::PI;

fn main() {
    //min_winit_example::draw_window();
    my_image::draw_window();
    //step_test();
    //color_test();
    // image_example::image_example();
    // minigrep::minigrep();
}

fn step_test() {
    let r = (0..(PI*2.0*100.0) as i32);
    let v = r.map(|n| n as f64 * 0.01).collect::<Vec<f64>>();
    println!("{:?}", v);
}

fn color_test() {
    let mut col: [u8;4] = [0xff,6,6,6];
    println!("{}", u8::MIN);
    println!("{}", u8::MAX);
    println!("{:?}", col);
    println!("{:?}", rand_color());
    println!("{:?}", rand_color());
    println!("{:?}", rand_color());
    println!("{:?}", rand_color());
}

fn rand_color() -> [u8; 4] {
    let r = rand::thread_rng().gen_range(0, 255) as u8;
    let g = rand::thread_rng().gen_range(0, 255) as u8;
    let b = rand::thread_rng().gen_range(0, 255) as u8;
    [r, g, b, 255]
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
