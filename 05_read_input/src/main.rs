use std::io;

fn main() {
    println!("This program takes 2 numbers from user and prints sum of numbers.");
    
    println!("Enter first number : ");
    let first_number = read_from_console();

    //second number
    println!("Enter second number : ");
    let second_number = read_from_console();
        
    println!("First Number is {}", first_number);
    println!("Second Number is {}", second_number);

   // let sum = i8::new();
    let sum = first_number + second_number;
    println!("Sum is {}",sum);
}

fn read_from_console() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let number = input.trim().parse::<i32>().expect("Not a number");
    return number;
}
