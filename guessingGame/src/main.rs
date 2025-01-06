use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Enter guess number: ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failled to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");
    //println!("You guessed: {}", guess);

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number: {secret_number}");


    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }
}
