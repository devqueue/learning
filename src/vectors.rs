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

    println!("Numbers Vec: {:?}", numbers_mut);

    // pluralsight code
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("{:?}", a);

    // index types

    /* i32 will not work because indexies cannot be signed values and 
    cannot be 32 bit on a 64 bit machine hence usize must be used 
    / let index:i32 = 0; 
    */
    let index:usize = 0;
    println!("a[0] = {}", a[index]);

    /* in a vector you cannot know how many elements exist at compile 
    time hence if an element out of bounds is acessed the program willl
    crash
    To prevent this we can use a vector.get() method which returns an 
    option type
    */

    // option
    match a.get(6){
        Some(x) => println!("a[6] = {}", x),
        None => println!("No such element found")
    }

    // iteration over vectors

    for i in &a{
        println!("{}", i);
    }

    // removing elements
    let lst = a.pop(); // returns an option
    println!("{:?}", lst);

    // let Some(last_elem) = a.pop();  // not valid because a.pop might return a none hence it cannot be assigned to a Some(x)

    while let Some(x) = a.pop(){
        println!("{}", x);
    }
}
