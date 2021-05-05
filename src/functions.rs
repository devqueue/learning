
// Functions - Used to store blocks of code for re-use
pub fn run(){
    greeting("hello", "Haziq");
    
    // Bind function values to variables
    let getsum = add(5,5);
    println!("Sum: {}", getsum);

    // closure
    let n3:i32 = 10;
    let add_nums = | n1:i32, n2:i32 | n1+n2+n3;  // main advantage: can use global vars like n3
    println!("Closure sum : {}", add_nums(3,3))

}


fn greeting(greet: &str, name: &str){
    println!("{}, nice to meet you {}", greet, name )
}


fn add(n1: i32, n2: i32) -> i32 {
    n1+n2 // No semi colon means this is what we will return 
}