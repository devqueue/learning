// use std::mem;
// mod print;
// mod vars;
// mod types;
// mod strings;
// mod tuples;
// mod arrays;
// mod operators;
// mod vectors;
// mod scope_and_shadowing;


// GLOBAL VARIABLE DECLARATION
const MEANING_OF_LIFE:u8 = 42;  // No fixed adress

static mut MEANING_LIFE:u16 = 32; // issue with memory safety

fn main() {
    println!("Hello, world!");
    // print::run();
    // vars::run();
    // vars::variables();
    // types::run();
    // strings::run();
    // tuples::run();
    // vectors::run();
    // operators::operator();
    // scope_and_shadowing::scope_shadowing();

    println!("MEANING OF LIFE {}", MEANING_OF_LIFE);
    
    // My promise that everything shall work fine
    unsafe{
        MEANING_LIFE=77;
        println!("{}", MEANING_LIFE)
    }
}
