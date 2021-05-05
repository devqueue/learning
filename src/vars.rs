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

pub fn variables(){
    // Rust pluralsight course
    let a:u8 = 123; // 8bits  unsigned (0-225)
    let b:i8 = 100; // 8 bits signed (-124)
    println!("{}", a);
    println!("{}", b);
    
    // int types: i8, u8, i16, u16, i32, u32, i64, u64
    
    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z= {}, takes up {} bytes, {}-bit os", z, size_of_z, size_of_z*8);

    let d='x'; //char
    println!("d = {}, size = {}, bytes", d, mem::size_of_val(&d));

    let e = 2.5; // double precision, 8 bytes or b4 bits, f32 or f64
    println!("e = {}, size of {} bytes", e, mem::size_of_val(&e));

    // Boolean
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g))
}