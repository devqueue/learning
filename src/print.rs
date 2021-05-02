pub fn run(){
    // Print to console
    println!("Hello from the print.rs");

    // Basic formatting
    // println!(1); NOT VALID
    println!("Number: {}", 1); //valid
    println!("{} am a {} programmer", "haziq", "rust");

    // Positional Arguments
    println!("{0} is a {1} programmer and {0} is learning {2}", "Haziq", "python","rust");

    // Named arguments
    println!("{name} likes to play {activity}",name = "John", activity= "baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic math
    println!("10 + 10 = {}", 10+10);
}