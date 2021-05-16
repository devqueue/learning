#![allow(dead_code)]

fn how_many(x:i32) -> &'static str{
    match x{
        0 => "No",
        1 | 2 => "One or 2",
        12 => "A dozen",
        _z @ 9..=11 => "lots", //name for the range _z
        _ if (x%2==0) => "some",
        _ => "a hell lot"
    }
}


pub fn run(){
    for x in 0..13{
        println!("{}: i have {} oranges", x, how_many(x));
    }
    
    let point = (3,4);

    match point{
        (0,0) => println!("origin"),
        (0, y) => println!("x = {}, y = {}", 0, y),
        (x, 0) => println!("x = {}, y = {}", x, 0),
        (x,y) => println!("x ={}, y = {}", x, y)
    }
}