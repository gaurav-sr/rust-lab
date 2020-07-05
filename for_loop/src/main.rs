fn main() {
    println!("Hello, Rust.. for loop!");
    simple_loop();
}

fn simple_loop() {
    let a = [2,3,4,5,7];
    for element in a.iter() {
        println!("Element is {}", element);
    }
}
