// Tuples group together values of different types
// Max 12 elements

fn sum_and_product(x:i32, y:i32) -> (i32, i32){
    (x+y, x*y)
}


fn tuples(){
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);

    println!("{:?}", sp);

    // destructuring
    let (a,b) = sp;
    println!("{}, {}", a, b);

    // tuple of tuples
    let sp2 = sum_and_product(3,4);
    let combined = (sp, sp2);

    // desructuring tuple of tuples
    println!("last elem = {}", (combined.1).1);

    // single element tuples
    let _tup = (48,);
}


pub fn run() {
    let person: (&str, &str, i8) = ("Haziq", "riyadh", 17);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
    tuples();
    
}

