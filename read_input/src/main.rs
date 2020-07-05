use std::io;

fn main() {
    println!("This program takes 2 numbers from user and prints sum of numbers.");
    let mut first_number = String::new();
    println!("Enter first number : ");
    
    io::stdin().read_line(&mut first_number)
    .expect("Failed to read input");

    //second number
    let mut second_number = String::new();
    println!("Enter second number : ");
    
    io::stdin().read_line(&mut second_number)
    .expect("Failed to read input");
        
    println!("First Number is {}", first_number);
    println!("Second Number is {}", second_number);

   // let sum = i8::new();
    let f_num: i32 = first_number.trim().parse::<i32>().expect("Not a number");
    let s_num: i32 = second_number.trim().parse::<i32>().expect("Not a number");
    let sum: i32 = f_num + s_num;
    println!("Sum is {}",sum);
}
