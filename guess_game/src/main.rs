use std::io;
use rand::Rng;

fn main() {
    println!("This programs takes number and compares with random number.");

    let mut guess = String::new();
    println!("Guess the number between 1 to 100");
    io::stdin().read_line(&mut guess).expect("Failed to read input");
    println!("You guessed {}", guess);

    //generate the random number
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number is {}", secret_number);
}
