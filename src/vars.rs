// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Haziq";
    let age = 17;
    // age = 18  Not valid since its immutable

    let mut age_mut = 17;
    println!("My name is {} and I am {}", name, age_mut);

    age_mut = 18;
    println!("My name is {} and I am {}", name, age_mut);

    // define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_age, my_name) = ("haziq", 17);
    println!("{} is {}", my_name, my_age);
}