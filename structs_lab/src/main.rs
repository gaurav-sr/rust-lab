/**
 * Define a custom struct
 */
struct User {
    name: String,
    age: u32,
    city: String
}
fn main() {
    println!("Hello, world!");
    let my_user: User = User {
        name: String::from("Gaurav"),
        age: 35,
        city: String::from("Pune")
    };
    println!("Name is {}", my_user.name);
    println!("Age is {}", my_user.age);
    println!("City is {}", my_user.city);
    let second_user: User = build_user(String::from("Gaurav"), 40);
    print!("Second user name {}", second_user.name);
}
 
/**
 * Function to build user
 */
fn build_user(name: String, age: u32) -> User {
    User {
        name,
        age,
        city: String::from("Dublin")
    }
}


