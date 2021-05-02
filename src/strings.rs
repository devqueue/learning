// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
    let hello = "Hello"; // primitive strings
    let mut hello_str = String::from("Hello ");

    // Get length
    println!("{}", hello);
    println!("length: {}", hello.len());

     // Can only push one character
    hello_str.push('W');

    // Can push more than one character
    hello_str.push_str("orld!");

    println!("{}", hello_str);

    // Capacity
    println!("Capacity: {}", hello_str.capacity());

    // Empty
    println!("Is Empty: {}", hello_str.is_empty());

    // Contains
    println!("contains 'world': {}", hello_str.contains("World"));

    //Replace
    println!("Replace: {}", hello_str.replace("World", "There"));

    // Loop through string
    for word in hello_str.split_whitespace(){
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('b');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}