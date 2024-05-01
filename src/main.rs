use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("please guess a number between 1 and 10");
    let winning_guess: i32 = rand::thread_rng().gen_range(1..10);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("something went wrong");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match guess.cmp(&winning_guess) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big 'that is what she said'"),
            Ordering::Equal => {
                println!("You won");
                break;
            }
        }
    }
}
