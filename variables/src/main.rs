fn main() {
    println!("Hello, Rust Variables!");
    let name = "Gaurav";
    println!("Name is {}", name);
    println!("Trying to change name...");
    // name = "Dublin"; This will result in compiler error
    let mut age = 23;
    println!("Age is {}", age);
    println!("Changing age to 24");
    age = 24;
    if age == 24 {
        println!("Age changed successfully");
    } else {
        println!("Age could not be changed");
    }
    declare_constants();
}

fn declare_constants() {
    const YEAR: u16 = 2019;
    println!("Year is {}", YEAR);
}
