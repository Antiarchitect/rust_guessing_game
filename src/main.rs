use std::io;
use std::rand;
use std::cmp::Ordering;

fn cmp(a: uint, b: uint) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}

fn main() {
    println!("Guess the number!");

    let secret = (rand::random::<uint>() % 100u) + 1u;

    loop {
        println!("Please input your guess.");

        let input = io::stdin().read_line().ok().expect("Failed to read line.");
        let input_number = input.trim().parse::<uint>();
        let number = match input_number {
            Some(number) => number,
            None         => {
                println!("Please input a number!");
                return;
            }
        };

        println!("You guessed: {}", input);

        match cmp(number, secret) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                return;
            },
        }
    }
}
