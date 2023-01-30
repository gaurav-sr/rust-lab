fn main() {
    println!("Demonstrating if let...");
    iflet();
}

fn iflet() {    
    let marks = 97;
    let grade = if marks > 90 {
        "A+"
    } else if marks > 80 {
        "A"
    } else {
        "C"
    };
    println!("Grade is {}", grade);
}
