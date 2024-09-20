use std::io;
fn main() {
    println!("Enter guess number: ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failled to read line");
    let a:i32 = 5;

    println!("You guessed: {guess}, {}", a);


    // generate secret number

}
