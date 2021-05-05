// Vectors - Resizable Vectorss

use std::mem;

pub fn run(){
    // mutable Vectors
    let mut numbers_mut: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers_mut);

    // re-assign value
    numbers_mut[2] = 20;

    // Add on to vector
    numbers_mut.push(5);
    numbers_mut.push(6);

    // pop off last element
    numbers_mut.pop();
    
    println!("{:?}", numbers_mut);

    // get Vectors length
    println!("Vectors length: {}", numbers_mut.len());

    // Vectors are stack allocated
    println!("Vectors occupies {} bytes", mem::size_of_val(&numbers_mut));

    // get slice
    let slice: &[i32] = &numbers_mut[0..2];
    println!("slice: {:?}", slice);

    // loop through vector values
    for x in numbers_mut.iter(){
        println!("Numbers: {}", x);
    }

    // loop and mutate values
    for x in numbers_mut.iter_mut(){
        *x *=2;
    }

    println!("Numbers Vec: {:?}", numbers_mut)

}
