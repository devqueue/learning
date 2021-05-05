// pluralsight code

pub fn operator(){
    // arithmetic
    let mut a = 2+3*4;
    println!("{}", a);
    a += 1;
    println!("{}", a);

    // power
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3); // integral cube
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^ pi  = {}", b , b_cubed, b , b_to_pi);

    // bitwise 
    // | - or, &- and, ^ XOR, ! NOR
    let c = 1 | 2;
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);
    println!("{}", c);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("{}", pi_less_4)

}