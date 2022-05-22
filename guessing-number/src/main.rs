use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..11);

    println!("Guess the number! 1 ~ 10");

    loop {
        println!("Please input your guess.");

        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line...");

        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {}", num);

        match num.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
        println!("-----");
    }
}