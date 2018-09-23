extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("Guess the number!");
        //let secret_number = rand::thread_rng().gen_range(1,101).to_string();
        let secret_number = rand::thread_rng().gen_range(1, 101);
        println!("the secret number is:{}", secret_number);
        let mut guess = String::new();
        println!("please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed : {}", guess);
        //let guess: i32 = guess.trim().parse().expect("please input a number");
        let guess:i32 = match guess.trim().parse()
        {
            Ok(num)=> num,
            Err(_) => {
                println!("please input a number");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Greater"),
            Ordering::Less => println!("too small"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
        }
    }
}
