use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("input guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed");

    println!("guess : {}", guess);

    match guess.cmp(&secret_number ) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("WInd"),
    }
}
