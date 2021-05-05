#![allow(dead_code)]
use std::mem;

struct Point{
    x: f64,
    y: f64
}

fn origin() -> Point{
    Point{x: 0.0, y: 0.0}
}

pub fn sh(){
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("P1 takes {}", mem::size_of_val(&p1));
    println!("P2 takes {}", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("{}", p3.x);

}