#![allow(dead_code)]

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
// mod stack_and_heap;
// mod conditionals;
// mod loops;
// mod functions;
// mod pointer_ref;
// mod structs;
// mod enums;
// mod cli;
// mod match_case;
// mod optiont;
// mod slices;
// mod pattern_matching;
// mod generics;
// mod method;
// mod closures;
// mod higher_order_fn;
// mod traits;
mod ownership;


// GLOBAL VARIABLE DECLARATION
const MEANING_OF_LIFE:u8 = 42;  // No fixed adress
static mut MEANING_LIFE:u16 = 32; // issue with memory safety because varibale is gloabl and mutable

fn main() {
    println!("HELLO, WORLD!");
    // operators::operator();
    // scope_and_shadowing::scope_shadowing();
    // stack_and_heap::sh();
    // print::run();
    // vars::run();
    // vars::variables();
    // types::run();
    // strings::run();
    // tuples::run();
    // vectors::run();
    // arrays::run();
    // conditionals::run();
    // loops::run();
    // functions::run();
    // pointer_ref::run();
    // structs::run();
    // enums::run();
    // cli::run();
    // match_case::run(7);
    // optiont::run()
    // slices::run();
    // pattern_matching::run();
    // generics::run();
    // method::run();
    // closures::run();
    // higher_order_fn::run();
    // traits::run();
    ownership::run();
    

    println!("MEANING OF LIFE {}", MEANING_OF_LIFE);
    // My promise that everything shall work fine
    unsafe{
        MEANING_LIFE=77;
        println!("{}", MEANING_LIFE)
    }


}
