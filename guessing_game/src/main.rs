use std::io;
extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
 println!("Guess the number!");
 println!("Please input your guess.");
 let secret_number = rand::thread_rng().gen_range(1, 101);

 loop {
    let mut guess = String::new();
 io::stdin().read_line(&mut guess)
 .expect("Failed to read line");
 let guess: u32 = guess.trim().parse()
 .expect("Please type a number!");
 
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